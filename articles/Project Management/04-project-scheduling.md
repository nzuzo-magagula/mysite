#####
date = "2025-11-07"
author = "Nzuzo Magagula"
summary = "Scheduling techniques and theories"
thumbnail = "https://i.postimg.cc/pd1RWNGX/G2CM-BI108-Backlog-Images-Project-mgmt-approach-V1b.png"
category = "Educational"
show_references = true

[[article_series]]
name = "Project Management"
prev = "Project Management/03-metrics"
next = "Project Management/05-network-analysis"
#####
# The Project Scheduling Process

Scheduling is one of the more practical parts of project management. It's where plans turn into timelines and activities, and where you work out who does what, when, and in what order.

It comes down to three things: dividing work, assigning it, and estimating how long it takes. Each feeds into a schedule you can track, communicate and refine as the project runs.

## Why Bother With a Schedule?

A schedule gives structure, visibility and a rhythm to a project that would otherwise feel formless.

### It measures progress

Schedules quantify how far along you are against where you planned to be. If a feature was meant to be done by Friday and it's Wednesday with nothing to show, that gap is telling you an assumption was wrong. Maybe the work was underestimated, or there's a dependency nobody saw.

That makes the schedule a feedback loop: it tells you whether the original assumptions about time, effort and sequence still hold. When they don't, you adjust.

Say you planned three days to integrate a payment gateway, and testing exposes a dependency on a vendor API that isn't available yet. The dependency shifts your timeline and possibly the priority of everything after it.

### It makes the project legible

Large projects are hard to hold in your head. Schedules make them digestible by showing not just what needs to happen but how the pieces connect.

If team A is building a login system while team B builds a payment module, team B might depend on team A's authentication tokens for integration testing. Without a shared schedule, both teams work in silos and end up blocking each other by accident.

A good schedule makes dependencies explicit, so each team knows when the things they need will be ready.

### It makes teams responsive

Watching which tasks lag and which finish early lets a project manager reassign people, revise milestones or revisit scope before a small delay becomes a big one. The schedule ends up working as an early warning system.

### The underlying goal

Ultimately the goal is to minimize unnecessary dependencies between tasks. A tightly interdependent project is fragile, because one delay cascades through everything. A loosely coupled one has flexibility.

It's modular design applied to time. The fewer dependencies a task has, the more work can happen in parallel without bottlenecks.

## Non-Agile Scheduling

In a [traditional or waterfall][1] environment, scheduling is a pre-planning exercise. The goal is a linear, predictable sequence of tasks built from detailed requirements and design documents.

Starting from requirements and design specifications, you identify activities by breaking the project into discrete work units, identify which ones depend on which, estimate the people, tools and materials needed, allocate the right people to the right tasks, and then visualize the sequence, usually as a [Gantt chart][2].

That chart becomes the roadmap for tracking progress.

## Gantt Charts

A [Gantt chart][2] is a visual timeline with tasks on a horizontal time axis. Each task is a bar whose position and length reflect its start date, end date and duration, and arrows between bars show dependencies.

It answers three questions at once: what needs to be done, when it will be done, and how it relates to everything else.

Say a team is building a mobile banking app. The chart might show "Design Login UI" running January 1 to 5, "Implement Login API" running January 6 to 10, and "Integration Testing" starting only once both are complete. If the design phase slips, the whole downstream schedule shifts, and that visibility is the point.

Gantt charts are also difficult to get right. Projects rarely follow an exact sequence, especially in software where discovery and iteration are normal. Treat them as guides rather than scripts. They give you structure and foresight; flexibility has to stay part of the philosophy.

They're most useful when time is the primary constraint: construction projects with fixed delivery dates, hardware production schedules, or software releases tied to external commitments like a marketing launch.

## Kanban Boards

Where Gantt charts show when work happens, [Kanban boards][3] show how work flows. They visualize the current state of tasks and make progress easy to track dynamically.

A board is divided into columns representing stages, and tasks move between them as they progress. That's enough to see where work is bottlenecked and what's actively being worked on.

A typical board has a backlog holding all potential tasks, a to-do column for the current cycle, in progress for active work, testing or review for validation, and done.

Take the task "implement password reset." It starts in the backlog. Once prioritized for the sprint it moves to to-do. A developer picks it up and it shifts to in progress. When it's done QA tests it, so it moves to testing. Once verified it lands in done.

Kanban's strength is real-time visibility. Where Gantt charts are static and predictive, Kanban boards are adaptive, which suits environments where priorities shift often: agile development teams, maintenance operations. Looking at a board tells you immediately which tasks are delayed, which stages are overloaded, and where extra help would do something.

## Using Both

Plenty of teams run both: a Gantt chart for strategic planning and a Kanban board for tactical execution. The chart sets the overall timeline and dependencies between modules, the board manages daily progress within each team.

The Gantt chart answers "where are we headed?" The Kanban board answers "where are we right now?" Between them you get long-term planning and short-term adaptability, which is roughly what you need.

---

## References

[1]: https://www.projectmanager.com/guides/waterfall-methodology "Waterfall Project Management Methodology"
[2]: https://www.gantt.com/ "Gantt Charts - History and Modern Usage"
[3]: https://www.atlassian.com/agile/kanban "Kanban Board - Agile Project Management"
