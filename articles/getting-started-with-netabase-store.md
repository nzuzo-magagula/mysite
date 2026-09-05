#####
date = "2025-11-21"
author = "Nzuzo Magagula"
summary = "A practical guide to netabase_store, from basic CRUD through batching and transactions to the zero-copy API"
thumbnail = "https://i.postimg.cc/ydzVB88M/7714.jpg"
category = "Tutorial"
show_references = true

[[references]]
title = "netabase_store on crates.io"
url = "https://crates.io/crates/netabase_store"
description = "Official crate page for netabase_store"

[[references]]
title = "Rust Book - Using Structs"
url = "https://doc.rust-lang.org/book/ch05-00-structs.html"
description = "Understanding Rust structs and data modeling"

[[references]]
title = "Serde - Serialization Framework"
url = "https://serde.rs/"
description = "Serialize and deserialize Rust data structures"

[[references]]
title = "Bincode Documentation"
url = "https://docs.rs/bincode/latest/bincode/"
description = "Binary serialization for Rust"

[[references]]
title = "Result and Error Handling"
url = "https://doc.rust-lang.org/book/ch09-00-error-handling.html"
description = "Rust's approach to error handling"
#####

# Getting Started with netabase_store

`netabase_store` is a type-safe, multi-backend key-value storage library for Rust. Whether you're writing a desktop app on Sled, a server on Redb, or a web app on IndexedDB, you get the same API and the same compile-time guarantees.

This is the practical guide: basic CRUD first, then batching, transactions, backend portability, and the zero-copy API for when performance matters.

The short version of what you get: the compiler catches type mismatches instead of the database doing it at runtime, the same code runs on any supported backend, procedural macros write the repetitive parts, secondary indexes maintain themselves, and none of it costs you anything at runtime.

## Prerequisites

Add netabase_store to your `Cargo.toml`:

```toml
[dependencies]
netabase_store = { version = "0.1", features = ["native", "sled"] }
bincode = "2.0"
serde = { version = "1.0", features = ["derive"] }

[dependencies.anyhow]
version = "1.0"  # For error handling in examples
```

## Part 1: Defining Your Schema

Every netabase_store application starts by defining models. Models are regular Rust structs annotated with procedural macros.

### Your First Model

```rust
use netabase_store::{netabase_definition_module, NetabaseModel, netabase};

#[netabase_definition_module(AppDefinition, AppKeys)]
pub mod schema {
    use super::*;

    #[derive(
        NetabaseModel,
        Clone,
        Debug,
        PartialEq,
        bincode::Encode,
        bincode::Decode,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[netabase(AppDefinition)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        pub username: String,
        #[secondary_key]
        pub email: String,
    }
}

use schema::*;
```

Four things are happening. `#[netabase_definition_module(AppDefinition, AppKeys)]` wraps your models into a single database schema. `#[derive(NetabaseModel)]` generates the key types and indexing code. `#[primary_key]` marks `id` as the unique identifier, and you need exactly one per model. `#[secondary_key]` on `email` creates an index so you can look users up by email.

Out of that you get `UserPrimaryKey(u64)` and `UserEmailSecondaryKey(String)` as wrappers, `UserSecondaryKeys` as an enum of all the secondary indexes, `AppDefinition` wrapping all model types, and `AppKeys` covering all key types. All of it at compile time, none of it costing anything at runtime.

## Part 2: Basic CRUD Operations

Let's create a database and perform basic operations.

### Creating a Store

```rust
use netabase_store::NetabaseStore;

fn main() -> anyhow::Result<()> {
    // Create a temporary Sled database for this example
    let store = NetabaseStore::<AppDefinition, _>::temp()?;

    // Open a tree for the User model
    let user_tree = store.open_tree::<User>();

    Ok(())
}
```

### Create: Inserting Data

```rust
let user = User {
    id: 1,
    username: "alice".to_string(),
    email: "alice@example.com".to_string(),
};

user_tree.put(user.clone())?;
println!("✓ Inserted user: {}", user.username);
```

The `put()` method:
- Stores the model using its primary key
- Updates all secondary indexes automatically
- Is idempotent (calling it twice with the same ID updates the record)

### Read: Fetching by Primary Key

```rust
use netabase_store::traits::model::NetabaseModelTrait;

let retrieved = user_tree.get(user.primary_key())?;

match retrieved {
    Some(user) => println!("Found: {} ({})", user.username, user.email),
    None => println!("User not found"),
}
```

You can also use the generated key type directly:

```rust
let retrieved = user_tree.get(UserPrimaryKey(1))?;
```

### Update: Modifying Records

Updates use the same `put()` method:

```rust
let mut user = user_tree.get(UserPrimaryKey(1))?.unwrap();
user.email = "alice.new@example.com".to_string();

user_tree.put(user)?;
println!("✓ Updated user email");
```

Worth knowing: the library removes the stale secondary indexes, inserts the new ones, and does it atomically, so you never end up with an index pointing at a record that changed.

### Delete: Removing Records

```rust
let deleted = user_tree.remove(UserPrimaryKey(1))?;

match deleted {
    Some(user) => println!("Deleted: {}", user.username),
    None => println!("User not found"),
}
```

The `remove()` method returns the deleted model, allowing you to access its data one last time before it's gone.

### Counting and Checking

```rust
// Check if tree is empty
if user_tree.is_empty() {
    println!("No users in database");
}

// Get total count
let count = user_tree.len();
println!("Total users: {}", count);

// Clear all records (careful!)
user_tree.clear()?;
```

## Part 3: Querying with Secondary Keys

Secondary keys enable fast lookups without scanning the entire database.

### Simple Secondary Key Query

```rust
// Find users by email
let users = user_tree.get_by_secondary_key(
    UserSecondaryKeys::Email(
        UserEmailSecondaryKey("alice@example.com".to_string())
    )
)?;

for user in users {
    println!("Found: {} with email {}", user.username, user.email);
}
```

Secondary keys aren't unique, since multiple users could share an email, so the return type is `Vec<User>`.

### Multiple Secondary Keys

Add more secondary keys to your model:

```rust
#[derive(NetabaseModel, Clone, Debug, bincode::Encode, bincode::Decode)]
#[netabase(AppDefinition)]
pub struct User {
    #[primary_key]
    pub id: u64,
    pub username: String,
    #[secondary_key]
    pub email: String,
    #[secondary_key]
    pub country: String,
}
```

Now you can query by either index:

```rust
// Query by email
let by_email = user_tree.get_by_secondary_key(
    UserSecondaryKeys::Email(UserEmailSecondaryKey("alice@example.com".to_string()))
)?;

// Query by country
let by_country = user_tree.get_by_secondary_key(
    UserSecondaryKeys::Country(UserCountrySecondaryKey("USA".to_string()))
)?;

println!("Users in USA: {}", by_country.len());
```

### Convenience Extension Traits

The macros generate extension traits for ergonomic queries:

```rust
use schema::AsUserEmail;

let email_key = "alice@example.com".as_user_email_key();
let users = user_tree.get_by_secondary_key(
    UserSecondaryKeys::Email(email_key)
)?;
```

## Part 4: Batch Operations and Bulk Imports

For inserting multiple records, batch operations are 10-100x faster than individual `put()` calls.

### The Problem with Individual Inserts

```rust
// Slow: each insert creates its own transaction
for i in 0..1000 {
    user_tree.put(User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })?;
}
// This could take 1-2 seconds!
```

### Using Batch Operations

```rust
use netabase_store::traits::batch::{Batchable, BatchBuilder};

// Fast: all inserts in one transaction
let mut batch = user_tree.create_batch()?;

for i in 0..1000 {
    batch.put(User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })?;
}

batch.commit()?;
// This takes ~10-50ms!
```

That's 50 to 100x faster for bulk work.

### Real-World Example: CSV Import

```rust
use std::time::Instant;

fn import_users_from_csv(
    user_tree: &impl Batchable<AppDefinition, User>,
    csv_path: &str
) -> anyhow::Result<()> {
    const BATCH_SIZE: usize = 1000;

    let users = load_from_csv(csv_path)?;
    let total = users.len();

    println!("Importing {} users...", total);
    let start = Instant::now();

    // Process in chunks
    for (i, chunk) in users.chunks(BATCH_SIZE).enumerate() {
        let mut batch = user_tree.create_batch()?;

        for user in chunk {
            batch.put(user.clone())?;
        }

        batch.commit()?;

        if (i + 1) * BATCH_SIZE % 5000 == 0 {
            println!("  Imported {} users...", (i + 1) * BATCH_SIZE);
        }
    }

    let elapsed = start.elapsed();
    println!("✓ Imported {} users in {:?}", total, elapsed);
    println!("  Average: {:?} per user", elapsed / total as u32);

    Ok(())
}
```

### Atomic Batch Updates

Batches are atomic: all the changes succeed or none of them do.

```rust
// Mark all users from a country as inactive
let users = user_tree.get_by_secondary_key(
    UserSecondaryKeys::Country(UserCountrySecondaryKey("UK".to_string()))
)?;

let mut batch = user_tree.create_batch()?;

for mut user in users {
    user.active = false;
    batch.put(user)?;
}

batch.commit()?;  // All or nothing
println!("✓ Deactivated {} UK users atomically", users.len());
```

## Part 5: Transaction Management

Transactions provide explicit control over database operations with compile-time safety guarantees.

### Read-Only Transactions

```rust
let mut txn = store.read();
let user_tree = txn.open_tree::<User>();

// Read operations work
let user = user_tree.get(UserPrimaryKey(1))?;
let count = user_tree.len()?;

// This won't compile:
// user_tree.put(user)?;
// Error: no method `put` found for `TreeView<'_, D, User, ReadOnly>`

// Transaction auto-closes on drop
```

The type system prevents accidental writes through read-only transactions!

### Read-Write Transactions

```rust
let mut txn = store.write();
let mut user_tree = txn.open_tree::<User>();

// All operations share the same transaction
for i in 0..1000 {
    user_tree.put(User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })?;
}

txn.commit()?;  // Atomic commit of all 1000 inserts
```

**Performance:** Same 50-100x improvement as batches, but with more control.

### Multi-Model Transactions

```rust
#[derive(NetabaseModel, Clone, Debug, bincode::Encode, bincode::Decode)]
#[netabase(AppDefinition)]
pub struct Post {
    #[primary_key]
    pub id: u64,
    pub title: String,
    #[secondary_key]
    pub author_id: u64,
}
```

```rust
let mut txn = store.write();

// Insert user
let user = User {
    id: 100,
    username: "blogger".to_string(),
    email: "blogger@example.com".to_string(),
};

{
    let mut user_tree = txn.open_tree::<User>();
    user_tree.put(user.clone())?;
}  // Drop user_tree to release mutable borrow

// Insert posts for that user
let mut post_tree = txn.open_tree::<Post>();
for i in 0..5 {
    post_tree.put(Post {
        id: i,
        title: format!("Post #{}", i),
        author_id: user.id,
    })?;
}

txn.commit()?;  // User and 5 posts committed atomically
```

### Automatic Rollback

```rust
{
    let mut txn = store.write();
    let mut user_tree = txn.open_tree::<User>();

    user_tree.put(User {
        id: 9999,
        username: "temporary".to_string(),
        email: "temp@example.com".to_string(),
    })?;

    // Transaction drops here without commit = automatic rollback
}

// User 9999 was never inserted
```

### Bulk Operations with Helpers

```rust
let mut txn = store.write();
let mut user_tree = txn.open_tree::<User>();

let users: Vec<User> = (0..100)
    .map(|i| User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })
    .collect();

user_tree.put_many(users)?;  // Optimized bulk insert
txn.commit()?;
```

## Part 6: Backend Portability

The point of the trait layer is that you write the code once and it runs against any backend.

### Using Different Backends

```rust
use netabase_store::config::FileConfig;

// Sled backend (high throughput, lock-free)
let sled_store = NetabaseStore::<AppDefinition, _>::sled("./data.sled")?;

// Redb backend (ACID compliance, crash recovery)
let redb_store = NetabaseStore::<AppDefinition, _>::redb("./data.redb")?;

// Memory backend (testing, temporary data)
let mem_store = NetabaseStore::<AppDefinition, _>::memory()?;

// All three use the EXACT same API!
```

### Configuration API

```rust
use netabase_store::config::FileConfig;
use netabase_store::traits::backend_store::BackendStore;

let config = FileConfig::builder()
    .path("./database.db".into())
    .cache_size_mb(512)
    .create_if_missing(true)
    .truncate(false)
    .read_only(false)
    .use_fsync(true)
    .build();

// Works with both Sled and Redb
let store = NetabaseStore::<AppDefinition, _>::sled_with_config(config)?;
```

### Backend-Agnostic Functions

```rust
use netabase_store::traits::tree::NetabaseTreeSync;

fn count_users_by_country<'a, T>(
    tree: &T,
    country: &str
) -> anyhow::Result<usize>
where
    T: NetabaseTreeSync<'a, AppDefinition, User>
{
    let users = tree.get_by_secondary_key(
        UserSecondaryKeys::Country(UserCountrySecondaryKey(country.to_string()))
    )?;
    Ok(users.len())
}

// Works with ANY backend!
let sled_count = count_users_by_country(&sled_tree, "USA")?;
let redb_count = count_users_by_country(&redb_tree, "USA")?;
```

## Part 7: High-Performance Zero-Copy API

For maximum performance, netabase_store offers a zero-copy API for Redb that's 10-50x faster for bulk operations.

### When to Use Zero-Copy

Use the zero-copy backend when:
- Importing/exporting large datasets
- Batch processing operations
- Every millisecond matters
- You need explicit transaction control

Use the standard backend when:
- Prototyping or learning
- Simple one-off operations
- You prefer automatic transaction management

### Basic Zero-Copy Usage

```rust
use netabase_store::databases::redb_zerocopy::RedbStoreZeroCopy;
use netabase_store::config::FileConfig;
use netabase_store::traits::backend_store::BackendStore;

let config = FileConfig::builder()
    .path("./app.redb".into())
    .build();

let store = <RedbStoreZeroCopy<AppDefinition> as BackendStore<AppDefinition>>::new(config)?;
```

### Explicit Transactions

```rust
// Write transaction
let mut txn = store.begin_write()?;
let mut tree = txn.open_tree::<User>()?;

for i in 0..1000 {
    tree.put(User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })?;
}

drop(tree);  // Must drop tree before committing
txn.commit()?;

println!("✓ Inserted 1000 users in single transaction");
```

### Bulk Operations

```rust
let users: Vec<User> = (0..10000)
    .map(|i| User {
        id: i,
        username: format!("user{}", i),
        email: format!("user{}@example.com", i),
    })
    .collect();

let mut txn = store.begin_write()?;
let mut tree = txn.open_tree::<User>()?;

let start = std::time::Instant::now();
tree.put_many(users)?;
drop(tree);
txn.commit()?;

println!("Inserted 10,000 users in {:?}", start.elapsed());
// Typical: 20-50ms vs 500-2000ms with per-operation transactions
```

### Read Transactions

```rust
let txn = store.begin_read()?;
let tree = txn.open_tree::<User>()?;

// Zero-copy read (future enhancement)
if let Some(user) = tree.get(&UserPrimaryKey(1))? {
    println!("Found: {} ({})", user.name, user.email);
}

// Secondary key queries
let users = tree.get_by_secondary_key(
    &UserSecondaryKeys::Email(UserEmailSecondaryKey("alice@example.com".to_string()))
)?;

// Transaction automatically closes on drop
```

### Helper Functions for Common Patterns

```rust
use netabase_store::databases::redb_zerocopy::*;

// Automatic commit on success, rollback on error
let count = with_write_transaction(&store, |txn| {
    let mut tree = txn.open_tree::<User>()?;
    tree.put(User {
        id: 1,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
    })?;
    Ok(tree.len()?)
})?;

// Read-only helper
let user = with_read_transaction(&store, |txn| {
    let tree = txn.open_tree::<User>()?;
    tree.get(&UserPrimaryKey(1))
})?;
```

## Part 8: Real-World Example - Blog Backend

Let's put it all together with a complete example:

```rust
use netabase_store::{netabase_definition_module, NetabaseStore};

#[netabase_definition_module(BlogDB, BlogKeys)]
pub mod models {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, Clone, Debug, bincode::Encode, bincode::Decode,
             serde::Serialize, serde::Deserialize)]
    #[netabase(BlogDB)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        pub username: String,
        #[secondary_key]
        pub email: String,
        pub created_at: u64,  // Unix timestamp
    }

    #[derive(NetabaseModel, Clone, Debug, bincode::Encode, bincode::Decode,
             serde::Serialize, serde::Deserialize)]
    #[netabase(BlogDB)]
    pub struct Post {
        #[primary_key]
        pub id: u64,
        pub title: String,
        pub content: String,
        #[secondary_key]
        pub author_id: u64,
        #[secondary_key]
        pub published: bool,
        pub created_at: u64,
    }
}

use models::*;

struct BlogService {
    store: NetabaseStore<BlogDB, sled::Db>,
}

impl BlogService {
    pub fn new(path: &str) -> anyhow::Result<Self> {
        let store = NetabaseStore::<BlogDB, _>::sled(path)?;
        Ok(Self { store })
    }

    /// Create a new user
    pub fn create_user(&self, username: String, email: String) -> anyhow::Result<User> {
        let user_tree = self.store.open_tree::<User>();

        // Check if email already exists
        let existing = user_tree.get_by_secondary_key(
            UserSecondaryKeys::Email(UserEmailSecondaryKey(email.clone()))
        )?;

        if !existing.is_empty() {
            anyhow::bail!("Email already registered");
        }

        let user = User {
            id: self.next_user_id()?,
            username,
            email,
            created_at: current_timestamp(),
        };

        user_tree.put(user.clone())?;
        Ok(user)
    }

    /// Create a new post for a user
    pub fn create_post(
        &self,
        author_id: u64,
        title: String,
        content: String
    ) -> anyhow::Result<Post> {
        // Verify user exists
        let user_tree = self.store.open_tree::<User>();
        if user_tree.get(UserPrimaryKey(author_id))?.is_none() {
            anyhow::bail!("User not found");
        }

        let post_tree = self.store.open_tree::<Post>();
        let post = Post {
            id: self.next_post_id()?,
            title,
            content,
            author_id,
            published: false,
            created_at: current_timestamp(),
        };

        post_tree.put(post.clone())?;
        Ok(post)
    }

    /// Get all published posts by a user
    pub fn get_user_posts(&self, user_id: u64) -> anyhow::Result<Vec<Post>> {
        let post_tree = self.store.open_tree::<Post>();

        let all_posts = post_tree.get_by_secondary_key(
            PostSecondaryKeys::AuthorId(PostAuthorIdSecondaryKey(user_id))
        )?;

        // Filter for published posts
        let published: Vec<Post> = all_posts
            .into_iter()
            .filter(|p| p.published)
            .collect();

        Ok(published)
    }

    /// Publish a post (atomic update)
    pub fn publish_post(&self, post_id: u64) -> anyhow::Result<()> {
        let post_tree = self.store.open_tree::<Post>();

        let mut post = post_tree.get(PostPrimaryKey(post_id))?
            .ok_or_else(|| anyhow::anyhow!("Post not found"))?;

        post.published = true;
        post_tree.put(post)?;

        Ok(())
    }

    /// Get all published posts (for homepage)
    pub fn get_all_published_posts(&self) -> anyhow::Result<Vec<Post>> {
        let post_tree = self.store.open_tree::<Post>();

        post_tree.get_by_secondary_key(
            PostSecondaryKeys::Published(PostPublishedSecondaryKey(true))
        )
    }

    /// Helper: Generate next user ID
    fn next_user_id(&self) -> anyhow::Result<u64> {
        let user_tree = self.store.open_tree::<User>();
        Ok(user_tree.len() as u64 + 1)
    }

    /// Helper: Generate next post ID
    fn next_post_id(&self) -> anyhow::Result<u64> {
        let post_tree = self.store.open_tree::<Post>();
        Ok(post_tree.len() as u64 + 1)
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn main() -> anyhow::Result<()> {
    let blog = BlogService::new("./blog.db")?;

    // Create users
    let alice = blog.create_user("alice".into(), "alice@example.com".into())?;
    let bob = blog.create_user("bob".into(), "bob@example.com".into())?;

    println!("✓ Created users: {} and {}", alice.username, bob.username);

    // Create posts
    let post1 = blog.create_post(
        alice.id,
        "My First Post".into(),
        "Hello, world!".into()
    )?;

    let post2 = blog.create_post(
        alice.id,
        "Second Post".into(),
        "More content here.".into()
    )?;

    println!("✓ Created {} posts for {}", 2, alice.username);

    // Publish posts
    blog.publish_post(post1.id)?;
    blog.publish_post(post2.id)?;

    println!("✓ Published posts");

    // Query published posts
    let alice_posts = blog.get_user_posts(alice.id)?;
    println!("✓ Alice has {} published posts", alice_posts.len());

    let all_published = blog.get_all_published_posts()?;
    println!("✓ Total published posts: {}", all_published.len());

    Ok(())
}
```

## Performance Tips

Use batches for bulk operations, since they're 50 to 100x faster than individual inserts. Use transactions for related operations, which gets you atomicity and speed at the same time. Consider the zero-copy API on hot paths for another 5 to 10x on Redb. Only index the fields you'll actually query, because secondary keys are fast but not free. And benchmark your own workload, because the characteristics vary a lot by use case.

## Common Patterns

### Pagination

```rust
fn get_users_page(
    tree: &impl NetabaseTreeSync<'_, AppDefinition, User>,
    page: usize,
    page_size: usize
) -> anyhow::Result<Vec<User>> {
    let skip = page * page_size;
    let mut results = Vec::new();

    for (i, result) in tree.iter().enumerate() {
        if i < skip {
            continue;
        }
        if i >= skip + page_size {
            break;
        }
        let (_, user) = result?;
        results.push(user);
    }

    Ok(results)
}
```

### Conditional Updates

```rust
fn update_user_if_exists(
    tree: &impl NetabaseTreeSync<'_, AppDefinition, User>,
    user_id: u64,
    new_email: String
) -> anyhow::Result<bool> {
    match tree.get(UserPrimaryKey(user_id))? {
        Some(mut user) => {
            user.email = new_email;
            tree.put(user)?;
            Ok(true)
        }
        None => Ok(false),
    }
}
```

### Bulk Delete

```rust
fn delete_inactive_users(
    tree: &impl NetabaseTreeSync<'_, AppDefinition, User>,
) -> anyhow::Result<usize> {
    let users = tree.get_by_secondary_key(
        UserSecondaryKeys::Active(UserActiveSecondaryKey(false))
    )?;

    let count = users.len();
    for user in users {
        tree.remove(user.primary_key())?;
    }

    Ok(count)
}
```

## Troubleshooting

### "No method named `put` found"

You're trying to write through a read-only transaction. Change `store.read()` to `store.write()`.

### "Primary key already exists"

Use `put()` to update existing records. It's idempotent and will replace the old record.

### Performance issues with many inserts

Use batch operations or transactions to group multiple operations together.

### Secondary key queries returning empty results

Make sure the secondary key type matches exactly, including any wrapper types.

## Where to Go Next

The API docs live at [docs.rs/netabase_store](https://docs.rs/netabase_store), and there are full examples in the [GitHub repository](https://github.com/newsnet-africa/netabase_store/tree/main/examples). Issues and contributions are welcome there too. If you want to know how any of this works underneath, the architecture series goes through the internals.

The summary is that you get a type-safe abstraction over embedded databases without paying for it in performance. The macros handle the boilerplate, the compiler handles the safety, and the unified API means your code isn't tied to a particular backend.

Start with the tree-based API. Move to transactions and then zero-copy if and when your workload needs it.

---

**Project Links:**
- [GitHub Repository](https://github.com/newsnet-africa/netabase_store)
- [Documentation](https://docs.rs/netabase_store)
- [Crates.io](https://crates.io/crates/netabase_store)
