#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How should we decide what to do?"
thumbnail = "https://i.postimg.cc/pd1RWNGX/G2CM-BI108-Backlog-Images-Project-mgmt-approach-V1b.png"
category = "Educational"
show_references = true

[[article_series]]
name = "Project Management"
next = "Project Management/02-people"
#####
# Planning and Scope: The Five Levels of Project Design

Before getting into the levels themselves, it's worth noting there are roughly three main methodologies practiced in project management. I won't cover them in detail here, but they're all attempts to model the same underlying thing: the [Software Development Life Cycle (SDLC)][1].

The SDLC is a structured way to describe how software gets created and maintained. It's slightly odd to think about in project management terms, since projects are instances with a start and an end, not cycles. But software has to come from somewhere, and that somewhere follows a repeating pattern of design, implementation, maintenance, planning and analysis.

The methodologies differ in how they interpret and apply those phases. The [Waterfall Model][2] breaks the SDLC into distinct sequential phases. [Incremental Development][3] overlaps them so functionality gets built gradually in versions. Integration and Configuration assembles reusable components into a complete system. This series mostly deals with Waterfall and Incremental.

The levels below should be approached in order, since each one builds on the last.

---

## Level 1: The Project

At the start, the goal is to decide what you're building. That means defining what the thing needs to do, which gives you requirements, and how far you'll go implementing them, which gives you scope.

A requirement like "I need a vehicle that moves" is useless until scope exists. Moving around your neighbourhood? A bicycle. Across the country? A plane.

Scope often matters more than requirements, for two reasons. First, most people don't know exactly what they want. Customers don't know what's possible, developers don't know what's feasible or efficient, and ambiguity occasionally inspires creativity but more often produces rework. Second, scope is what makes estimation possible. Experienced developers learn how long different kinds of work take, and scope gives them a frame for those estimates.

Requirements define what to build. Scope defines how much of it to build.

### What Are Requirements?

Requirements describe the functionality and usability the end user or customer expects. They're the criteria the software's success gets measured against.

They cover features and functionality, meaning what the system must do: register users, process payments, generate reports. They cover data inputs and outputs, including compliance constraints like [GDPR][4] or [HIPAA][5]. They cover user content and interfaces, meaning how information is presented and what the user interacts with. And they cover constraints, which are limits on performance, usability or environment, like "the mobile app must load within 2 seconds on a 3G connection."

Written out, requirements usually look like this:

```text
The system shall allow users to register, log in, and reset passwords via email verification.
The system shall restrict administrative features to users with the 'Admin' role.
The system shall validate input forms and reject incomplete submissions.
All passwords shall be stored using salted SHA-256 hashing[6].
The codebase shall be modularized for reusability and follow company naming conventions.
```

The pattern is "the system shall do X." They describe what must happen, not how or why.

### Why Bother Writing Them Down?

In large organizations, requirements arrive pre-defined. For hobbyists and small teams it's tempting to skip this and keep it all in your head.

Formalizing them is still worth it when you're facing uncertainty. It keeps focus on concrete goals, prevents over-engineering and feature creep, and forces you to identify constraints early. It also aligns understanding across a team, which reduces miscommunication and rework and makes progress measurable.

One nice side effect is that requirements translate directly into tests:

```rust
// Requirement: The system shall reject empty usernames.
fn validate_username(name: &str) -> bool {
    !name.trim().is_empty()
}

#[test]
fn rejects_empty_username() {
    assert!(!validate_username(""));
}
```

Encode requirements as tests and you can continuously verify that the functionality still matches the intent.

### A Rough Checklist

Functional requirements determine whether the software does what it's supposed to: inputs, outputs, hardware, UI.

Non-functional requirements determine how well it does it: performance, latency, security, reliability.

Completeness requirements determine whether the system feels finished: documentation, logging, cleanup.

Correctness requirements make sure the functionality and documentation are accurate and testable: testability, readability, relevance.

---

## What Is the Scope of a Project?

Scope defines how much of the system gets built and under what constraints. It turns abstract requirements into concrete, testable goals, and there are two common ways to describe it.

### Narrative descriptions

A narrative scope reads like a high-level story of the system.

For the requirement "provide clinicians with a consolidated, real-time view of patient physiological data," the narrative might be: "The system displays a patient vitals dashboard showing heart rate, blood pressure, oxygen saturation, and trends over time. Nurses can filter by time range and annotate readings."

That's useful mainly because of what it rules out: storing voice notes, editing medical history, and so on.

If a requirement says "the system should help doctors make informed decisions," a narrative scope might clarify that the system will highlight abnormal values in red and suggest possible diagnoses from stored medical protocols, and that it will not include treatment planning, medication management, or direct patient communication.

### Use cases

A use case is more concrete, describing exactly how and when a user interacts with the system.

Take the requirement "user should be able to report and block missing or lost credit cards." The narrative is "the system allows users to report a missing card and block further transactions." The use case is:

> As a customer who has just realized my card is missing, I can open the mobile app, tap 'Report Lost Card,' and instantly block transactions.
>
> Acceptance criteria:
>
> 1. The 'Report Lost Card' button is visible from the home screen.
> 2. The block takes effect immediately.
> 3. Confirmation appears within 3 seconds.

Use cases are easier to test, communicate and reason about than narratives. Here's another, for "the system should allow teachers to grade student assignments":

> As a teacher who has just finished reviewing a student's essay, I want to assign a grade and provide feedback so the student can improve.
>
> Steps:
>
> 1. Teacher navigates to the 'Assignments' page.
> 2. Teacher selects a specific student's submission.
> 3. Teacher enters a numerical grade (0-100) and optional written feedback.
> 4. Teacher clicks 'Submit Grade.'
> 5. System saves the grade and feedback, and sends a notification to the student.
>
> Acceptance criteria:
>
> 1. Grade must be between 0 and 100.
> 2. Feedback is optional but limited to 500 characters.
> 3. Student receives notification within 10 seconds.
> 4. Grade is immediately visible in the student's grade book.

At that level of specificity a developer knows exactly what to build and a tester knows exactly what to verify.

---

## Determining Feasibility

Once requirements and scope exist, you assess what's actually possible with the resources you have.

People are the obvious one, and the least intuitive. More people does not mean faster progress. Adding developers to a late project can slow it down, which is [Brooks's Law][7]. Work out why a project is lagging before scaling the team, and account for onboarding time, communication overhead, and whether the tasks can even be divided.

Tooling matters too. IDEs, CI pipelines and tracking software can improve productivity a lot, but they carry cost and complexity, so adopt only what genuinely helps. A small team might do fine with GitHub Issues and a Makefile where a larger organization needs [Jira][8], [Jenkins][9], and real monitoring.

Hardware is worth checking early. Make sure development environments meet the project's needs, and learn to do more with less. Mastering command-line tools and lightweight workflows pays for itself. Ask whether developers need fast machines for compilation, or whether you need cloud infrastructure to test at scale.

Reusable components accelerate everything, but allocate time to learn and prototype before committing. A well-established framework like [Django][10] or Axum can save months compared to building from scratch.

Treat feasibility as ongoing rather than a one-time gate, and reevaluate at each milestone. You might discover halfway through that your chosen database doesn't scale the way you assumed, and continuous reassessment is what lets you pivot before that becomes critical.

By the end of this level you should have a view on your programming languages, subsystems and code organization, main classes and their responsibilities, database schema, business rules, UI, resource limits, security and performance requirements, scalability goals, error handling, and dependencies. Without clarity here, every later decision gets harder and more error-prone.

---

## Level 2: Subsystems

At this point you should know which [architectural pattern][11] you're using: layered, hexagonal, microservice-based, whatever fits. Now you work out how the modules interact.

This determines the structure and maintainability of the whole codebase. Poor subsystem design produces tight coupling, and tight coupling makes every change expensive and risky.

### Fan-out

Fan-out measures how many other modules a given module depends on. It's a quantitative measure of interdependence: high fan-out means a brittle architecture, low fan-out means a more modular one.

```rust
// High fan-out example: depends on too many components
fn process_order() {
    update_inventory();
    charge_payment();
    send_confirmation_email();
    log_transaction();
}
```

A refactor could introduce an order service that encapsulates these interactions:

```rust
fn process_order() {
    order_service::process();
}
```

The reason high fan-out hurts is that each of those four functions can change its interface or behaviour independently. `process_order` now has four separate reasons to change, which violates the [Single Responsibility Principle][12] and makes testing painful, since you have to mock all four dependencies to test it in isolation. An intermediate service reduces the direct dependencies and gives you a more stable interface.

### Fan-in

Fan-in measures the opposite: how many other modules depend on a given module. Fan-out asks "how many things do I depend on?" and fan-in asks "how many things depend on me?" Together they describe how interconnected a system is, and a healthy architecture balances the two.

High fan-in means a module is widely reused, so it's probably a core utility or service. That's usually good, but it also means changes ripple. Fan-in is a measure of both importance and risk.

```rust
// High fan-in example: a common utility used across modules
pub struct Logger;

impl Logger {
    pub fn log(&self, msg: &str) {
        println!("[LOG]: {}", msg);
    }
}

// Used by multiple modules:
mod auth {
    use super::Logger;
    pub fn authenticate(user: &str, logger: &Logger) {
        logger.log(&format!("Authenticating user: {}", user));
    }
}

mod billing {
    use super::Logger;
    pub fn charge(amount: f64, logger: &Logger) {
        logger.log(&format!("Charging user: ${}", amount));
    }
}

fn main() {
    let logger = Logger;
    auth::authenticate("Alice", &logger);
    billing::charge(42.0, &logger);
}
```

`Logger` has high fan-in. If its interface changes, say to include timestamps or write to a file, every dependent module may need modification.

Low fan-in means a module isn't reused much, which sometimes indicates duplication or a missed abstraction: three modules each with their own date parsing function, for instance. Medium fan-in, like a validation module used by authentication and user profiles, is generally healthy. High fan-in is your logging framework or connection pool: central, and risky to change.

### Balancing the two

High fan-in with low fan-out is what you want. The module is simple and widely useful, like a date formatting utility that depends only on the standard library.

Low fan-in with high fan-out is dangerous. The module depends on many others but isn't reused, so it's pure maintenance burden. Think of a legacy report generator that imports from ten modules and gets called from one place.

High fan-in with high fan-out is risky, because the module is both central and complex, so every change cascades. This is usually a god object coordinating multiple subsystems.

Low fan-in with low fan-out is isolated. Safe, possibly underused, like an encryption module that only runs during setup.

When you do have a high fan-in module, keep interfaces minimal and stable and expose only what's needed through `pub(crate)` or traits. Rather than exposing ten logging methods, expose one parameterized method that handles the cases internally. Write integration tests so downstream modules don't break during internal refactors. Apply versioning discipline, since a breaking change to a widely-used module should trigger a major version bump and migration notes. And document the interface, because high fan-in modules become de facto APIs for the team whether you intended that or not.

```rust
// Good balance example
pub mod date_utils {
    use chrono::{DateTime, Utc};
    
    pub fn now_iso() -> String {
        Utc::now().to_rfc3339()
    }
}

// Used by multiple subsystems
mod audit_log;
mod reports;
mod analytics;
```

Here `date_utils` has high fan-in and low fan-out, since it only depends on `chrono`. That's a clean, stable dependency structure.

### Cohesion

Cohesion measures how strongly related the responsibilities of a single module are. A highly cohesive module does one thing and contains only elements that contribute to that purpose. A low-cohesion module mixes unrelated concerns: data handling, UI formatting and network calls all in one place.

It's a qualitative property, and it reflects how understandable, maintainable and reusable a module is. High cohesion tends to reduce bugs and side effects, because the logic is contained and has a clear reason to exist.

```rust
// Low cohesion example
pub mod account_service {
    use uuid::Uuid;

    pub struct AccountService {
        pub db_conn: String,
    }

    impl AccountService {
        pub fn create_account(&self, user_name: &str) -> Uuid {
            // Handles database logic
            println!("Connecting to DB: {}", self.db_conn);
            let id = Uuid::new_v4();
            println!("Inserting new account for {}", user_name);
            
            // Also handles unrelated responsibilities:
            // formatting, validation, and even sending an email!
            if user_name.is_empty() {
                panic!("Invalid username");
            }
            
            self.send_welcome_email(user_name);
            id
        }

        fn send_welcome_email(&self, user_name: &str) {
            println!("Sending welcome email to {}", user_name);
        }
    }
}
```

`AccountService` is doing database management, input validation, logging and email. Change how emails work, or how validation happens, and you're editing the same module. That's low cohesion, and it means unrelated changes start interfering with each other.

Refactored so each module has one responsibility:

```rust
pub mod account_service {
    use uuid::Uuid;
    use crate::{database::Database, email::EmailService, validation::Validator};

    pub struct AccountService<'a> {
        pub db: &'a Database,
        pub email_service: &'a EmailService,
        pub validator: &'a Validator,
    }

    impl<'a> AccountService<'a> {
        pub fn create_account(&self, user_name: &str) -> Result<Uuid, String> {
            self.validator.validate_username(user_name)?;
            let account_id = self.db.insert_new_account(user_name)?;
            self.email_service.send_welcome(user_name)?;
            Ok(account_id)
        }
    }
}

// Cohesive, focused modules below:

pub mod validation {
    pub struct Validator;

    impl Validator {
        pub fn validate_username(&self, user_name: &str) -> Result<(), String> {
            if user_name.is_empty() {
                Err("Username cannot be empty".into())
            } else {
                Ok(())
            }
        }
    }
}

pub mod email {
    pub struct EmailService;

    impl EmailService {
        pub fn send_welcome(&self, user_name: &str) -> Result<(), String> {
            println!("Sent welcome email to {user_name}");
            Ok(())
        }
    }
}

pub mod database {
    use uuid::Uuid;

    pub struct Database;

    impl Database {
        pub fn insert_new_account(&self, user_name: &str) -> Result<Uuid, String> {
            println!("Inserted new account for {user_name}");
            Ok(Uuid::new_v4())
        }
    }
}
```

Now `account_service` coordinates account creation, `validation` handles validation, `database` encapsulates storage, and `email` handles outbound mail. Each has [one reason to change][12]. If the email API changes, you modify `email.rs` and nothing else.

Testing gets easier too, which is usually the first sign that cohesion improved:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_account_successfully() {
        let db = Database;
        let email = EmailService;
        let validator = Validator;
        let service = AccountService { 
            db: &db, 
            email_service: &email, 
            validator: &validator 
        };
        
        assert!(service.create_account("Alice").is_ok());
    }

    #[test]
    fn rejects_invalid_usernames() {
        let db = Database;
        let email = EmailService;
        let validator = Validator;
        let service = AccountService { 
            db: &db, 
            email_service: &email, 
            validator: &validator 
        };
        
        assert!(service.create_account("").is_err());
    }
}
```

---

## Level 3: Classes

The third level builds on the second. Once you've defined the rules and general interactions between modules, you move more granularly into the system's classes.

The main thing is understanding the responsibilities of each module and designing its classes or structs accordingly. Being conscious of cohesion helps here, because it gets overwhelming fast if you try to design responsibilities and interactions while also holding a reasonable dependency model in your head.

Say a module is in charge of networking. It's tempting to put logic handlers in there to manage network events directly. That's faster to write, and it spreads your business logic across modules. You could instead create a class in the business logic module that handles network events, leaving the networking module focused on connection management, transmission and protocol handling.

Concretely: you're building a multiplayer game. The networking module handles TCP/UDP connections, packet serialization and retry logic. When a "player joined" packet arrives, the game logic module decides how to spawn that player, update state and notify other systems. Networking raises an event, game logic consumes it.

In an OOP-focused design you also need to be careful about what belongs to the class versus what belongs to the objects. Conflating the two gets confusing quickly.

It's tempting to store global information about users, like total user count, in the User class, which is also the instance class for a user. That produces a messy architecture, because the static variable is hard to manage across instances, and concurrency and testing both get harder once your user object is spread through the codebase.

That mistake is fairly intuitive to avoid on its own. The convenience gets more tempting when there's a lot of boilerplate. Consider User functions that operate on a database connection. If many classes need that connection, you might want to store it statically, or on the instance, rather than passing it into every call.

That creates three problems. Testing becomes difficult, because you can't easily mock or replace the database. Concurrency issues appear, because multiple threads hitting a shared static connection can race. And lifecycle management gets unclear: when is the connection opened, closed, refreshed?

Dependency injection is the better approach. Pass the connection or pool as a parameter, or store it in a context object that's explicitly passed through the call chain.

```rust
// Avoid this: static connection embedded in the class
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    // Bad: relies on global state
    pub fn save(&self) {
        GLOBAL_DB_CONNECTION.execute("INSERT INTO users ...");
    }
}

// Prefer this: explicit dependency
pub struct User {
    pub id: Uuid,
    pub name: String,
}

impl User {
    // Good: explicit dependency
    pub fn save(&self, db: &Database) -> Result<(), DbError> {
        db.execute("INSERT INTO users (id, name) VALUES (?, ?)", &[&self.id, &self.name])
    }
}
```

This level is tricky to work through in isolation. There will still be open questions about whether various classes can actually interact the way you assumed. With less experience in a language or architectural style, expect to spend real time researching the relationships you intend to create and how your classes have to accommodate them.

Avoiding the general anti-patterns, like circular dependencies or oversized functions, makes these heuristics apply more or less by default. If Class A depends on Class B and Class B depends on Class A, that's usually a design flaw. The fix is normally a third class or interface that both depend on, or rethinking responsibilities so the dependency flows one way.

Sometimes you need to think outside an OOP framework entirely. That's harder, because thinking about a codebase abstractly is genuinely difficult, but learning it can improve both performance and how you manage classes.

Rather than creating a class for each particle or object in a game engine, you might use [Data-Oriented Design][13], which moves your objects into classes responsible for changes in data rather than for the objects themselves. Instead of a `Particle` class with position, velocity and colour, you have separate arrays for each. Operations iterate over those arrays in parallel, which is more cache-friendly and usually faster.

```rust
// Object-Oriented approach
struct Particle {
    position: Vec3,
    velocity: Vec3,
    color: Color,
}

let particles: Vec<Particle> = vec![/* ... */];

// Data-Oriented approach
struct ParticleSystem {
    positions: Vec<Vec3>,
    velocities: Vec<Vec3>,
    colors: Vec<Color>,
}

impl ParticleSystem {
    fn update(&mut self, delta_time: f32) {
        for i in 0..self.positions.len() {
            self.positions[i] += self.velocities[i] * delta_time;
        }
    }
}
```

This level works best in close conjunction with the next one.

---

## Level 4: Classes to Routines

Now you decide how data, events and operations flow through the system. It's simpler to use objects to abstract real-world entities, but this level requires thinking about exactly how your data changes, so you can work out what to do to change it.

My favourite way to start is by asking what I'd like the API of a module or class to look like for its use case.

Say you want to insert data into some store, and your library requires the data be verified before it touches the backend. If you want the API to be clean, you want this:

```rust
database.put(some_data);
```

The flow being: user insert, then our database API, then verification, then insert.

The trouble starts with verification:

```rust
fn put(&self, data: T) -> Result<(), SomeError> {
    // Verify
    match data {
        OneThing => if data.id != 0 { /* ... */ } else { return Err(SomeError) },
        AnotherThing => if data.other_condition() { /* ... */ } else { return Err(SomeError) },
        _ => // ...
    }
    
    // Transform
    let encoded = data.encode();
    
    // Insert
    backend.insert(encoded)
}
```

There's a clear flow of data all encapsulated in one function. That's hard to debug, because if the same flow exists elsewhere you may have to visit every instance to find where the encoding error came from. It also violates the Single Responsibility Principle, since `put` verifies, transforms and inserts.

Separating the concerns:

```rust
pub struct Database<'a> {
    validator: &'a Validator,
    encoder: &'a Encoder,
    backend: &'a Backend,
}

impl<'a> Database<'a> {
    pub fn put(&self, data: T) -> Result<(), SomeError> {
        let validated = self.validator.validate(data)?;
        let encoded = self.encoder.encode(validated)?;
        self.backend.insert(encoded)?;
        Ok(())
    }
}
```

Each step is isolated and testable. If encoding fails you know where to look, and the validator and encoder are reusable elsewhere.

Another example: an array of objects that need updating frequently on some event, which is common in simulations, game engines and GUI libraries.

```rust
let some_vect = vec![ThisObject, ThisObject, ThisObject, ThisObject];

fn update_property(some_objects: Vec<ThisObject>, some_mutation: u8) {
    for object in some_objects {
        object.property = object.property + some_mutation;
    }
}

fn update_another_property(some_objects: Vec<ThisObject>, some_mutation: u8) {
    for object in some_objects {
        object.other_property = object.other_property + some_mutation;
    }
}
```

Updating two properties means iterating over everything twice. Instead:

```rust
struct ThisObject {
    property: u8,
    other_property: u8,
}

impl ThisObject {
    // Encapsulate updates within the object
    fn update(&mut self, mutation: u8, other_mutation: u8) {
        self.property = self.property.saturating_add(mutation);
        self.other_property = self.other_property.saturating_add(other_mutation);
    }
}

fn update_all(objects: &mut [ThisObject], mutation: u8, other_mutation: u8) {
    for object in objects {
        object.update(mutation, other_mutation);
    }
}
```

One pass, both properties.

If the datasets are large and performance matters, the data-oriented version:

```rust
struct ObjectSystem {
    properties: Vec<u8>,
    other_properties: Vec<u8>,
}

impl ObjectSystem {
    fn update_all(&mut self, mutation: u8, other_mutation: u8) {
        for prop in &mut self.properties {
            *prop = prop.saturating_add(mutation);
        }
        for other_prop in &mut self.other_properties {
            *other_prop = other_prop.saturating_add(other_mutation);
        }
    }
}
```

This enables SIMD optimizations and better cache locality, which matters in tight loops.

### Designing routine interactions

Take a notification system, with the requirement "the system shall notify users of important events via email and push notifications."

```rust
fn notify_user(user_id: UserId, message: &str) {
    // Send email
    let email_client = EmailClient::new();
    email_client.send(user_id, message);
    
    // Send push notification
    let push_client = PushClient::new();
    push_client.send(user_id, message);
    
    // Log notification
    println!("Notified user {} with message: {}", user_id, message);
}
```

This is hard to test, since you can't test email and push separately. It's hard to extend, since adding SMS means modifying `notify_user`. And it's tightly coupled, since `notify_user` knows about email, push and logging.

```rust
trait NotificationChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError>;
}

struct EmailChannel {
    client: EmailClient,
}

impl NotificationChannel for EmailChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError> {
        self.client.send(user_id, message)
    }
}

struct PushChannel {
    client: PushClient,
}

impl NotificationChannel for PushChannel {
    fn send(&self, user_id: UserId, message: &str) -> Result<(), NotificationError> {
        self.client.send(user_id, message)
    }
}

struct NotificationService {
    channels: Vec<Box<dyn NotificationChannel>>,
}

impl NotificationService {
    fn notify(&self, user_id: UserId, message: &str) -> Vec<Result<(), NotificationError>> {
        self.channels
            .iter()
            .map(|channel| channel.send(user_id, message))
            .collect()
    }
}
```

Each channel is isolated and testable, adding SMS or Slack doesn't require touching `NotificationService`, and the service orchestrates channels without knowing how they work.

---

## Level 5: Routine Interactions and Data

The first step here is checking that, at a high level, the functions you've defined actually fulfil the requirements. You may not know every requirement or exactly how it'll be met, but you should at least be able to say "the system will execute function x to fulfil requirement a."

Consider all the requirements before writing code. Performance and security are difficult to refactor into an existing codebase without extensive rework. Discovering late that your authentication doesn't meet security requirements, that passwords aren't hashed properly or sessions aren't invalidated correctly, means reworking significant portions of the system.

This is where you spend time designing and documenting your intention. It's also the best point to reduce work for the implementation phase, because specificity reduces the mental work of implementing. Done well, you stop asking yourself how to implement specific functionality.

It also creates a consensus for everyone involved, including Future You, which makes dividing work easier. If one team relies on another's outputs, the postconditions are clear before the functions exist.

A few things worth finalizing here.

Requirements and their documentation. Every function should map back to at least one requirement, and if it doesn't, question why it exists.

Chosen design patterns. Document which you're using and where: Factory for object creation, Observer for events, Strategy for interchangeable algorithms.

Parameters and return values. Define each signature, what it takes, what it returns and what the edge cases are.

```rust
/// Authenticates a user by username and password.
/// 
/// # Parameters
/// - `username`: The user's unique identifier
/// - `password`: The plaintext password
/// 
/// # Returns
/// - `Ok(SessionToken)` if authentication succeeds
/// - `Err(AuthError::InvalidCredentials)` if credentials are wrong
/// - `Err(AuthError::AccountLocked)` if the account is locked
/// 
/// # Security
/// - Passwords are compared using constant-time comparison
/// - Failed attempts are rate-limited
fn authenticate(username: &str, password: &str) -> Result<SessionToken, AuthError>;
```

Pre- and post-conditions. What must be true before the function runs, and what will be true after.

```rust
/// Withdraws money from an account.
/// 
/// # Preconditions
/// - `account_id` must exist in the database
/// - `amount` must be positive
/// - Account balance must be >= amount
/// 
/// # Postconditions
/// - Account balance is reduced by `amount`
/// - Transaction is logged
/// - If balance falls below minimum, a warning flag is set
fn withdraw(account_id: Uuid, amount: f64) -> Result<(), WithdrawError>;
```

Assertions and checks that enforce them. Debug assertions for internal invariants, runtime checks for external input.

```rust
fn withdraw(account_id: Uuid, amount: f64) -> Result<(), WithdrawError> {
    // Precondition checks
    if amount <= 0.0 {
        return Err(WithdrawError::InvalidAmount);
    }
    
    let account = get_account(account_id)?;
    if account.balance < amount {
        return Err(WithdrawError::InsufficientFunds);
    }
    
    // Perform withdrawal
    account.balance -= amount;
    log_transaction(account_id, amount);
    
    // Postcondition check (in debug builds)
    debug_assert!(account.balance >= 0.0, "Balance went negative!");
    
    Ok(())
}
```

And a high-level implementation plan. You don't need pseudocode for every line, just the major steps.

```text
Function: process_payment
1. Validate payment details (card number, CVV, expiry)
2. Check for fraud (via external API)
3. Charge card (via payment gateway)
4. If successful:
   a. Update order status to "paid"
   b. Send confirmation email
   c. Log transaction
5. If failed:
   a. Log failure reason
   b. Notify user
   c. Retry if transient error (up to 3 times)
```

Document all of that and implementation becomes close to mechanical. You get to focus on writing correct code instead of working out what to write.

---

# Goals and Deliverables

Somewhere during design, or during the activities that follow it, you start thinking about the outputs of the project. Depending on your methodology and communication style you'll need to decide what measuring stick you're using to track progress.

As overused as it is, [SMART][14] is still a reasonable way to guide goal setting.

**Specific.** For teams of any size it matters that everybody is on the same page, which is different from making sure everybody understands. Understanding can only be measured by the person being assessed, and it's entirely possible for everyone to understand the same thing differently. Specificity mitigates that. "Improve the website" is vague. "Reduce the homepage load time from 4 seconds to under 2 seconds for users on 3G connections" is specific: it says what, how much, and for whom.

**Measurable.** Measures help with mentality, progress tracking and plain tangibility. I like them mostly because they make sure understanding carries through. "Improve system reliability" isn't measurable; "achieve 99.9% uptime over the next quarter" is. Measurability also gives you intermediate checkpoints, so you can track weekly uptime and correct course before the quarter ends.

**Agreed upon.** Everyone affected has to agree: developers, managers, customers. Disagreement produces misaligned expectations. If the development team thinks the goal is a functional prototype and the customer expects production-ready software, you have a problem. Review and sign off on goals before work starts, and write the agreement down.

**Realistic.** Goals should be challenging and achievable given the resources. Unrealistic goals demoralize people, encourage corner-cutting and produce technical debt. "Rebuild the entire application from scratch in two weeks with one developer" is not realistic. "Refactor the authentication module to use [OAuth2][15] within the next two-week sprint with the current team of three" might be. To assess realism, look at how long similar tasks took historically, how much time each person can actually dedicate, and what's blocked by things outside your control.

**Time-based.** Every goal needs a deadline, because without one there's no urgency and work expands to fill the time available, which is [Parkinson's Law][16]. "Implement user authentication" is open-ended. "Implement user authentication by November 15th" isn't. Deadlines also make retrospectives possible, so you can adjust future estimates based on what actually happened.

## Deliverables

Deliverables are the items that need to be produced to meet the goals of a project. Like goals they need to be specific and verifiable, but they also need to tick boxes of their own, outside the SMART checklist. Creating quality standards for both goals and deliverables is what lets everybody agree that a measure has been met.

For the goal "launch the new feature by December 1st," the deliverables might be a design document due November 1st, with the standard that it's reviewed and approved by at least two senior engineers and the product manager. Functional code due November 20th, with all acceptance criteria met, tests passing and code review approved. Documentation due November 25th, with user-facing docs complete, internal API docs generated and no broken links. And a production deployment on December 1st, with the feature flag enabled for 10% of users and no critical bugs in the first 48 hours.

Defining deliverables and their quality standards is what removes subjectivity from "done."

### Goals and deliverables together

"Improve system security" is vague and not measurable. What does improve mean, and how would you know when you got there?

A better goal is "eliminate all critical and high-severity security vulnerabilities identified in the latest penetration test by the end of Q1." The deliverables are a penetration test report received January 5th from a certified third-party firm, a remediation plan due January 15th reviewed by the CTO and security team with a timeline and owners for each vulnerability, patched code due March 15th with all critical and high-severity vulnerabilities addressed, and a follow-up penetration test due March 25th showing none remaining.

Now the goal is clear and the deliverables are checkpoints. If patched code slips, you know the goal is at risk while there's still time to act.

Goals without deliverables give you a target and no roadmap, so teams don't know what to produce. "Make the app faster" could mean optimizing database queries, caching API responses, or reducing image sizes, and without deliverables developers might optimize the wrong thing entirely.

Deliverables without goals give you outputs with no purpose. A team might produce a 50-page design document, 10,000 lines of code and comprehensive tests, and if the goal was to validate feasibility with a quick prototype, all of that was wasted. A proof of concept would have done it.

---

## Scheduling

This article doesn't cover scheduling in detail, and a future series might, but scheduling during the planning phase is a bit strange.

Unless you're running a project everybody has done before, schedules are hard to adhere to when the implementations are unknown. You don't want them too tight, because you don't know what will go wrong. You also want to actually finish, and too much slack either isn't possible or kills the team's momentum.

Say task A and task B are both scheduled for 5 days and both feed into task C. If A finishes in 1 day, four days are idle. If B needs 7, C is late.

A few things help. Estimate in ranges rather than points, so "3 to 7 days" instead of "5 days." Build in 20 to 30% buffer for the unexpected, sometimes called management reserve. Parallelize where tasks are genuinely independent. Use rolling wave planning, where near-term tasks are planned in detail and far-future ones stay high level until you know more. And track actual against estimated time, because that's the only way the intuition ever develops.

The next few articles walk through techniques for managing this and for estimating work.

---

# Supporting Plans

A couple of additional plans are worth having, mostly because they account for the fact that humans are involved and our interactions and risk management benefit from being standardized. Everybody would resolve an obstacle their own way, and what you want is for the method to be predictable and efficient.

## Human resources plan

The next article covers this properly. For now, a human resources plan is how you manage workloads. People are themselves resources, and over the course of a project they often need to be reorganized.

The plan establishes key roles and responsibilities so that when things change, nobody ends up out of place, unprepared or badly suited to the change. That usually means role definitions covering who is responsible for what, a staffing plan covering how many people you need and when, an onboarding process, a training plan for the skills people need to develop, and a documented way of handling disagreements or performance issues.

Writing this down upfront is what stops you making ad-hoc decisions that disrupt the team.

## Communications plan

Depending on your methodology you may need to define how people interact and why. Knowing why an interaction happens has two benefits.

It focuses the interaction. Meetings with a purpose are more productive, because there's no room for unrelated work. Regular updates shouldn't include interpersonal issues; those need their own defined path. A daily standup answers three questions: what did I accomplish yesterday, what will I work on today, and are there any blockers. That keeps it to fifteen minutes. Detailed technical discussions and design debates happen elsewhere.

It also minimizes unnecessary communication. People often don't know where to raise a concern, which is a problem when the issue is only urgent in some contexts. If someone notices a module from another team misbehaving, waiting until the next meeting could be harmful in production. Defining where those go prevents that.

In practice that means something like: chat or email for non-urgent updates and questions, the daily standup for status and blockers, a weekly planning meeting for sprint planning and backlog grooming, an on-call system for critical production issues, and one-on-ones for career development and sensitive feedback.

---

Projects are complicated and difficult to execute, and planning how to manage them is the first step toward getting one finished. There are five things you need to plan for to cover your bases: people, meaning who's involved and how they communicate; product, meaning what you're building and what it requires; price, meaning budget and available resources; process, meaning your methodology and how work gets tracked; and project, meaning the schedule and the milestones.

Address those during planning and you reduce uncertainty, align everyone, and create a shared understanding of what needs to happen.

---

## References

[1]: https://en.wikipedia.org/wiki/Systems_development_life_cycle "Software Development Life Cycle (SDLC) - Wikipedia"
[2]: https://www.projectmanager.com/guides/waterfall-methodology "Waterfall Model - Project Management Guide"
[3]: https://www.agilealliance.org/glossary/incremental-development/ "Incremental Development - Agile Alliance"
[4]: https://gdpr.eu/what-is-gdpr/ "General Data Protection Regulation (GDPR) Guide"
[5]: https://www.hhs.gov/hipaa/index.html "Health Insurance Portability and Accountability Act (HIPAA) - HHS"
[6]: https://auth0.com/blog/hashing-passwords-one-way-road-to-security/ "Password Hashing Best Practices - Auth0"
[7]: https://en.wikipedia.org/wiki/Brooks%27s_law "Brooks's Law - Wikipedia"
[8]: https://www.atlassian.com/software/jira "Jira - Issue & Project Tracking Software"
[9]: https://www.jenkins.io/ "Jenkins - Continuous Integration and Delivery"
[10]: https://www.djangoproject.com/ "Django Web Framework"
[11]: https://martinfowler.com/architecture/ "Software Architecture Guide - Martin Fowler"
[12]: https://en.wikipedia.org/wiki/Single-responsibility_principle "Single Responsibility Principle - Wikipedia"
[13]: https://www.dataorienteddesign.com/dodbook/ "Data-Oriented Design - Richard Fabian"
[14]: https://www.projectsmart.co.uk/smart-goals.php "SMART Goals - Project Management Guide"
[15]: https://oauth.net/2/ "OAuth 2.0 - Authorization Framework"
[16]: https://en.wikipedia.org/wiki/Parkinson%27s_law "Parkinson's Law - Wikipedia"
