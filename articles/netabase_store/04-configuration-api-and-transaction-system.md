#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "A configuration API that initializes any backend the same way, and a transaction system that makes writing through a read-only transaction a compile error"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/03-backend-implementation-and-trait-design"
next = "netabase_store/05-performance-optimization-and-zerocopy-api"

[[references]]
title = "Builder Pattern - Rust Design Patterns"
url = "https://rust-unofficial.github.io/patterns/patterns/creational/builder.html"
description = "Guide to the builder pattern in Rust"

[[references]]
title = "Type-State Pattern"
url = "https://cliffle.com/blog/rust-typestate/"
description = "Using the type system to encode state machines"

[[references]]
title = "Zero-Cost Abstractions"
url = "https://doc.rust-lang.org/book/ch13-04-performance.html"
description = "Understanding Rust's zero-cost abstractions"

[[references]]
title = "Traits - The Rust Book"
url = "https://doc.rust-lang.org/book/ch10-02-traits.html"
description = "Comprehensive guide to Rust traits"
#####
# Configuration API and Transaction System

[Part 3](./03-backend-implementation-and-trait-design.md) covered how trait-based design gives you backend portability. This one covers the two systems that make the library usable in practice: a configuration API that initializes any backend the same way, and a transaction system that catches misuse at compile time.

They demonstrate two different Rust patterns, the [builder pattern][2] for configuration and the [type-state pattern][3] for transactions.

## The Configuration Problem

Before there was a unified system, initialization looked like this:

```rust
// Different constructors for each backend
let sled = SledStore::new("path.db")?;
let redb = RedbStore::open_with_path("path.redb")?;
let temp = SledStore::temp()?;

// Different configuration parameters
let sled = SledStore::with_cache_size("db", 512)?;
let redb = RedbStore::new_with_options("db", RedbOptions { ... })?;
```

Every backend had its own initialization pattern, which made switching between them annoying enough that nobody would.

## Unified Configuration

I used the [`typed-builder`][5] crate to build type-safe, self-documenting configuration objects:

```rust
use typed_builder::TypedBuilder;
use std::path::PathBuf;

#[derive(Debug, Clone, TypedBuilder)]
#[builder(doc)]
pub struct FileConfig {
    /// Path to the database file or directory
    pub path: PathBuf,

    /// Cache size in megabytes
    #[builder(default = 256)]
    pub cache_size_mb: usize,

    /// Whether to create the database if it doesn't exist
    #[builder(default = true)]
    pub create_if_missing: bool,

    /// Whether to truncate/recreate if database already exists
    #[builder(default = false)]
    pub truncate: bool,

    /// Read-only mode (if supported by backend)
    #[builder(default = false)]
    pub read_only: bool,

    /// Enable fsync for durability (may impact performance)
    #[builder(default = true)]
    pub use_fsync: bool,
}
```

The derive gives you required fields that must be set, optional ones with defaults, type checking at compile time, IDE autocomplete showing available options, and documentation on each field. Usage stays readable:

```rust
let config = FileConfig::builder()
    .path("my_app.db".into())
    .cache_size_mb(512)
    .truncate(true)
    .build();
```

For simple cases there are shortcuts:

```rust
impl FileConfig {
    /// Create with just a path, using defaults
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            cache_size_mb: 256,
            create_if_missing: true,
            truncate: false,
            read_only: false,
            use_fsync: true,
        }
    }

    /// Create a temporary database configuration
    pub fn temp() -> Self {
        let temp_path = std::env::temp_dir()
            .join(format!("netabase_{}", uuid::Uuid::new_v4()));
        Self::new(temp_path)
    }
}

// Usage
let config = FileConfig::new("app.db");  // Simple
let temp = FileConfig::temp();            // For testing
```

## The BackendStore Trait

To consume those configs uniformly:

```rust
pub trait BackendStore<D: NetabaseDefinitionTrait>: Sized {
    type Config;

    /// Create/open a database with the provided configuration
    fn new(config: Self::Config) -> Result<Self, NetabaseError>;

    /// Open an existing database (fails if missing)
    fn open(config: Self::Config) -> Result<Self, NetabaseError>;

    /// Create a temporary database (for testing)
    fn temp() -> Result<Self, NetabaseError>;
}
```

Each backend implements it with its own config type:

```rust
impl<D> BackendStore<D> for SledStore<D>
where
    D: NetabaseDefinitionTrait,
{
    type Config = FileConfig;

    fn new(config: FileConfig) -> Result<Self, NetabaseError> {
        let mut sled_config = sled::Config::new()
            .path(&config.path)
            .cache_capacity(config.cache_size_mb * 1024 * 1024);

        if config.truncate && config.path.exists() {
            std::fs::remove_dir_all(&config.path)?;
        }

        if config.read_only {
            sled_config = sled_config.mode(sled::Mode::LowSpace);
        }

        let db = sled_config.open()?;

        Ok(SledStore {
            db,
            trees: Vec::new(),
        })
    }

    fn open(config: FileConfig) -> Result<Self, NetabaseError> {
        let mut cfg = config;
        cfg.create_if_missing = false;
        Self::new(cfg)
    }

    fn temp() -> Result<Self, NetabaseError> {
        Self::new(FileConfig::temp())
    }
}
```

Switching backends becomes trivial:

```rust
use netabase_store::config::FileConfig;
use netabase_store::traits::backend_store::BackendStore;

let config = FileConfig::builder()
    .path("database.db".into())
    .cache_size_mb(512)
    .build();

// Try different backends - same config!
#[cfg(feature = "sled")]
let store = <SledStore<MyDef> as BackendStore<MyDef>>::new(config.clone())?;

#[cfg(feature = "redb")]
let store = <RedbStore<MyDef> as BackendStore<MyDef>>::new(config.clone())?;

// API is identical from here on
let tree = store.open_tree::<User>();
tree.put(user)?;
```

The same `FileConfig` works everywhere because it represents common database concepts rather than backend-specific details.

Some backends need their own config shape. In-memory:

```rust
#[derive(Debug, Clone, TypedBuilder)]
pub struct MemoryConfig {
    #[builder(default = 1000)]
    pub initial_capacity: usize,

    #[builder(default = None)]
    pub max_entries: Option<usize>,
}

// Usage
let config = MemoryConfig::builder()
    .initial_capacity(10000)
    .max_entries(Some(1000000))
    .build();

let store = <MemoryStore<MyDef> as BackendStore<MyDef>>::new(config)?;
```

And IndexedDB:

```rust
#[derive(Debug, Clone, TypedBuilder)]
pub struct IndexedDBConfig {
    pub database_name: String,

    #[builder(default = 1)]
    pub version: u32,
}

// Usage (in WASM)
let config = IndexedDBConfig::builder()
    .database_name("my_app_store".to_string())
    .version(2)
    .build();

let store = <IndexedDBStore<MyDef> as BackendStore<MyDef>>::new(config).await?;
```

## The Transaction Problem

Originally every operation created its own transaction:

```rust
// OLD: Each operation = one transaction
tree.put(user1)?;  // Transaction 1: open → put → commit
tree.put(user2)?;  // Transaction 2: open → put → commit
tree.put(user3)?;  // Transaction 3: open → put → commit
```

For Redb this was catastrophically slow, because each transaction acquires an exclusive lock, creates transaction metadata, commits to the write-ahead log, then releases the lock. Do that a thousand times and the overhead is the whole runtime.

## Type-State for Transactions

The fix is reusable transactions with the mode tracked at compile time.

The mode markers cost nothing:

```rust
/// Zero-cost marker type for read-only transactions
pub struct ReadOnly;

/// Zero-cost marker type for read-write transactions
pub struct ReadWrite;
```

They exist only at compile time, generate no runtime code, and enable compile-time dispatch.

```rust
pub struct TxnGuard<'db, D, Mode> {
    backend: TxnBackend<'db, D>,
    _mode: PhantomData<Mode>,  // Zero-cost type marker
}
```

The `Mode` parameter determines which methods exist:

```rust
// Operations on ALL modes
impl<'db, D, Mode> TxnGuard<'db, D, Mode> {
    pub fn open_tree<M>(&mut self) -> TreeView<'_, D, M, Mode> {
        // Implementation
    }
}

// Operations ONLY on ReadWrite mode
impl<'db, D> TxnGuard<'db, D, ReadWrite> {
    pub fn commit(self) -> Result<(), NetabaseError> {
        // Implementation
    }

    pub fn rollback(self) -> Result<(), NetabaseError> {
        // Implementation
    }
}
```

Which means this is a compile error rather than a runtime failure:

```rust
let txn = store.read();  // Type: TxnGuard<ReadOnly>
let tree = txn.open_tree::<User>();  // Type: TreeView<ReadOnly>

// Read operations work
let user = tree.get(UserPrimaryKey(1))?;

// Write operations don't compile
tree.put(user)?;
// Error: no method named `put` found for struct `TreeView<'_, D, User, ReadOnly>`
```

Tree views inherit the mode the same way:

```rust
pub struct TreeView<'txn, D, M, Mode> {
    backend: TreeBackend<'txn, D, M>,
    _mode: PhantomData<Mode>,
}

// Read operations on ALL modes
impl<'txn, D, M, Mode> TreeView<'txn, D, M, Mode> {
    pub fn get(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError> {
        // Implementation
    }

    pub fn len(&self) -> Result<usize, NetabaseError> {
        // Implementation
    }
}

// Write operations ONLY on ReadWrite mode
impl<'txn, D, M> TreeView<'txn, D, M, ReadWrite> {
    pub fn put(&mut self, model: M) -> Result<(), NetabaseError> {
        // Implementation
    }

    pub fn remove(&mut self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError> {
        // Implementation
    }
}
```

## How Each Backend Handles It

Sled doesn't have true multi-tree transactions, so operations apply immediately:

```rust
pub(crate) struct SledTreeBackend<'txn, D, M> {
    pub(crate) tree: sled::Tree,          // Arc-based, cheap to clone
    pub(crate) secondary_tree: sled::Tree,
    pub(crate) _phantom: PhantomData<(&'txn (), D, M)>,
}

// Put applies immediately to the tree
impl<'txn, D, M> TreeView<'txn, D, M, ReadWrite>
where
    TreeBackend<'txn, D, M>: From<SledTreeBackend<'txn, D, M>>,
{
    pub fn put(&mut self, model: M) -> Result<(), NetabaseError> {
        // Directly insert into sled::Tree
        self.backend.tree.insert(key_bytes, model_bytes)?;
        Ok(())
    }
}
```

Redb stores and reuses the transaction:

```rust
pub(crate) struct RedbTxnBackend<'db, D> {
    pub(crate) read_txn: RefCell<Option<redb::ReadTransaction>>,
    pub(crate) write_txn: RefCell<Option<redb::WriteTransaction>>,
    pub(crate) db: &'db Arc<redb::Database>,
    pub(crate) _phantom: PhantomData<D>,
}

// All operations reuse the same transaction
impl<'db, D> TxnGuard<'db, D, ReadWrite> {
    pub fn commit(self) -> Result<(), NetabaseError> {
        match self.backend {
            TxnBackend::Redb(redb) => {
                let write_txn = redb.write_txn.borrow_mut().take()
                    .ok_or(NetabaseError::TransactionError("No write transaction".to_string()))?;
                write_txn.commit()?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
```

Every operation shares the same Redb transaction until `commit()` is called, which is where the speedup comes from.

## Using It

Read-only:

```rust
let txn = store.read();
let user_tree = txn.open_tree::<User>();
let post_tree = txn.open_tree::<Post>();

let user = user_tree.get(UserPrimaryKey(1))?;
let posts = post_tree.get_by_secondary_key(
    PostSecondaryKeys::AuthorId(PostAuthorIdSecondaryKey(1))
)?;

// Auto-closes on drop - no explicit cleanup needed
```

Read-write:

```rust
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

// All operations in a single transaction
for i in 0..1000 {
    tree.put(User {
        id: i,
        name: format!("User {}", i),
        email: format!("user{}@example.com", i),
    })?;
}

txn.commit()?;  // Atomic commit of all 1000 inserts
// Or drop to rollback
```

With an explicit rollback:

```rust
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

tree.put(user)?;

if some_condition {
    txn.rollback()?;  // Explicitly abort
} else {
    txn.commit()?;    // Or commit
}
```

## What It Bought

Transaction reuse made a large difference for Redb. A thousand inserts went from around 250ms to around 5ms. A thousand reads went from around 150ms to around 3ms. Mixed operations went from around 200ms to around 4ms. Roughly a 50x improvement across the board.

For sled the improvement is much smaller, since there wasn't transaction overhead to remove, but the API is cleaner either way.

The two systems compose:

```rust
// Configure the store
let config = FileConfig::builder()
    .path("app.db".into())
    .cache_size_mb(1024)
    .build();

let store = <RedbStore<MyDef> as BackendStore<MyDef>>::new(config)?;

// Use transactions
let mut txn = store.write()?;
let mut tree = txn.open_tree::<User>();

tree.put_many(users)?;  // Bulk insert in one transaction
txn.commit()?;
```

## Patterns Used

The configuration API uses the builder pattern for ergonomic construction, [associated types][10] so each backend declares its own config, a shared trait for the unified interface, and sensible defaults so only the required fields are mandatory.

The transaction system uses the type-state pattern for compile-time mode tracking, [phantom types][7] for zero-cost polymorphism, [RAII][12] for automatic rollback on drop, and [`RefCell`][14] for shared access to the transaction.

Between them you get no runtime overhead, unreachable invalid states, memory safety through lifetimes, and code that ports between backends.

Next article: the zero-copy API for Redb, where careful use of lifetimes and the [`ouroboros`][16] crate lets you read data without deserialization overhead.

## References

[1]: https://doc.rust-lang.org/book/ch19-06-macros.html#zero-cost-abstractions
[2]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
[3]: https://cliffle.com/blog/rust-typestate/
[4]: https://doc.rust-lang.org/book/ch10-02-traits.html
[5]: https://docs.rs/typed-builder/
[6]: https://doc.rust-lang.org/reference/procedural-macros.html
[7]: https://doc.rust-lang.org/nomicon/phantom-data.html
[8]: https://docs.rs/sled/
[9]: https://docs.rs/redb/
[10]: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#specifying-placeholder-types-in-trait-definitions-with-associated-types
[11]: https://doc.rust-lang.org/book/ch17-02-trait-objects.html
[12]: https://doc.rust-lang.org/rust-by-example/scope/raii.html
[13]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
[14]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
[15]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
[16]: https://docs.rs/ouroboros/
