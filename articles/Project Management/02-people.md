#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "How can we manage people effectively?"
thumbnail = "https://i.postimg.cc/pd1RWNGX/G2CM-BI108-Backlog-Images-Project-mgmt-approach-V1b.png"
category = "Educational"
show_references = true

[[article_series]]
name = "Project Management"
prev = "Project Management/01-planning-and-scope"
next = "Project Management/03-metrics"
#####
# People

So far this series has covered how to plan projects and mitigate the risks that come from their unpredictability. This article carries that on, looking at project management through the lens of managing people.

## The Human Factor

The relationship between people and projects is genuinely complicated. People are nuanced, emotional and occasionally unpredictable, and one of the harder parts of project management is organizing them so they can execute something complex together.

There's no universal formula for managing people. There are still effective ways to organize them around a shared goal.

## Roles and Responsibilities

The most straightforward way to manage people is by assigning roles and responsibilities.

What makes people valuable is their identities. Each person brings a different mix of skills, preferences, strengths and perspectives, and when those differences are harnessed properly they become the engine of the project.

### Skills

A strong team has diverse skill sets and experience. That diversity fuels creativity, but more importantly it creates resilience, because the team can attack problems from several directions.

Imagine a team designing a mobile application. People naturally organize into sub-teams: UI focused on interface design and usability, networking handling data flow and APIs, systems managing backend logic and infrastructure.

The benefits of diverse skills go past those obvious divisions, though. A UI expert can't do what a backend developer does and vice versa, but the intangible skills matter just as much.

Someone who communicates well under pressure, or who enjoys presenting, is the obvious choice to lead sprint reviews or stakeholder demos. Their calm and clarity affect both morale and stakeholder confidence.

Someone who naturally notices inconsistencies and edge cases will excel at code reviews, QA and compliance. Their perceptiveness stops subtle bugs from turning into expensive ones.

Someone who senses team tension or unspoken concerns is well suited to user research, stakeholder relations or conflict resolution, where understanding people matters more than understanding code.

Using both the tangible and intangible skills improves your chances.

### People are not static

People grow, learn and adapt. On long projects, rigid role definitions lead to stagnation or burnout. Some stability is necessary for continuity, but there should be room for growth and rotation where it makes sense.

Investing in development pays off in three ways.

Cross-team communication improves. When developers understand each other's domains, collaboration gets smoother. A frontend engineer who grasps backend principles will design UI requests that are API-efficient and easier to implement.

Teams get more resilient. Cross-trained members can cover for each other. If only one person knows the deployment process, their absence is a risk. If three do, the team stays productive through unexpected changes.

And broader knowledge produces more creativity. Understanding multiple domains lets people build bridges between them. A developer familiar with both UX and async operations might propose optimistic UI updates, improving performance and user experience at once.

## Range of Duties and Responsibilities

When planning a team it helps to think in responsibility clusters: broad categories of roles that cover all the critical aspects of the project.

### Project management roles

The project manager, or management team, usually covers several distinct sub-roles.

The strategist defines long-term vision and aligns it with business priorities, balancing technical ambition against business pragmatism. Ship a minimal viable product now or a fully-featured system later?

The leader motivates the team, creates psychological safety, celebrates wins, and reframes obstacles as learning opportunities.

The politician navigates stakeholder politics and competing interests, and translates progress into business language. "ROI improvement" instead of "reduced latency."

The facilitator keeps processes and meetings productive, recognizes when discussions drift, and keeps collaboration focused and balanced.

The administrator manages logistics: budgets, schedules, documentation, compliance. This role enables rather than directs, handling procurement, tracking deliverables and maintaining institutional memory.

### Systems analyst roles

The systems analyst bridges business, users and technical teams.

On stakeholder needs, they engage deeply enough to uncover the real need under the stated request. A stakeholder asks for "a dashboard like our competitor's," and the analyst discovers the underlying goal is reducing customer support calls.

On interaction design, they design workflows that are intuitive and tolerant of stress. In a hospital that might mean clear hierarchies, confirmation prompts, and strong accessibility standards.

On cost estimation, they produce realistic estimates that account for complexity, integration time, testing and risk, not just development hours.

### User interface designer

The UI designer shapes how the system feels and communicates. That means visual hierarchy that highlights what matters, consistency that reduces cognitive load, accessibility, and responsiveness across devices.

On a project dashboard, upcoming deadlines and critical items should dominate the visual space while historical data sits in expandable sections.

They also maintain the design system, a living style guide that keeps components, typography, colour and layout consistent.

### Architect

The software architect defines and maintains the technical structure.

That means an application overview: a coherent picture of how components interact. A React frontend talking to a Node.js API, talking to microservices, talking to PostgreSQL and Redis.

It means planning for performance: scalability, caching, load balancing, and setting performance budgets with monitoring in place to measure against them.

And it means middleware: how services communicate and integrate. RabbitMQ for asynchronous messaging, Kong for API management.

### Documentation

The documentation specialist makes sure knowledge gets captured and maintained, whether technical, operational or user-facing. That covers user guides and tutorials, API and architecture documentation, deployment and troubleshooting procedures, and contribution guidelines.

If the API docs exist but have no examples, this is the person who works with developers to add real-world use cases for each endpoint.

### Domain specialists

These are the people who bring deep contextual understanding of the target industry.

In healthcare, they make sure scheduling logic reflects how clinics actually run, with 15-minute checkups and 60-minute consultations. In finance, they make sure the regulatory compliance and accounting logic are right.

They validate that the system fits the real world, not just that it's technically correct.

## Choosing People

In practice you often don't get to pick your team. You work with who you have, and your job is to organize and empower them.

When you do have a say, look for gaps in skill, temperament or experience. Introduce new members strategically, balancing short-term disruption against long-term gain. And evaluate both technical and interpersonal fit.

Things worth assessing: problem-solving style, communication clarity, comfort with ambiguity, reaction to feedback, conflict management, and the balance between initiative and collaboration.

Different evaluation methods surface different things.

| Evaluation Type           | Reveals                                 |
| ------------------------- | --------------------------------------- |
| Technical interviews      | Problem-solving & domain knowledge      |
| Behavioral interviews     | Values, teamwork, and conflict handling |
| Work samples / trials     | Actual performance in context           |
| Reference checks          | Strengths, weaknesses, and reputation   |

## Management for Success

Software projects succeed when motivated people work toward a shared goal. Modern software is too complex for individuals to build alone, so coordination and shared understanding aren't optional.

**Encourage communication.** Information flow is what collaboration runs on. Pick patterns that fit the team: daily stand-ups for tight synchronization, async updates in chat for distributed teams, retrospectives for reflection. Then balance information availability against focus time, because both matter.

**Remove obstacles.** A good manager shields the team from politics, ensures access to tools and resources, resolves conflicts early, and enables productivity rather than dictating it. This is servant leadership[1] in practice.

**Create psychological safety.** Teams do well when people can admit mistakes early, ask "dumb" questions, challenge decisions constructively, and propose unconventional ideas. Fear suppresses communication; safety enables innovation[2].

**Recognize and grow people.** Beyond pay, people need recognition so their work feels like it matters, constructive feedback that's clear and actionable, opportunities to learn and advance, and an understanding of how their work fits the whole. Motivation grows where meaning is visible.

**Stay adaptive.** No plan survives contact with reality. Requirements evolve, contexts change, people come and go. The goal isn't eliminating uncertainty, it's navigating it without falling apart.

---

## References

[1]: https://www.greenleaf.org/what-is-servant-leadership/ "What is Servant Leadership? - Greenleaf Center"
[2]: https://www.psychologytoday.com/us/basics/psychological-safety "Psychological Safety in Teams - Psychology Today"
