#####
date = "2025-12-12"
author = "Nzuzo Magagula"
summary = "What Rust actually promises, what it genuinely costs, and why the discourse after the outage argued with a version of Rust that does not exist"
thumbnail = "https://i.postimg.cc/3wMCP1N0/cracked-white-plaster-wall-texture-background.jpg"
category = "Opinion"
show_references = true

[[article_series]]
name = "Cloudflare Outage & Infrastructure Fragility"
prev = "Cloudflare Outage/01-centralization-and-infrastructure"
#####
# Rust, Reality, and the Strange State of Language Discourse

Programming languages rarely inspire neutrality. They become symbols: of ideals, of identity, of "the right way to build software." Nowhere has that been clearer in the last decade than with Rust. The language's rise, its claims, and its constraints have created an environment where people don't debate technical tradeoffs so much as defend worldviews.

Before getting into specifics, here is roughly what I want to cover.

Rust does not promise perfection. It promises a specific kind of safety, mostly memory safety, enforced at compile time rather than through runtime checks or manual discipline. Over time that technical promise turned into a cultural narrative: that Rust eliminates entire categories of bugs everywhere, for everyone, effortlessly. Rust does have real costs, but they are rarely the ones people complain about, so the discourse ends up arguing with strawmen. Switching languages is expensive in tooling, training, patterns and mental models, and a lot of anti-Rust takes come from underestimating that. And underneath all of it is tribalism, which smothers nuance, keeps misinformation alive, and turns criticism into betrayal.

I should also say up front: I like Rust a lot. This article is not neutral. I'm just tired of the same hollow arguments looping forever.

## The Strange Ride Rust Has Had

Rust's trajectory has been unusually volatile. When I first encountered it, before I'd ever touched C++, two things stood out immediately. Anything at all can become controversial. And developers really don't like change.

Those turned out to be related but distinct. What I originally read as simple resistance was tribalism: people identify heavily with the tools they invest their time in, and Rust happens to sit in a domain where that identification runs deep.

That lens makes the more unhinged parts of the discourse after the Cloudflare outage make a kind of sense. People didn't want to talk about the bug. They wanted the bug to confirm what they already believed.

The argument I keep running into is some version of:

> "If Rust is so perfect, why can X still happen?"

Charitably paraphrased, it goes: Rust promises memory safety, memory safety should eliminate certain classes of bugs, therefore anything bad happening in Rust is a betrayal of its promises, and since `unsafe` exists, Rust is lying, so the benefits are fake.

That isn't what Rust claims. It's worth looking at what it actually does.

## What Rust Actually Promises

The safety guarantees are built on a small, fairly boring set of rules, first laid out in *The Book*:

> Each value in Rust has a single owner.
> Only one owner may exist at a time.
> When the owner goes out of scope, the value is dropped.

These prevent use-after-free, double frees, dangling pointers, data races, and aliasing violations.

The ownership rules are augmented by rules about references. You may have one mutable reference, or any number of immutable references, but not both at once. And no reference may outlive the data it points to.

All of this is checked at compile time, which is what makes it eliminate categories of bugs rather than reduce their frequency. That's the promise. Not a bug-free utopia.

### What that looks like in practice

A dangling pointer in C++ compiles fine and is undefined behaviour at runtime:

```cpp
int* ptr;
{
    int x = 5;
    ptr = &x;
}
return *ptr; // UB, but compiles fine
```

Rust stops you before the program runs:

```rust
let r = {
    let x = 5;
    &x
}; // error: borrowed value does not live long enough
```

Aliasing with mutation is the same story. In C++:

```cpp
void break_it(int* a, int* b) {
    *a = 10;
    *b = 20; // may mutate same memory -> UB
}
```

In Rust, the compiler rejects the case where both point at the same memory:

```rust
fn break_it(a: &mut i32, b: &mut i32) { /* ... */ }
```

That is the core of it.

## Why `unsafe` Exists

Rust includes `unsafe` because the hardware is unsafe. You cannot write OS kernels, device drivers, memory allocators, FFI bindings, or custom data structures without direct control over raw pointers.

`unsafe` marks the code where you, not the compiler, take responsibility for upholding the guarantees:

```rust
pub unsafe fn copy(src: *const u8, dst: *mut u8, len: usize) {
    std::ptr::copy_nonoverlapping(src, dst, len);
}
```

Safe Rust cannot do this. Unsafe Rust can, and the point is that it can be isolated, audited and kept small.

## `unwrap()` in Production

During the Cloudflare outage, an error bubbled up into an `unwrap()`, which panicked. Cue the takes about how Rust shouldn't allow bugs like this.

`unwrap` is a deliberate opt-out. It's visible in the source, and it's your responsibility:

```rust
let val = maybe_value.unwrap();
```

If `maybe_value` is `Some(_)`, fine. If it's `None`, panic. Rust forces you to acknowledge that something might fail, and `unwrap` is you saying it won't. Sometimes you're wrong about that.

There are better options when you aren't certain. Propagate the error with `?`:

```rust
fn read_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
```

Match explicitly when you want to convert the error:

```rust
match db.get(key) {
    Ok(v) => v,
    Err(e) => return Err(MyError::DbFailure(e)),
}
```

Use `expect` when you genuinely are asserting an invariant, so the panic message says something useful:

```rust
config.get("api_key")
    .expect("missing key: api_key")
```

And use `todo!()` while scaffolding, so unfinished paths are obvious:

```rust
fn parse_advanced_mode() {
    todo!("Advanced mode parsing unimplemented");
}
```

## Where This Leaves Rust

Rust isn't collapsing and it isn't taking over the world overnight.

It has real costs. Migration and rewrites are expensive, and institutional knowledge doesn't transfer for free. Ignoring that does everyone a disservice, and it's the part of the criticism that deserves more airtime than it gets.

It also has real benefits. Memory safety by construction is not marketing. Rust prevents classes of bugs that have cost the industry an enormous amount of money.

And it has a community that sometimes loses itself in identity. Rust isn't a religion, and neither is C++ or Go. Technical criticism is not an attack, and refusing to acknowledge flaws isn't loyalty so much as insecurity.

People like Rust. Not all of them, and not universally, but enough that the momentum is real and durable. That counts for something.

## The Actual State of Rust

Rust is neither the savior nor the villain it gets portrayed as online. It's an opinionated tool that improves memory safety substantially, costs you something in learnability, forces explicitness where other languages let you stay vague, demands care around error handling, and gets you high performance without giving up correctness.

It is not perfect, and its users need to stop treating it as though it should be. But in an industry that has spent decades shipping preventable memory bugs, the differences matter.

Rust didn't promise perfection. It promised a tradeoff, a hard one, and a worthwhile one for a lot of domains. Like any tradeoff, its value comes from understanding it properly and using it responsibly, not from pretending it's flawless.
