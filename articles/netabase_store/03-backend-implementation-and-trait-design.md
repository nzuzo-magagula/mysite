#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Designing one API that works across Sled, Redb and IndexedDB, and what the trait abstraction actually costs at runtime"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/02-procedural-macros-and-code-generation"
next = "netabase_store/04-configuration-api-and-transaction-system"

[[references]]
title = "Rust Traits - The Rust Book"
url = "https://doc.rust-lang.org/book/ch10-02-traits.html"
description = "Official Rust documentation on traits and trait-based abstraction"

[[references]]
title = "Associated Types - The Rust Book"
url = "https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types"
description = "Guide to using associated types in trait definitions"

[[references]]
title = "Generic Parameters - Rust By Example"
url = "https://doc.rust-lang.org/rust-by-example/generics.html"
description = "Examples and explanations of generic type parameters in Rust"

[[references]]
title = "Enum Types - The Rust Book"
url = "https://doc.rust-lang.org/book/ch06-00-enums.html"
description = "Complete guide to Rust enums and pattern matching"
#####
# Backend Implementation and Trait Design

The [previous article](./02-procedural-macros-and-code-generation.md) covered how the procedural macros generate type-safe code. This one is about how trait-based design gets you actual backend portability, so the same application code runs against Sled, Redb or IndexedDB.

I'll cover designing abstractions that hide backend differences, implementing `NetabaseTreeSync` for Sled, managing lifetimes for safe resource access, handling secondary key indexing, and the native versus WASM split.

## Why Traits Work Here

The useful observation is that despite their differences, key-value databases offer more or less the same operations. Define a [trait][1] capturing those and you can write code that works with any of them:

```rust
pub trait NetabaseTreeSync<'db, D, M> {
    type PrimaryKey;
    type SecondaryKeys;

    fn put(&self, model: M) -> Result<(), NetabaseError>;
    fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;

    fn is_empty(&self) -> Result<bool, NetabaseError>;
    fn len(&self) -> Result<usize, NetabaseError>;
    fn clear(&self) -> Result<(), NetabaseError>;
}
```

A few decisions worth explaining.

I used [associated types][2] rather than generic parameters:

```rust
// Why this:
pub trait NetabaseTreeSync<'db, D, M> {
    type PrimaryKey;
    type SecondaryKeys;
}

// Instead of this:
pub trait NetabaseTreeSync<'db, D, M, PK, SK> {
    fn get(&self, key: PK) -> Result<Option<M>, NetabaseError>;
}
```

Each model has exactly one primary key type and one secondary keys enum, so those belong to the implementation rather than being chosen at each call site. It keeps the API cleaner.

`put` takes the model by value:

```rust
fn put(&self, model: M) -> Result<(), NetabaseError>;
```

Models are typically cloned from user code anyway, extracting keys needs owned values, and consuming makes the ownership semantics obvious.

And secondary key lookups return a `Vec`:

```rust
fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
    -> Result<Vec<M>, NetabaseError>;
```

Primary keys are unique. Secondary keys aren't, so multiple models can share a value, and the return type should say so.

## Implementing SledStore

[Sled][5] is a high-performance embedded database. The store itself is thin:

```rust
pub struct SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    pub(crate) db: sled::Db,
    pub trees: Vec<D::Discriminant>,
}
```

It holds a sled database instance and a list of known tree discriminants for iteration.

Each model gets its own tree, which is sled's term for a namespace within the database:

```rust
pub struct SledStoreTree<'db, D, M>
where
    D: NetabaseDefinitionTrait,
    M: NetabaseModelTrait<D>,
{
    pub(crate) tree: sled::Tree,           // Primary key → model
    pub(crate) secondary_tree: sled::Tree,  // Secondary key → primary key
    pub db: sled::Db,                       // Reference to parent DB
    pub(crate) _phantom_d: PhantomData<D>,
    pub(crate) _phantom_m: PhantomData<M>,
    pub(crate) _phantom_db: PhantomData<&'db ()>,
}
```

Two sled trees per model: one for primary storage, one for secondary indexes. The [phantom types][6] give type safety at no runtime cost, and the `'db` [lifetime][7] ties the tree to its parent store.

```rust
impl<D> SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    pub fn open_tree<M>(&self) -> SledStoreTree<'_, D, M>
    where
        M: NetabaseModelTrait<D>,
    {
        let tree_name = M::discriminant_name();
        SledStoreTree::new(&self.db, tree_name)
    }
}

impl<'db, D, M> SledStoreTree<'db, D, M> {
    pub(crate) fn new(db: &sled::Db, tree_name: &str) -> Self {
        let tree = db.open_tree(tree_name)
            .expect("Failed to open tree");

        let sec_tree_name = format!("{}_secondary", tree_name);
        let secondary_tree = db.open_tree(sec_tree_name)
            .expect("Failed to open secondary tree");

        Self {
            tree,
            secondary_tree,
            db: db.clone(),
            _phantom_d: PhantomData,
            _phantom_m: PhantomData,
            _phantom_db: PhantomData,
        }
    }
}
```

`open_tree` uses the model's discriminant, so "User" and "Post" each get isolated storage.

## The Write Path

`put` is the most complex operation, because it has to keep the primary and secondary indexes consistent:

```rust
impl<'db, D, M> NetabaseTreeSync<'db, D, M> for SledStoreTree<'db, D, M>
where
    D: NetabaseDefinitionTrait + From<M>,
    M: NetabaseModelTrait<D> + Clone,
{
    type PrimaryKey = M::PrimaryKey;
    type SecondaryKeys = M::SecondaryKeys;

    fn put(&self, model: M) -> Result<(), NetabaseError> {
        // Step 1: Extract keys from model
        let primary_key = model.primary_key();
        let secondary_keys = model.secondary_keys();

        // Step 2: Serialize keys and model
        let pk_bytes = bincode::encode_to_vec(&primary_key, bincode::config::standard())?;
        let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;

        // Step 3: Check if model already exists (for secondary key cleanup)
        let old_model: Option<M> = self.tree.get(&pk_bytes)?
            .map(|bytes| {
                let (m, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())?;
                Ok::<M, NetabaseError>(m)
            })
            .transpose()?;

        // Step 4: Use a batch to ensure atomicity
        let mut batch = sled::Batch::default();

        // Insert primary record
        batch.insert(pk_bytes.clone(), model_bytes);

        // Step 5: Remove old secondary indexes
        if let Some(old) = old_model {
            for old_sk in old.secondary_keys() {
                let sk_bytes = bincode::encode_to_vec(&old_sk, bincode::config::standard())?;
                batch.remove(sk_bytes);
            }
        }

        // Step 6: Insert new secondary indexes
        for sk in secondary_keys {
            let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
            batch.insert(sk_bytes, pk_bytes.clone());
        }

        // Step 7: Apply batch atomically
        self.tree.apply_batch(batch)?;

        Ok(())
    }
}
```

The complexity buys three things. The batch makes it atomic, so primary and secondary changes land together or not at all. Updating an existing model removes the stale [secondary indexes][9] rather than leaving them pointing nowhere. And that means indexes always point at valid primary keys.

For a `User` model, the layout ends up as:

```
Primary Tree ("User"):
  [bincode(UserPrimaryKey(1))] → [bincode(User { id: 1, email: "alice@example.com", ... })]

Secondary Tree ("User_secondary"):
  [bincode(UserSecondaryKeys::Email("alice@example.com"))] → [bincode(UserPrimaryKey(1))]
```

## The Read Path

Reading by primary key is much simpler:

```rust
fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError> {
    // Serialize the key
    let key_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Look up in primary tree
    let value_bytes = self.tree.get(key_bytes)?;

    // Deserialize if found
    match value_bytes {
        Some(bytes) => {
            let (model, _) = bincode::decode_from_slice(
                &bytes,
                bincode::config::standard()
            )?;
            Ok(Some(model))
        }
        None => Ok(None),
    }
}
```

Sled gives you O(log n) lookups via a [B-tree][10], deserialization cost scales with model size, and there are no allocations beyond the model itself.

## Secondary Key Queries

Querying by secondary key takes two lookups:

```rust
fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
    -> Result<Vec<M>, NetabaseError>
{
    let mut results = Vec::new();

    // Step 1: Serialize secondary key
    let sk_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Step 2: Scan secondary tree for matches
    for item in self.secondary_tree.scan_prefix(&sk_bytes) {
        let (_, pk_bytes) = item?;

        // Step 3: Look up model by primary key
        if let Some(model_bytes) = self.tree.get(pk_bytes)? {
            let (model, _) = bincode::decode_from_slice(
                &model_bytes,
                bincode::config::standard()
            )?;
            results.push(model);
        }
    }

    Ok(results)
}
```

`scan_prefix` works because serializing the whole secondary key enum means every record with the same key shares a prefix:

```
[bincode(UserSecondaryKeys::Email("alice@example.com"))] = prefix for all alice@ records
[bincode(UserSecondaryKeys::Age(30))] = prefix for all age 30 records
```

This was a genuinely cool thing to learn about how Rust enums serialize, and a good example of why it's worth understanding what the language does underneath.

## Removal

Removal also has to clean up the secondary indexes:

```rust
fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError> {
    let key_bytes = bincode::encode_to_vec(&key, bincode::config::standard())?;

    // Step 1: Get the model (if it exists)
    let model = match self.tree.get(&key_bytes)? {
        Some(bytes) => {
            let (m, _) = bincode::decode_from_slice(&bytes, bincode::config::standard())?;
            Some(m)
        }
        None => return Ok(None),
    };

    let model = model.unwrap();

    // Step 2: Create batch for atomic deletion
    let mut batch = sled::Batch::default();

    // Remove primary record
    batch.remove(key_bytes);

    // Step 3: Remove all secondary indexes
    for sk in model.secondary_keys() {
        let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
        batch.remove(sk_bytes);
    }

    // Step 4: Apply batch
    self.tree.apply_batch(batch)?;

    Ok(Some(model))
}
```

Returning the deleted model lets callers use its data one last time.

## Async Traits for WASM

[IndexedDB][12] has an asynchronous API, so there's a parallel async trait:

```rust
#[cfg(feature = "wasm")]
#[async_trait(?Send)]
pub trait NetabaseTreeAsync<D, M> {
    type PrimaryKey;
    type SecondaryKeys;

    async fn put(&self, model: M) -> Result<(), NetabaseError>;
    async fn get(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    async fn remove(&self, key: Self::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    async fn get_by_secondary_key(&self, key: Self::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;

    async fn is_empty(&self) -> Result<bool, NetabaseError>;
    async fn len(&self) -> Result<usize, NetabaseError>;
    async fn clear(&self) -> Result<(), NetabaseError>;
}
```

`?Send` because JavaScript is single-threaded and the futures don't need to be `Send`.

## Backend-Agnostic Code

With the traits in place you can write code that doesn't know or care which backend it's running on:

```rust
// This function works with SledStore, RedbStore, or any future backend
fn count_users<'a, T>(tree: &T) -> Result<usize, NetabaseError>
where
    T: NetabaseTreeSync<'a, BlogDefinition, User>
{
    tree.len()
}

// Use with Sled
let sled_store = SledStore::<BlogDefinition>::temp()?;
let sled_tree = sled_store.open_tree::<User>();
let sled_count = count_users(&sled_tree)?;

// Use with Redb (same function!)
let redb_store = RedbStore::<BlogDefinition>::temp()?;
let redb_tree = redb_store.open_tree::<User>();
let redb_count = count_users(&redb_tree)?;
```

## Lifetimes

The `'db` lifetime is what keeps this safe:

```rust
pub struct SledStoreTree<'db, D, M> {
    // ...
    pub(crate) _phantom_db: PhantomData<&'db ()>,
}
```

It ties the tree's lifetime to the store's:

```rust
// OK: tree outlived by store
{
    let store = SledStore::<BlogDef>::temp()?;
    let tree = store.open_tree::<User>();
    // Use tree...
}  // Both drop together

// Compile error: tree would outlive store
let tree = {
    let store = SledStore::<BlogDef>::temp()?;
    store.open_tree::<User>()
};  // Error: `store` dropped while borrowed
```

You can't use a tree after its parent store is gone, and the compiler enforces that rather than you having to remember it.

## Does the Abstraction Cost Anything?

Worth asking. Comparing a direct sled call:

```rust
// Direct sled call
let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;
tree.insert(key, model_bytes)?;

// Through NetabaseTreeSync trait
tree.put(model)?;
```

The trait methods are [monomorphized][15] at compile time and produce identical machine code to the handwritten version, so the answer is no.

## What Changes for Redb

[Redb][16] is similar to sled with different trade-offs:

```rust
pub struct RedbStoreTree<'db, D, M> {
    db: Arc<redb::Database>,
    table_def: TableDefinition<'static, BincodeWrapper<M::PrimaryKey>, BincodeWrapper<M>>,
    // ...
}
```

Redb requires compile-time table definitions, so we wrap types in `BincodeWrapper<T>` to implement Redb's `Value` trait, and it gives stronger [ACID][17] guarantees in return. Despite that, the `NetabaseTreeSync` implementation looks nearly identical from outside.

## Summary

Trait-based backend abstraction gets you portability, type safety, no runtime overhead, easy testing across backends, and a clean path for adding new ones later.

The techniques that make it work are associated types for cleaner APIs, lifetime parameters for resource safety, phantom types for zero-cost type tracking, batch operations for atomic consistency, and careful serialization so the backends stay compatible with each other.

Next up: the configuration API and transaction system, which is how we provide a unified way to configure different backends and manage multi-operation transactions without the overhead.

## References

[1]: https://doc.rust-lang.org/book/ch10-02-traits.html
[2]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types
[3]: https://doc.rust-lang.org/book/ch10-01-syntax.html
[4]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[5]: https://docs.rs/sled/
[6]: https://doc.rust-lang.org/nomicon/phantom-data.html
[7]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
[8]: https://docs.rs/bincode/
[9]: https://en.wikipedia.org/wiki/Database_index#Secondary_index
[10]: https://en.wikipedia.org/wiki/B-tree
[11]: https://webassembly.org/
[12]: https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
[13]: https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
[14]: https://doc.rust-lang.org/book/ch19-06-macros.html#zero-cost-abstractions
[15]: https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
[16]: https://docs.rs/redb/
[17]: https://en.wikipedia.org/wiki/ACID
[18]: https://serde.rs/
