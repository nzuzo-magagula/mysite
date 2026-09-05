#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "How netabase_store started, and why a type-safe multi-backend key-value store in Rust needed procedural macros and trait-based design"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
next = "netabase_store/02-procedural-macros-and-code-generation"

[[references]]
title = "Rust Procedural Macros"
url = "https://doc.rust-lang.org/reference/procedural-macros.html"
description = "Official reference on procedural macros in Rust"

[[references]]
title = "Traits - The Rust Book"
url = "https://doc.rust-lang.org/book/ch10-02-traits.html"
description = "Understanding trait-based abstraction in Rust"

[[references]]
title = "Serialization with Serde"
url = "https://serde.rs/"
description = "Official Serde documentation for serialization/deserialization"

[[references]]
title = "libp2p Documentation"
url = "https://docs.libp2p.io/"
description = "Peer-to-peer networking framework"

[[references]]
title = "Kademlia DHT"
url = "https://en.wikipedia.org/wiki/Kademlia"
description = "Distributed hash table for peer-to-peer networks"
#####
# Building netabase_store: A Type-Safe Multi-Backend Database Abstraction

## Introduction

This series is about `netabase_store`, a type-safe, multi-backend key-value storage library I wrote in Rust. Rather than trying to write something authoritative, I want to show how I actually ended up building it, what confused me, and why the final design looks the way it does.

If you've ever looked at Rust's procedural macros, or wondered how to design a flexible API across different storage backends, my wandering path through this might be useful, or at least entertaining.

## The Problem

When I started working on `NewsNet` I came across [`libp2p`][1]. If you haven't tried it, it's a good way to get into [peer-to-peer networking][2]. What stood out immediately is how open-ended it is: most components give you knobs you can turn in all sorts of directions. That's great while you're still learning how everything fits together, and it also meant I spent a lot of time trying configurations.

The goal was to decentralize as much of `NewsNet` as possible, and that research rabbit hole eventually led me to [Kademlia][3] and libp2p's implementation of it. The discovery logic fascinated me enough on its own to make me want to prototype.

Then I started prototyping and hit problems quickly.

### Only bytes allowed

To plug your own storage backend into libp2p's [DHT][4] you implement their [`RecordStore` trait][5]. That part was fine. The problem is that `RecordStore` only deals with byte arrays. As soon as I needed anything past trivial store-and-fetch-blob behaviour, it got messy.

I wanted to store richer, typed objects, and managing serialization by hand everywhere kept producing confusing edge cases. It didn't scale.

### Constant rituals

libp2p gives you an in-memory `RecordStore` and nothing beyond it. Every time I wanted to try something slightly more advanced I had to strip out fields like `Instant` that couldn't be serialized, convert types, redefine structures, and then repeat the whole ritual for the next experiment.

The networking side had the same problem. Listening to `Behaviour` events and updating state based on them was interesting, but every small experiment meant rewriting the same setup code.

### Two questions

That left me with two questions. How do I stop juggling raw bytes and work with actual typed data? And how do I avoid rewriting the same swarm setup every time I want to try something?

Those two frustrations became the `netabase` ecosystem.

## The Spark

The idea was simple. I wanted a library sitting between me and libp2p, handling the repetitive parts, flattening the byte-level details, and letting me focus on logic.

`netabase_store` is the first step, and it answers the first question: how do you build type-safe abstractions over key-value stores without writing a mountain of boilerplate?

It started with a basic goal. Define models once, and let the library generate the machinery to [serialize and deserialize][6] them, generate typed keys, create [secondary indexes][7], talk to any backend ([sled][8], [redb][9], [IndexedDB][10]), and eventually plug into the libp2p DHT.

And, importantly, not slow anything down. I wanted the abstraction to feel like handwritten Rust rather than something sitting on top adding weight.

## Architecture

Here's how the library ended up structured, top to bottom.

### The macro layer

This is where most of the work happens. Two [procedural macros][11] generate the repetitive type definitions and [traits][12] for each model.

```rust
#[netabase_definition_module(BlogDefinition, BlogKeys)]
pub mod blog {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, Clone, Debug,
             bincode::Encode, bincode::Decode)]
    #[netabase(BlogDefinition)]
    pub struct User {
        #[primary_key]
        pub id: u64,
        pub username: String,
        #[secondary_key]
        pub email: String,
    }
}
```

From that, the macro creates enums, key types and lookup functions I would never want to write by hand.

### The trait layer

I knew early that I wanted the same API across multiple databases, so the traits became the backbone.

```rust
pub trait NetabaseTreeSync<D, M> {
    fn put(&self, model: M) -> Result<(), NetabaseError>;
    fn get(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn remove(&self, key: M::PrimaryKey) -> Result<Option<M>, NetabaseError>;
    fn get_by_secondary_key(&self, key: M::SecondaryKeys)
        -> Result<Vec<M>, NetabaseError>;
}
```

A [WASM][14]-compatible async version mirrors it, mostly so IndexedDB works without hacks.

### The backend layer

Once the traits existed, adding backends became straightforward. Each one only needs to store byte keys and byte values, and the trait layer handles the typed world above it.

```rust
pub struct SledStore<D: NetabaseDefinitionTrait> { … }
pub struct RedbStore<D: NetabaseDefinitionTrait> { … }
pub struct IndexedDBStore<D: NetabaseDefinitionTrait> { … }
```

### The unified API

This is the part I wanted from the start: a simple API that hides the backend differences.

```rust
let store = NetabaseStore::<BlogDefinition, _>::sled("./data")?;

let user_tree = store.open_tree::<User>();
user_tree.put(user)?;
let retrieved = user_tree.get(UserPrimaryKey(1))?;
```

The same code runs against all three backends.

## Things I Learned Along the Way

### Zero-cost abstractions

Rust's compiler is generous when you work with the type system rather than around it. All the macro-generated code boils down to plain, efficient Rust, so the [abstraction stays cheap][15].

### The type-state pattern

This one surprised me. By encoding read-only versus read-write at the [type level][16], I could prevent writes while a read-only transaction is open.

```rust
let txn = store.read();  
let tree = txn.open_tree::<User>();

tree.get(key)?;   // Works
tree.put(user)?;  // Compile error
```

The compiler does the guarding for you.

### Automatic secondary indexing

A big quality-of-life improvement. I didn't want to store extra keys manually for lookups, so the macro generates everything needed when you annotate a field with `#[secondary_key]`.

## Example

```rust
use netabase_store::traits::model::NetabaseModelTrait;
use netabase_store::traits::store_ops::OpenTree;
use netabase_store::{NetabaseStore, netabase_definition_module};

#[netabase_definition_module(ExampleDefs, ExampleDefKeys)]
pub mod definitions {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(
        NetabaseModel,
        bincode::Encode,
        bincode::Decode,
        Clone,
        Debug,
        serde::Serialize,
        serde::Deserialize,
    )]
    #[netabase(ExampleDefs)]
    pub struct User {
        #[primary_key]
        pub name: String,
        pub age: u8,
        #[secondary_key]
        pub email: String,
    }
}

use definitions::*;

fn main() {
    // Use the unified NetabaseStore API with Sled backend
    let store = NetabaseStore::<ExampleDefs, _>::sled(
        tempfile::tempdir()
            .expect("Failed to create temp dir")
            .path(),
    )
    .expect("The store failed to open");

    let user_tree = store.open_tree::<User>();

    let user = User {
        name: "It's You!".to_string(),
        age: 24,
        email: "some@email.com".to_string(),
    };
    let user2 = User {
        name: "It's Me!".to_string(),
        age: 20,
        email: "some@email.com".to_string(),
    };

    let put_result = user_tree.put(user.clone());

    let get_result = user_tree.get(user.primary_key());

    // Query by secondary key using the model-prefixed type
    let get_secondary_result = user_tree.get_by_secondary_key(UserSecondaryKeys::Email(
        UserEmailSecondaryKey("some@email.com".to_string()),
    ));

    println!("Get Result: {get_result:?}");
    println!("Get Secondary Result: {get_secondary_result:?}");

    assert!(put_result.is_ok());
    assert!(get_result.is_ok());

    let put_result = user_tree.put(user2.clone());

    let get_result = user_tree.get(user2.primary_key());

    // Query by secondary key using the model-prefixed type
    let get_secondary_result = user_tree.get_by_secondary_key(UserSecondaryKeys::Email(
        UserEmailSecondaryKey("some@email.com".to_string()),
    ));

    println!("Get Result: {get_result:?}");
    println!("Get Secondary Result: {get_secondary_result:?}");

    assert!(put_result.is_ok());
    assert!(get_result.is_ok());

    println!("\nBasic store operations completed successfully!");
}
```

## What Made This Interesting

The fun part was how many Rust features ended up having to work together: procedural macros, [GATs][17], [phantom types][18], [conditional compilation][19], [zero-copy optimizations][20], and backend-agnostic traits. I didn't plan that. The stack emerged as the project grew.

## Performance

I didn't start with benchmarks in mind, but once things stabilized I tested it and the results were better than I expected. sled is fast by default, redb is very memory-efficient, and the batch operations and zero-copy APIs gave large speedups.

## What's Next

The next article digs into procedural macros, which is the part that intimidated me most when I started. I'll cover how I learned to parse Rust syntax trees, generate enums and trait impls, and structure macro code so it stays maintainable.

`netabase_store` came out of my own frustration with juggling raw bytes and repetitive networking setup, and it turned into a fairly robust type-safe abstraction that works across several storage backends. Sharing the learning process rather than just the outcome is the point here, and hopefully it gives a clearer picture of how a Rust library like this actually evolves.

## References

[1]: https://libp2p.io/
[2]: https://en.wikipedia.org/wiki/Peer-to-peer
[3]: https://en.wikipedia.org/wiki/Kademlia
[4]: https://docs.libp2p.io/concepts/fundamentals/protocols/#distributed-hash-table-dht
[5]: https://docs.rs/libp2p-kad/latest/libp2p_kad/record/store/trait.RecordStore.html
[6]: https://serde.rs/
[7]: https://en.wikipedia.org/wiki/Database_index#Secondary_index
[8]: https://docs.rs/sled/
[9]: https://docs.rs/redb/
[10]: https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
[11]: https://doc.rust-lang.org/reference/procedural-macros.html
[12]: https://doc.rust-lang.org/book/ch10-02-traits.html
[13]: https://docs.rs/bincode/
[14]: https://webassembly.org/
[15]: https://doc.rust-lang.org/book/ch19-06-macros.html#zero-cost-abstractions
[16]: https://cliffle.com/blog/rust-typestate/
[17]: https://blog.rust-lang.org/2022/10/28/gats-stabilization.html
[18]: https://doc.rust-lang.org/nomicon/phantom-data.html
[19]: https://doc.rust-lang.org/reference/conditional-compilation.html
[20]: https://www.youtube.com/watch?v=bSkpMdDe4g4
