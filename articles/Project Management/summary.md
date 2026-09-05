#####
short_summary = "Software projects usually fail on planning and structure rather than on code. This series takes project management ideas, including the corporate ones, and adapts them for individual developers and small teams: when to plan, what actually counts as a project, and which techniques are worth the overhead."
name = "summary"
#####
# Designing and Managing Maintainable Software Projects

Designing software is complicated. Early on I would catch myself thinking "why on earth is this here, let me just move it, that feels cleaner," and then ten minutes later thinking "oh, so that's why they did it that way, I should leave it alone."

That back-and-forth between intuition and understanding is universal. It doesn't only show up in large formal architectures either. Small personal projects spiral into confusion just as fast when the design decisions were never made deliberately.

The most useful thing I've learned about this is that Future Me is not Current Me, and the two of them have never met.

Design is a mental process as much as a professional one. Whether you're working alone or in a team, structured planning is what keeps intention and action pointed in the same direction, and it's what makes code readable and predictable later.

## What Is Project Management?

Project management is the discipline of coordinating complex work so software can be delivered within constraints: time, scope, and resources.

It gets associated with large organizations, and for good reason, since businesses need predictable delivery and accountability. But the principles scale down. A startup building a mobile app might use sprints and burndown charts. A solo developer might list weekly milestones and estimate hours. Both are project management, just at different scales. The tools change, the mindset doesn't.

## What Counts as a Project?

It helps to start with what a project isn't. Projects get confused with tasks, products and processes, and those sit at different levels of abstraction.

A program or portfolio is a collection of related projects grouped by theme or strategic goal. A "Payments Program" might contain separate projects for building a payment API, designing a dashboard, and integrating fraud detection. Each is distinct, but they serve one objective.

A product is the result of a project, not the project itself. The project is "build a cross-platform note-taking app." The product is the app people download. Over time products spawn their own projects: adding sync, migrating to a new framework, and so on.

A process is a repeatable set of activities you perform regularly. "Deploy the backend every Friday" is a process. "Migrate the backend from AWS to Azure" is a project, because it happens once and has a defined end. Processes can be the output of a project, like an automated testing pipeline, or the target of one, like reducing build time.

If you own a bakery, your method for baking vanilla cakes is a process. Creating a new cheesecake recipe is a project, and the recipe it produces becomes a new process.

### How they differ in practice

Projects are created ad hoc for a specific purpose and executed once, while processes are routine and designed to repeat indefinitely. That means projects focus on achieving an outcome and processes prioritize consistency.

Projects have a start and an end date, even if those dates move. Processes run continuously or cyclically. Projects need flexible goals; processes thrive on predictability.

Project preconditions and tasks are often uncertain or exploratory. Process ones are documented and predictable, which is why it's much easier to onboard someone into a process than into a project.

Project success rates are typically lower, and failures are common and instructive. Processes improve through iteration, usually by absorbing the lessons the projects produced.

Projects can redefine their goals or outputs entirely. Processes change in small ways, refining execution rather than direction.

And they're measured differently. Projects are judged on output quality and whether the goal was met. Processes are judged on efficiency and consistency. An inefficient process compounds cost over time, while a project concludes and resets.

## What Should You Plan?

Not every idea needs a plan. Planning should match the scale, risk and uncertainty of the work.

There's a useful distinction here: project planning defines *how* something gets done, covering scheduling, cost estimation and risk. Software design defines *what* gets done, covering architecture, data models and behaviour. They overlap in practice, and the questions look similar either way. When should this be finished? How will a future contributor understand it? What does it depend on? How will it be tested? How should the interfaces evolve? What sequence of events gets us to the goal?

A beginner building a to-do app doesn't need a Gantt chart, but writing down "finish the CRUD backend before UI work" can save hours. That's already lightweight project management.

Plenty of the techniques in this series will be overkill for what you're doing. The context of your project is the only real way to decide what you need, and over time you develop an intuition for it.

### When to plan

My heuristic is to estimate the effort, double it, then decide whether it's worth planning in advance.

For small utilities, like a script that parses CSV files, planning is minimal. For multi-component systems, say an IoT dashboard that collects, stores and visualizes data, planning becomes critical, because there are several layers of coordination and the thing has to survive long-term maintenance.

That's really why planning exists. Most software is intended to persist, and maintaining it over years requires documentation, consistency and clarity for everyone who touches it later, including you. In production this stops being a convenience and becomes survival. A lack of planning brings down critical systems or stalls delivery over dependencies nobody understood.

## Why Plan at All?

Sometimes you shouldn't. Project management can become a trap where planning replaces doing. Over-planning feels safe and quietly stalls execution.

No planning can be just as bad. For paid or mission-critical work, skipping it invites cost overruns, scope creep and outright failure.

The balance comes down to two variables: how much you don't yet know, and how much coordination is required. The more of either, the more planning earns its keep. Writing a script to rename files? Just start. Building a REST API for multiple clients? Outline the endpoints, error cases and versioning strategy first. Contributing to an open-source compiler? Plan, because the complexity demands it.

### A practical example

When I was developing [`netabase_store`](https://github.com/newsnet-africa/netabase_store), integrating with [libp2p](https://docs.rs/libp2p/latest/libp2p/index.html) turned out to be far trickier than I expected.

What started as a simple wrapper spiralled into serialization failures on non-serializable record types, excessive cloning for multithreaded communication, and a borrow checker that kept exposing design flaws I hadn't planned around. Each refactor introduced new dependencies and side effects.

What I eventually took from it is that uncertainty is the signal that planning is needed. Every unexpected problem made the next round of planning better informed, and once I understood the constraints, development became straightforward, because the hard thinking was already done.

## This Series

The series draws on *Software Engineering* by [Ian Sommerville](https://books.google.co.za/books/about/Software_Engineering_Global_Edition.html?id=W_LjCwAAQBAJ&redir_esc=y) and adapts it for individual developers and smaller teams.

The first half covers planning and control: how project plans get established, how to define goals and requirements, how to estimate cost and time without lying to yourself, and how to balance flexibility against accountability when you're working alone.

The second half covers execution and risk: methodologies like Agile, Waterfall and Spiral and how to adapt them at small scale, managing iterative feedback, identifying and quantifying risk, and translating enterprise techniques into something useful for personal productivity.

From there we move into scope, scheduling, and the network analysis techniques that tell you which delays actually matter.
