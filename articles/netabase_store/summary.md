#####
short_summary = "A series on building a database abstraction library in Rust: procedural macros, trait-based backend design, the type-state pattern, and zero-copy optimizations, worked through on a real library that runs on Sled, Redb and IndexedDB."
name = "summary"
#####

# netabase_store: a detour

For the last few months I've been building an application called Netabase. It uses the browser's IndexedDB for client-side storage, wrapped in a library I wrote called netabase_store, and it synchronizes with a backend over REST. It has grown into a large enough ecosystem that explaining how the whole thing fits together tends to overwhelm people.

This series is my attempt to fix that.

Over the next several articles I'll walk through building netabase_store: how it started, how it changed, what it solves now, and the decisions that got it there. I'm less interested in documenting the code than in explaining why each choice was made, including the constraints and the failures that led to it.

Each article covers one piece of the system, so you can follow the progression without juggling every detail at once. Along the way I cover how the library talks to IndexedDB, how data moves between browser and backend, what sits on top of the raw storage layer, and how those choices affect performance and usability.

If you've worked with IndexedDB you already know it's awkward and inconsistent. That's part of why I built this, and it turned into a chance to think properly about client-side persistence. If you're building something that needs offline storage, or you're curious about writing your own storage abstraction, there should be something useful here.
