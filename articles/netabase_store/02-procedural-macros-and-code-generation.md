#####
date = "2025-11-20"
author = "Nzuzo Magagula"
summary = "Parsing Rust syntax trees and generating type-safe database code at compile time, and how the macro crate got restructured three times to get there"
thumbnail = "https://i.postimg.cc/d1ZSWs9W/54a1b049-09d1-4d4b-82fd-2c620fbccc0c.jpg"
category = "Technical"
show_references = true

[[article_series]]
name = "Building netabase_store"
prev = "netabase_store/01-introduction-and-overview"
next = "netabase_store/03-backend-implementation-and-trait-design"

[[references]]
title = "Procedural Macros - The Rust Book"
url = "https://doc.rust-lang.org/book/ch19-06-macros.html"
description = "Official guide to macros in Rust"

[[references]]
title = "syn Crate Documentation"
url = "https://docs.rs/syn/latest/syn/"
description = "Library for parsing Rust syntax trees"

[[references]]
title = "quote Crate Documentation"
url = "https://docs.rs/quote/latest/quote/"
description = "Quasi-quoting for Rust code generation"

[[references]]
title = "Procedural Macros Reference"
url = "https://doc.rust-lang.org/reference/procedural-macros.html"
description = "Comprehensive reference for procedural macros"

[[references]]
title = "Abstract Syntax Tree (AST)"
url = "https://en.wikipedia.org/wiki/Abstract_syntax_tree"
description = "Understanding syntax trees in compiler design"
#####
# The Macro System

This article covers the two macros that power netabase_store: the derive macro (`#[derive(NetabaseModel)]`) and the attribute macro (`#[netabase_definition_module(Definition, DefinitionKeys)]`). Between them they traverse the [Rust syntax tree][1], build model metadata, and emit the structures and traits the runtime uses.

One distinction runs through all of this. Meta-logic is code that runs at compile time inside the macro. Runtime logic is the code the macro generates, which your application then uses. That difference is subtle and confusing at first, so I'll flag it whenever it matters.

## Overview

The macros form a two-stage compilation system for your data model.

The [derive macros][2] operate on a single struct. They parse the input [AST][1], visit each struct, field and attribute, extract the model metadata (keys, fields, discriminants), and generate strongly typed Rust code from it: newtypes, [traits][3] and impls.

The attribute macro acts as a linker. It takes the models already defined and compiles them into a single database definition module, generating discriminants, static descriptors and the public database API.

Conceptually it's a mini-compiler for your data model layer. The macros don't just automate small tasks; they generate an entire set of typed structures and database-level identifiers.

## Macro Crate Structure

Macros are powerful because they're flexible, and that flexibility makes them hard to manage. When I started I couldn't find much in the way of established structure or common practice for a macro library that needed to do what I wanted. Honestly I wasn't sure what exactly I wanted from my macros either, and this ended up being my favourite trial-and-error process of the whole project.

### One giant function

I started by defining the macro and all its functionality in a single function, which became a headache almost immediately:

```rust
// First attempt: Everything in one giant function
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // Parse attributes manually
    let mut primary_key = None;
    let mut secondary_keys = Vec::new();
    
    if let syn::Data::Struct(data) = &input.data {
        for field in &data.fields {
            for attr in &field.attrs {
                if attr.path().is_ident("primary_key") {
                    if primary_key.is_some() {
                        panic!("Multiple primary keys found");
                    }
                    primary_key = Some(field.ident.as_ref().unwrap());
                } else if attr.path().is_ident("secondary_key") {
                    secondary_keys.push(field.ident.as_ref().unwrap());
                }
            }
        }
    }
    
    let primary_key = primary_key.expect("No primary key found");
    
    // Generate primary key newtype
    let primary_key_name = format!("{}PrimaryKey", input.ident);
    let primary_key_ident = Ident::new(&primary_key_name, input.ident.span());
    
    // Generate secondary key types and enum
    let secondary_key_types = secondary_keys.iter().map(|key| {
        let type_name = format!("{}{}SecondaryKey", input.ident, key);
        Ident::new(&type_name, input.ident.span())
    });
    
    let secondary_key_variants = secondary_keys.iter().map(|key| {
        let variant_name = format!("{}", key);
        Ident::new(&variant_name, input.ident.span())
    });
    
    // Generate trait implementation
    let model_name = &input.ident;
    
    // ... and 100+ more lines of quote!{} spaghetti
}
```

You can see how painful updating this would be for every feature I wanted to add. The problem was the obvious one: when everything is in the same function, you can't see what isn't working. Which is more or less the first rule of good design.

The practical consequence was error messages. The compiler would tell me the macro was the problem, and I'd have to walk through the logic piece by piece to find out what was actually wrong. That's especially annoying because if your macros are wrong enough, the code doesn't expand at all.

### Breaking out helpers

My first improvement was extracting helper functions for different generation tasks. That made the code more readable without solving the underlying problem:

```rust
fn generate_primary_key(struct_name: &Ident, field: &Field) -> TokenStream2 {
    let key_name = format!("{}PrimaryKey", struct_name);
    let key_ident = Ident::new(&key_name, struct_name.span());
    let field_type = &field.ty;
    
    quote! {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode)]
        pub struct #key_ident(pub #field_type);
    }
}

fn generate_secondary_keys(struct_name: &Ident, fields: &[Field]) -> Vec<TokenStream2> {
    fields.iter().filter_map(|field| {
        if has_attribute(field, "secondary_key") {
            let field_name = field.ident.as_ref().unwrap();
            let key_name = format!("{}{}SecondaryKey", struct_name, field_name);
            let key_ident = Ident::new(&key_name, struct_name.span());
            let field_type = &field.ty;
            
            Some(quote! {
                #[derive(Debug, Clone, PartialEq, Eq, Hash, bincode::Encode, bincode::Decode)]
                pub struct #key_ident(pub #field_type);
            })
        } else {
            None
        }
    }).collect()
}

// The main function became cleaner but still mixed parsing and generation
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let primary_key_field = find_primary_key(&input).expect("No primary key found");
    let secondary_key_fields = find_secondary_keys(&input);
    
    let primary_key = generate_primary_key(&input.ident, &primary_key_field);
    let secondary_keys = generate_secondary_keys(&input.ident, &secondary_key_fields);
    let keys_enum = generate_keys_enum(&input.ident, &secondary_key_fields);
    let trait_impl = generate_trait_impl(&input.ident, &primary_key_field, &secondary_key_fields);
    
    quote! {
        #primary_key
        #(#secondary_keys)*
        #keys_enum
        #trait_impl
    }.into()
}
```

Better, but I still couldn't test individual components or get useful error messages. When something broke I was guessing which helper caused it.

### Real error messages

Things improved when I started emitting meaningful errors. Instead of panicking with "Multiple primary keys found," I needed to tell the user what went wrong and where:

```rust
fn validate_model(input: &DeriveInput) -> Result<(), syn::Error> {
    let mut primary_keys = Vec::new();
    let mut errors = Vec::new();
    
    if let syn::Data::Struct(data) = &input.data {
        for field in &data.fields {
            for attr in &field.attrs {
                if attr.path().is_ident("primary_key") {
                    primary_keys.push(field);
                }
            }
        }
    }
    
    if primary_keys.is_empty() {
        errors.push(syn::Error::new_spanned(
            input,
            "Model must have exactly one #[primary_key] field"
        ));
    } else if primary_keys.len() > 1 {
        for key in primary_keys {
            errors.push(syn::Error::new_spanned(
                key,
                "Multiple #[primary_key] fields found. Only one primary key is allowed per model."
            ));
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into_iter().reduce(|mut a, b| {
            a.combine(b);
            a
        }).unwrap())
    }
}
```

Users got clear errors, but the code was still hard to maintain. Validation logic was scattered across several functions, and adding a feature meant touching most of them.

### Separating visitors from generators

What finally worked was separating the parsing and visiting logic from the code generation logic entirely:

```rust
// Clean separation in the main derive function
#[proc_macro_derive(NetabaseModel, attributes(primary_key, secondary_key, link))]
pub fn netabase_model_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    // VISITOR: Extract metadata from AST
    let mut visitor = ModelVisitor::default();
    visitor.visit_derive_input(&input);
    
    // GENERATORS: Create code from metadata
    let (p, sl, s, k) = visitor.generate_keys();
    let trait_impl = visitor.generate_model_trait_impl();
    let borrow_impls = visitor.generate_borrow_impls();
    let extension_traits = visitor.generate_key_extension_traits();

    quote! {
        #p        // Primary key
        #(#sl)*   // Secondary key list  
        #s        // Secondary keys enum
        #k        // Combined keys enum
        #(#trait_impl)*
        #(#borrow_impls)*
        #(#extension_traits)*
    }.into()
}
```

That bought me four things. I can test `ModelVisitor` on its own to confirm it extracts the right metadata. The generation logic is organized by concern: keys, traits, borrow impls. When something breaks I know which component to check. And adding a feature means extending the visitor or adding a generator, not rewriting everything.

The same pattern applies to `netabase_definition_module`:

```rust
#[proc_macro_attribute]
pub fn netabase_definition_module(name: TokenStream, input: TokenStream) -> TokenStream {
    let mut def_module = parse_macro_input!(input as ItemMod);
    
    // VISITOR: Extract module structure and model information
    let mut visitor = DefinitionsVisitor::default();
    visitor.visit_item_mod(&def_module);
    
    // GENERATORS: Create definition enums, trait impls, etc.
    let (defin, def_key) = visitor.generate_definitions(definition, definition_key);
    let tables_struct = generators::table_definitions::generate_tables_struct(&visitor.modules, definition);
    let trait_impls = visitor.generate_definition_trait_impls(definition, definition_key, &tables_name);
    
    // ... more generators
}
```

Visitors understand the structure of the code, generators emit new code based on that understanding. Each evolves independently, and problems can be pinpointed.

## Stage 1: The Derive Macros

### How `syn` lets you climb the AST

The [`syn`][5] crate is the foundation of procedural macros in Rust. It parses, traverses and makes sense of Rust code at compile time.

An [AST][1] is a tree representation of your code's structure. When you write:

```rust
#[derive(NetabaseModel)]
pub struct User {
    #[primary_key]
    pub id: u64,
    #[secondary_key] 
    pub email: String,
}
```

the compiler parses it into something conceptually like:

```
DeriveInput
├── attributes: ["NetabaseModel"]
├── vis: "pub"  
├── ident: "User"
└── Data::Struct
    └── Fields::Named
        ├── Field
        │   ├── attributes: ["primary_key"]
        │   ├── vis: "pub"
        │   ├── ident: "id"
        │   └── ty: "u64"
        └── Field
            ├── attributes: ["secondary_key"]
            ├── vis: "pub" 
            ├── ident: "email"
            └── ty: "String"
```

A procedural macro receives a `TokenStream`, which is a flat sequence of tokens with no structure. `syn` turns that into something you can work with:

```rust
// TokenStream (flat, hard to work with):
// #[ derive ( NetabaseModel ) ] pub struct User { # [ primary_key ] pub id : u64 , ... }

// syn parses this into a structured DeriveInput (easy to work with):
let input = parse_macro_input!(input as DeriveInput);
// Now we have a structured object with fields, attributes, etc.
```

### The visitor pattern

The [`syn::visit::Visit`][7] trait provides methods for every node type in the AST, so you can walk the tree systematically:

```rust
impl<'a> Visit<'a> for ModelVisitor<'a> {
    fn visit_derive_input(&mut self, node: &'a DeriveInput) {
        // We've reached a struct/enum definition - our main target
        println!("Visiting struct: {}", node.ident);
        syn::visit::visit_derive_input(self, node); // Continue climbing
    }
    
    fn visit_field(&mut self, node: &'a Field) {
        // We're now looking at an individual field
        println!("Found field: {:?}", node.ident);
        syn::visit::visit_field(self, node); // Continue to attributes, type, etc.
    }
    
    fn visit_attribute(&mut self, node: &'a Attribute) {
        // We're examining an attribute like #[primary_key]
        if node.path().is_ident("primary_key") {
            println!("Found primary key attribute!");
        }
        syn::visit::visit_attribute(self, node);
    }
}
```

`syn` gives you two ways to do this. You can navigate manually, accessing exactly what you need:

```rust
fn visit_derive_input(&mut self, i: &'a DeriveInput) {
    // Directly access what we need
    self.name = Some(&i.ident);                    // Struct name
    self.definitions = Self::find_definitions(i);  // Custom attributes
    
    if let syn::Data::Struct(data) = &i.data {     // Drill into struct body
        for field in &data.fields {                // Examine each field
            // Process field attributes, types, etc.
        }
    }
}
```

Or you can let the [visitor pattern][6] traverse for you:

```rust
fn visit_field(&mut self, field: &'a Field) {
    // This gets called automatically for every field
    // The visitor pattern walks the entire tree for us
}
```

netabase_store uses a hybrid: implement `Visit`, override the specific methods that matter, and navigate manually inside those for precise extraction.

Walking through what happens with our example, the raw tokens become a `DeriveInput`, `visit_derive_input` gets called with the complete struct, and we extract the struct name `User`, the definition `BlogDefinition` from the `#[netabase]` attribute, and each field with its attributes. For each field we check for `#[primary_key]` and `#[secondary_key]`, then examine the field types (`u64`, `String`) to generate the right newtypes.

The whole pipeline looks like this:

```
Rust Source Code
    → TokenStream (raw tokens)
    → syn::DeriveInput (structured AST)
    → ModelVisitor (extract metadata)
    → Code Generators (emit new code using quote)
    → TokenStream (generated code)
    → Expanded Rust Code
```

Understanding this matters because it's what lets you target only the structures you care about, understand the relationships between attributes, fields and types, validate input before generating anything, and support the more complex parts of Rust like [generics][9] and [lifetimes][10].

### The visitor itself

The easiest way I found to think about the visitor is as a census worker. The neighbourhood is the syntax tree, each house is a node, and the paperwork the census worker fills in is the metadata: primary key, secondary keys, field attributes. The worker visits every house, records facts, alters nothing, and hands the collected data to the code generator.

```rust
use syn::{Ident, Path, Token, punctuated::Punctuated, visit::Visit};

use crate::{
    item_info::netabase_model::{ModelKeyInfo, ModelLinkInfo},
    util::extract_fields,
};

// Holds all the metadata collected during AST traversal.
#[derive(Default)]
pub struct ModelVisitor<'ast> {
    pub name: Option<&'ast Ident>,           // Which struct we're visiting
    pub key: Option<ModelKeyInfo<'ast>>,     // Primary and secondary key information
    pub links: Vec<ModelLinkInfo<'ast>>,     // Foreign key relationships (future links)
    pub definitions: Vec<Path>,              // Which database definition this model belongs to
    // Generics support removed - not yet implemented
    // pub generics: Option<&'ast Generics>,
}

impl<'a> Visit<'a> for ModelVisitor<'a> {
    fn visit_derive_input(&mut self, i: &'a syn::DeriveInput) {
        self.name = Some(&i.ident);
        
        // Generics support removed - not yet implemented
        // self.generics = Some(&i.generics);
        
        // Identify the primary and secondary keys among the fields
        self.key = match ModelKeyInfo::find_keys(extract_fields(i)) {
            Ok(k) => Some(k),
            Err(e) => panic!("Error parsing Model: {e}"),
        };
        
        self.definitions = Self::find_definitions(i);
        self.links = ModelLinkInfo::find_link(extract_fields(i)).collect();
    }
}

impl<'a> ModelVisitor<'a> {
    // Look for the `#[netabase]` attribute to determine which database
    // definition the model belongs to.
    pub fn find_definitions(input: &'a syn::DeriveInput) -> Vec<syn::Path> {
        let attr = input.attrs.iter().find(|a| a.path().is_ident("netabase"));
        
        if let Some(att) = attr
            && let Ok(list) = att.meta.require_list()
        {
            match list
                .parse_args_with(Punctuated::<syn::Path, Token![,]>::parse_terminated)
                .map_err(|e| e.into_compile_error())
            {
                Ok(r) => r.into_iter().collect(),
                Err(_) => vec![],
            }
        } else {
            vec![]
        }
    }
}
```

## Validating Model Metadata

Once the visitor has walked the AST we have a `ModelMeta` containing the model's name, fields, primary key, secondary keys and other attributes. A few assertions happen here, in the meta-logic layer.

**Exactly one primary key must exist.** It uniquely identifies entities, defines the storage layout, and determines which newtype gets generated. Zero or multiple primary keys would break the contract the backends rely on.

**All secondary keys must be newtypes.** They can't reuse primitives like `String` or `u32`, because they participate in the model's `SecondaryKey` enum and backends have to be able to tell them apart by type.

**Discriminants are required.** Each model needs a unique discriminant so the backend can tag rows by model type, separate index namespaces, and avoid table collisions. The definition macro generates the `ModelDiscriminant` enum from these.

**Generated conversion traits must be valid.** The `TryFrom<Enum>` implementations are what allow safe extraction of typed keys from the model key enums, which prevents type-mixing mistakes in a strongly typed backend.

Once that's validated, the derive macro generates the code: newtypes for primary and secondary keys, implementations of `Borrow`, `From`, `TryFrom` and `AsRef`, the model's key enums and `Descriptor` struct, and the backend-facing traits and helper methods for constructing keys. That generated code is the runtime logic your program and the backends actually use.

## Stage 2: The Definition Module Macro

Where the derive macros work per-model, `#[netabase_definition_module]` transforms a module containing multiple model definitions into a single database schema.

Derive macros go from struct to metadata to expansions. This one handles database-wide model registration, discriminant generation, wrapper enums for type-safe queries, and the public database API surface. It's what turns a module of structs into a complete typed schema.

The input is a module of annotated models:

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

    #[derive(NetabaseModel, Clone, Debug,
             bincode::Encode, bincode::Decode)]
    #[netabase(BlogDefinition)]
    pub struct Post {
        #[primary_key]
        pub id: u64,
        pub title: String,
        pub author_id: u64,
    }
}
```

The macro inspects the module AST, collects every model marked with `#[netabase(BlogDefinition)]`, and generates a `BlogDefinition` enum wrapping all model types for type-safe storage and retrieval, a `BlogDefinitionDiscriminant` enum (`#[repr(u16)]`) used as database-level namespace identifiers, a `BlogKeys` enum wrapping every possible key type across all models, a `BlogDefinitionTables` struct for Redb's compile-time table name validation, and the `NetabaseDefinitionTrait` implementation providing metadata access and conversions.

### Why the discriminant matters

The `BlogDefinitionDiscriminant` is a `#[repr(u16)]` enum with a variant per model. It acts as a database-level namespace identifier, and the backends use it to separate tables by model type, create isolated secondary index namespaces, encode composite keys for secondary indexes, and route typed queries to the right storage region.

That's what makes the backend code completely generic. It works with any schema a user defines.

The macro also enforces a few things at compile time: the module must contain at least one model, all listed models must implement `NetabaseModelTrait<BlogDefinition>`, each model receives a unique discriminant, and keys stay scoped to their parent definition through the wrapper enums.

## From Input to Generated Code

Here's a complete example traced through the system. The input:

```rust
#[netabase_definition_module(ExampleDefs, ExampleDefKeys)]
pub mod definitions {
    use netabase_store::{NetabaseModel, netabase};

    #[derive(NetabaseModel, bincode::Encode, bincode::Decode, Clone, Debug)]
    #[netabase(ExampleDefs)]
    pub struct User {
        #[primary_key]
        pub name: String,
        pub age: u8,
        #[secondary_key]
        pub email: String,
    }
}
```

The key types that come out:

```rust
// PRIMARY KEY NEWTYPE: Generated from #[primary_key] field
pub struct UserPrimaryKey(pub String);

// SECONDARY KEY NEWTYPE: Generated from #[secondary_key] field  
pub struct UserEmailSecondaryKey(pub String);

// SECONDARY KEYS ENUM: Unifies all secondary keys for this model
pub enum UserSecondaryKeys {
    Email(UserEmailSecondaryKey),
}

// COMBINED KEYS ENUM: Unifies primary and secondary keys
pub enum UserKey {
    Primary(UserPrimaryKey),
    Secondary(UserSecondaryKeys),
}
```

And the schema structures:

```rust
// DATABASE DEFINITION ENUM: Can hold any model in the schema
pub enum ExampleDefs {
    User(User),
    // Additional models would be added here as the schema grows
}

// DATABASE KEYS ENUM: Can hold any key from any model in the schema
pub enum ExampleDefKeys {
    UserKey(UserKey),
    // PostKey(PostKey),      // Additional model keys would appear here
    // CommentKey(CommentKey),
}

// DISCRIMINANT TYPES: For type-safe model identification
pub enum ExampleDefsDiscriminant { 
    User 
}

pub enum ExampleDefKeysDiscriminant { 
    UserKey 
}
```

### How each piece gets generated

The visitor detects `#[primary_key]` and `#[secondary_key]` and generates the corresponding newtypes:

```rust
// Input field:
#[primary_key]
pub name: String,           // Field name: "name", type: String

// Generated newtype:
pub struct UserPrimaryKey(pub String);  // Name: User + PrimaryKey, wraps String

// Input field:  
#[secondary_key]
pub email: String,          // Field name: "email", type: String

// Generated newtype:
pub struct UserEmailSecondaryKey(pub String);  // Name: User + Email + SecondaryKey
```

These newtypes are what give you compile-time safety. You can't accidentally pass a `UserPrimaryKey` where a `PostPrimaryKey` is expected, even though both wrap a `String`.

Then the enums unify all keys for ergonomic use:

```rust
// Generated from all #[secondary_key] fields
pub enum UserSecondaryKeys {
    Email(UserEmailSecondaryKey),  // One variant per secondary key
    // Age(UserAgeSecondaryKey),   // Additional keys would appear here
}

// Generated to combine primary and secondary keys  
pub enum UserKey {
    Primary(UserPrimaryKey),       // Primary key variant
    Secondary(UserSecondaryKeys),  // All secondary keys variant
}
```

That lets you work with any key type through one interface while keeping the full type information.

The definition module macro then links the models into a schema:

```rust
// Generated definition enum (holds any model in the module)
pub enum ExampleDefs {
    User(User),           // Your original User struct
    // Post(Post),       // Additional models would be variants
    // Comment(Comment),
}

// Generated keys enum (holds any key from any model)  
pub enum ExampleDefKeys {
    UserKey(UserKey),     // All keys from User model
    // PostKey(PostKey),   // All keys from Post model  
    // CommentKey(CommentKey),
}
```

This is the unified interface backends use to work with entire schemas generically while keeping type safety across models.

Finally the discriminants:

```rust
// Simple enums used for efficient type tagging
pub enum ExampleDefsDiscriminant { User }
pub enum ExampleDefKeysDiscriminant { UserKey }
```

These let backends route operations to the right storage without runtime type checking.

So the flow is: `ModelVisitor` finds `#[primary_key]` on `name: String` and `#[secondary_key]` on `email: String`, records that metadata, the generators create `UserPrimaryKey`, `UserEmailSecondaryKey`, `UserSecondaryKeys` and `UserKey`, and then `DefinitionsVisitor` collects all models and generates `ExampleDefs` and `ExampleDefKeys`.

Nine lines of user input produce eight type definitions that form a complete typed database interface. Each has a specific role, the naming follows consistent patterns (`{Model}PrimaryKey`, `{Model}{Field}SecondaryKey`), and every relationship is enforced by the compiler.

You aren't limited to types either. You can add trait definitions, extra modules, or anything else the system needs.

## Meta-Logic and Runtime Logic

This project is a good argument for learning to separate meta and runtime thinking, because the two layers never interact directly, only through generated code.

Meta-logic runs during compilation. It reads your code or a model list, generates more code, and contains the visitors, attribute parsing, assertions and error messages.

Runtime logic runs when your application executes. It defines how models behave and how keys get constructed, and it's what lets backends perform queries using descriptors and discriminants.

The mental model I use: meta-logic is the compiler writing Rust for you, runtime logic is your program running it.

## The Full Pipeline

Putting it together, the macro system is a staged compilation pipeline.

A visitor walks the AST of a single model annotated with `#[derive(NetabaseModel)]`. Metadata gets extracted and compile-time assertions check correctness. The derive macro emits runtime structures, trait implementations and `Borrow` impls for that model. The `#[netabase_definition_module]` attribute macro then traverses the module, collects all models, and generates the wrapper enums, static table definitions and schema trait implementations. Backends consume the generated schema through the trait APIs, using `Borrow` for zero-copy access and discriminants for namespace separation.

Two stages, with meta-logic and runtime logic kept clearly apart. A future migration to [`darling`][14] should make the attribute parsing stage cleaner still.

## References

[1]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[2]: https://doc.rust-lang.org/reference/procedural-macros.html
[3]: https://doc.rust-lang.org/book/ch10-02-traits.html
[4]: https://docs.rs/bincode/
[5]: https://docs.rs/syn/
[6]: https://en.wikipedia.org/wiki/Visitor_pattern
[7]: https://docs.rs/syn/latest/syn/visit/trait.Visit.html
[8]: https://doc.rust-lang.org/proc_macro/struct.TokenStream.html
[9]: https://doc.rust-lang.org/book/ch10-01-syntax.html
[10]: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
[11]: https://docs.rs/quote/
[12]: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
[13]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[14]: https://docs.rs/darling/
