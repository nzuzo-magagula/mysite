#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Where the 10-50x speedups came from: explicit transaction batching, zero-copy reads, and the lifetime hierarchy that keeps it safe"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/04-configuration-api-and-transaction-system"

[[references]]
title = "Zero-Copy Deserialization"
url = "https://docs.rs/zerocopy/latest/zerocopy/"
description = "Understanding zero-copy techniques in Rust"

[[references]]
title = "Redb Documentation"
url = "https://docs.rs/redb/latest/redb/"
description = "High-performance embedded database for Rust"

[[references]]
title = "Lifetimes - The Rust Book"
url = "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html"
description = "Understanding Rust lifetimes and borrowing"

[[references]]
title = "Performance - The Rust Book"
url = "https://doc.rust-lang.org/book/ch13-04-performance.html"
description = "Writing performant Rust code"
#####
# Performance Optimization and the Zero-Copy API

The [previous article](./04-configuration-api-and-transaction-system.md) covered configuration and transactions. This last one is about the zero-copy Redb backend: eliminating deserialization overhead, using lifetime tracking to safely borrow database memory, and designing explicit transaction APIs for batch work.

## The Problem

The standard API from the earlier articles is decent:

```rust
let store = RedbStore::<MyDef>::new("app.redb")?;
let tree = store.open_tree::<User>();

// This works but...
for i in 0..1000 {
    tree.put(User { id: i, name: format!("User{}", i), ... })?;
}
```

Every `put()` creates its own transaction, which is where the time goes:

```
put(user1): create_txn → write → commit → destroy
put(user2): create_txn → write → commit → destroy
put(user3): create_txn → write → commit → destroy
```

For Redb, creating a transaction means acquiring an exclusive lock, allocating transaction metadata, syncing to the [write-ahead log][4] on commit, and releasing the lock. For small operations that overhead is the entire cost.

There's a second problem too:

```rust
let user = tree.get(UserPrimaryKey(1))?;  // Always deserializes
```

Even if you only want to know whether a user exists, you pay to deserialize the whole model from [bincode][5].

## The Zero-Copy Backend

The `redb-zerocopy` backend addresses both.

Explicit transactions solve the first:

```rust
let store = RedbStoreZeroCopy::<MyDef>::new("app.redb")?;

let mut txn = store.begin_write()?;
let mut tree = txn.open_tree::<User>()?;

for i in 0..1000 {
    tree.put(User { id: i, name: format!("User{}", i), ... })?;
}

drop(tree);
txn.commit()?;  // One transaction for all 1000 operations
```

And borrowed reads will solve the second, though that part is still future work:

```rust
let txn = store.begin_read()?;
let tree = txn.open_tree::<User>()?;
let user_ref = tree.get_ref(&UserPrimaryKey(1))?;  // Borrows instead of cloning
```

## Benchmarks

Inserting 1000 items: the old wrapper loop takes 25.737 ms. Bulk zero-copy takes 2.827 ms, about 9x faster. The zero-copy loop takes 3.958 ms, about 6.5x faster.

Ten secondary key queries: the old wrapper loop takes 1030.03 µs, the zero-copy transaction takes 5.11 µs. That's roughly 200x, and it's almost entirely transaction reuse rather than anything clever.

Bulk operations on 1000 items: 34.156 ms against 2.940 ms, so around 11.6x.

## The Lifetime Hierarchy

The backend uses strict lifetime tracking:

```
RedbStoreZeroCopy<D>                    ('static or app lifetime)
  ↓ begin_write() / begin_read()
RedbWriteTransactionZC<'db, D>          (borrows 'db from store)
RedbReadTransactionZC<'db, D>           (borrows 'db from store)
  ↓ open_tree<M>()
RedbTreeMut<'txn, 'db, D, M>            (borrows 'txn from transaction)
RedbTree<'txn, 'db, D, M>               (borrows 'txn from transaction)
  ↓ get(), remove(), etc.
Model data (owned or borrowed)
```

Each level borrows from the one above, so trees can't outlive transactions and transactions can't outlive the store.

## The Store

```rust
pub struct RedbStoreZeroCopy<D>
where
    D: NetabaseDefinitionTrait,
{
    db: Arc<Database>,
    _phantom: PhantomData<D>,
}

impl<D> RedbStoreZeroCopy<D>
where
    D: NetabaseDefinitionTrait,
{
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, NetabaseError> {
        let db = Database::create(path)?;
        Ok(Self {
            db: Arc::new(db),
            _phantom: PhantomData,
        })
    }

    pub fn begin_write(&self) -> Result<RedbWriteTransactionZC<'_, D>, NetabaseError> {
        let txn = self.db.begin_write()?;
        Ok(RedbWriteTransactionZC {
            inner: txn,
            _phantom: PhantomData,
        })
    }

    pub fn begin_read(&self) -> Result<RedbReadTransactionZC<'_, D>, NetabaseError> {
        let txn = self.db.begin_read()?;
        Ok(RedbReadTransactionZC {
            inner: txn,
            _phantom: PhantomData,
        })
    }
}
```

The [`Arc`][6] around `Database` allows multiple concurrent readers, the `'_` in the return types ties transactions to the store, and there's no mutable state, so read transactions are thread-safe.

## Write Transactions

```rust
pub struct RedbWriteTransactionZC<'db, D> {
    inner: WriteTransaction,
    _phantom: PhantomData<&'db D>,
}

impl<'db, D> RedbWriteTransactionZC<'db, D>
where
    D: NetabaseDefinitionTrait,
{
    pub fn open_tree<M>(&mut self) -> Result<RedbTreeMut<'_, 'db, D, M>, NetabaseError>
    where
        M: NetabaseModelTrait<D>,
    {
        // Get static table definitions from the definition trait
        let table_name = M::discriminant_name();

        Ok(RedbTreeMut {
            txn: self,
            table_name,
            _phantom: PhantomData,
        })
    }

    pub fn commit(self) -> Result<(), NetabaseError> {
        self.inner.commit()?;
        Ok(())
    }

    pub fn abort(self) -> Result<(), NetabaseError> {
        self.inner.abort()?;
        Ok(())
    }
}
```

The mutable tree borrows from the transaction:

```rust
pub struct RedbTreeMut<'txn, 'db, D, M> {
    txn: &'txn mut RedbWriteTransactionZC<'db, D>,
    table_name: &'static str,
    _phantom: PhantomData<M>,
}
```

Two lifetimes: `'txn` for the transaction borrow, `'db` for the database, propagated through the transaction.

## Tree Operations

```rust
impl<'txn, 'db, D, M> RedbTreeMut<'txn, 'db, D, M>
where
    D: NetabaseDefinitionTrait,
    M: NetabaseModelTrait<D>,
{
    pub fn put(&mut self, model: M) -> Result<(), NetabaseError> {
        // Extract keys
        let pk = model.primary_key();
        let sk_list = model.secondary_keys();

        // Serialize
        let pk_bytes = bincode::encode_to_vec(&pk, bincode::config::standard())?;
        let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;

        // Get table from transaction
        let mut table = self.txn.inner.open_table(self.table_name)?;

        // Check for existing entry (for secondary key cleanup)
        let old_model: Option<M> = table.get(&pk_bytes)?
            .map(|v| bincode::decode_from_slice(v.value(), bincode::config::standard())
                .map(|(m, _)| m))
            .transpose()?;

        // Remove old secondary keys if updating
        if let Some(old) = old_model {
            let mut sec_table = self.txn.inner.open_multimap_table(
                &format!("{}_secondary", self.table_name)
            )?;

            for old_sk in old.secondary_keys() {
                let sk_bytes = bincode::encode_to_vec(&old_sk, bincode::config::standard())?;
                sec_table.remove(&sk_bytes, &pk_bytes)?;
            }
        }

        // Insert primary record
        table.insert(&pk_bytes, &model_bytes)?;

        // Insert new secondary indexes
        let mut sec_table = self.txn.inner.open_multimap_table(
            &format!("{}_secondary", self.table_name)
        )?;

        for sk in sk_list {
            let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
            sec_table.insert(&sk_bytes, &pk_bytes)?;
        }

        Ok(())
    }
}
```

Everything uses the same transaction (`self.txn.inner`). That's the whole trick behind the batching: multiple `put()` calls accumulate in memory and commit together.

Bulk methods go further by avoiding repeated transaction access:

```rust
pub fn put_many(&mut self, models: Vec<M>) -> Result<(), NetabaseError> {
    // Open tables once
    let mut table = self.txn.inner.open_table(self.table_name)?;
    let mut sec_table = self.txn.inner.open_multimap_table(
        &format!("{}_secondary", self.table_name)
    )?;

    // Process all models in one go
    for model in models {
        let pk = model.primary_key();
        let pk_bytes = bincode::encode_to_vec(&pk, bincode::config::standard())?;
        let model_bytes = bincode::encode_to_vec(&model, bincode::config::standard())?;

        table.insert(&pk_bytes, &model_bytes)?;

        for sk in model.secondary_keys() {
            let sk_bytes = bincode::encode_to_vec(&sk, bincode::config::standard())?;
            sec_table.insert(&sk_bytes, &pk_bytes)?;
        }
    }

    Ok(())
}
```

Tables get opened once, there are no repeated borrow checks, and everything stays in one transaction.

## Read Transactions

The read side is simpler because it's immutable:

```rust
pub struct RedbReadTransactionZC<'db, D> {
    inner: ReadTransaction,
    _phantom: PhantomData<&'db D>,
}

impl<'db, D> RedbReadTransactionZC<'db, D> {
    pub fn open_tree<M>(&self) -> Result<RedbTree<'_, 'db, D, M>, NetabaseError>
    where
        M: NetabaseModelTrait<D>,
    {
        Ok(RedbTree {
            txn: self,
            table_name: M::discriminant_name(),
            _phantom: PhantomData,
        })
    }
}
```

```rust
pub struct RedbTree<'txn, 'db, D, M> {
    txn: &'txn RedbReadTransactionZC<'db, D>,
    table_name: &'static str,
    _phantom: PhantomData<M>,
}

impl<'txn, 'db, D, M> RedbTree<'txn, 'db, D, M> {
    pub fn get(&self, key: &M::PrimaryKey) -> Result<Option<M>, NetabaseError>
    where
        M: NetabaseModelTrait<D>,
    {
        let table = self.txn.inner.open_table(self.table_name)?;
        let pk_bytes = bincode::encode_to_vec(key, bincode::config::standard())?;

        match table.get(&pk_bytes)? {
            Some(value) => {
                let (model, _) = bincode::decode_from_slice(
                    value.value(),
                    bincode::config::standard()
                )?;
                Ok(Some(model))
            }
            None => Ok(None),
        }
    }
}
```

The remaining optimization is a `get_ref()` that returns a borrowed reference instead of cloning. That needs the [`ouroboros`][8] crate for self-referential structs, and it hasn't happened yet.

## In Practice

Batch import:

```rust
fn import_users(store: &RedbStoreZeroCopy<AppDef>, csv_path: &str)
    -> Result<(), NetabaseError>
{
    let users = load_from_csv(csv_path)?;

    let mut txn = store.begin_write()?;
    let mut tree = txn.open_tree::<User>()?;

    tree.put_many(users)?;  // All in one transaction

    drop(tree);
    txn.commit()?;

    Ok(())
}
```

Roughly 10x faster than individual `put()` calls.

Complex updates, where you want the read and the write in the same transaction:

```rust
fn update_user_email(
    store: &RedbStoreZeroCopy<AppDef>,
    user_id: u64,
    new_email: String
) -> Result<(), NetabaseError> {
    let mut txn = store.begin_write()?;
    let mut tree = txn.open_tree::<User>()?;

    // Get existing user
    let mut user = tree.get(&UserPrimaryKey(user_id))?
        .ok_or_else(|| NetabaseError::NotFound)?;

    // Update email
    user.email = new_email;

    // Save (automatically updates secondary indexes)
    tree.put(user)?;

    drop(tree);
    txn.commit()?;

    Ok(())
}
```

Read-heavy work, where read transactions can run concurrently:

```rust
fn find_users_by_email(
    store: &RedbStoreZeroCopy<AppDef>,
    email: &str
) -> Result<Vec<User>, NetabaseError> {
    let txn = store.begin_read()?;
    let tree = txn.open_tree::<User>()?;

    let results = tree.get_by_secondary_key(
        &UserSecondaryKeys::Email(UserEmailSecondaryKey(email.to_string()))
    )?;

    // Transaction auto-closes when txn drops
    Ok(results)
}
```

## When to Use It

Use the zero-copy backend for batch operations like imports and exports, for complex transactions where several related changes have to be atomic, when performance genuinely matters, and when you want explicit control over transaction boundaries.

Use the standard backend for one-off operations and prototyping, when you'd rather transactions were managed for you, and while you're still learning the library.

It composes with the configuration API:

```rust
use netabase_store::config::FileConfig;
use netabase_store::traits::backend_store::BackendStore;
use netabase_store::databases::redb_zerocopy::RedbStoreZeroCopy;

let config = FileConfig::builder()
    .path("app.redb".into())
    .cache_size_mb(1024)
    .build();

let store = <RedbStoreZeroCopy<MyDef> as BackendStore<MyDef>>::new(config)?;

// Use explicit transactions
let mut txn = store.begin_write()?;
let mut tree = txn.open_tree::<User>()?;
tree.put_many(users)?;
drop(tree);
txn.commit()?;
```

## Design Notes

Lifetime propagation is what holds it together. Each type borrows from its parent, so dropping the store or a transaction automatically invalidates everything derived from it.

The other thing worth noting is the trade between explicit and implicit. The standard API hides the transaction:

```rust
tree.put(user)?;  // Invisible transaction
```

The zero-copy API doesn't:

```rust
let mut txn = store.begin_write()?;
let mut tree = txn.open_tree::<User>()?;
tree.put(user)?;
txn.commit()?;  // Visible transaction
```

Being explicit is what enables the optimization. It also asks more of the caller, which is why both APIs exist.

Finally, the types themselves carry the guarantees. `RedbWriteTransactionZC` can open mutable trees and `RedbReadTransactionZC` can't, so writing through a read transaction isn't a bug you find at runtime.

## Wrapping Up the Series

Across five articles we've gone from an idea to a working type-safe database abstraction: architecture and overview, procedural macros for code generation, trait-based backend abstraction, configuration and transactions, and finally performance.

The zero-copy backend leans on lifetime tracking for safe memory access, the type-state pattern for compile-time guarantees, explicit transactions for batching, and the backend abstraction underneath all of it. There's no `unsafe` anywhere in it, and the speedups are between 10x and 50x depending on the workload.

The techniques generalize beyond this project. Use procedural macros to eliminate boilerplate. Design traits for flexibility. Lean on lifetimes for compile-time safety. Apply the type-state pattern where API correctness matters. And profile before you optimize, because the bottleneck was transaction overhead, not anything I'd have guessed.

## References

[1]: https://doc.rust-lang.org/book/ch19-06-macros.html#zero-cost-abstractions
[2]: https://docs.rs/redb/
[3]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
[4]: https://en.wikipedia.org/wiki/Write-ahead_logging
[5]: https://docs.rs/bincode/
[6]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[7]: https://doc.rust-lang.org/nomicon/phantom-data.html
[8]: https://docs.rs/ouroboros/
[9]: https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html
[10]: https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html
[11]: https://cliffle.com/blog/rust-typestate/
[12]: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html
[13]: https://doc.rust-lang.org/reference/procedural-macros.html
[14]: https://doc.rust-lang.org/book/ch10-02-traits.html
