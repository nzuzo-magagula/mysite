#####
short_summary = "One company's bug took a large chunk of the internet offline. This series is about why that keeps happening: how centralization became the default, how individually sensible choices add up to collective fragility, and what the Rust discourse around the outage got wrong."
name = "summary"
#####
# Cloudflare Outage & The Fragility of Modern Infrastructure

Over the weekend, Cloudflare had an outage that made large parts of the internet unusable. It got bad enough that DownDetector went down, so the site we use to check whether things are down couldn't tell us things were down, because it was also down. Nobody missed the irony.

As is tradition, everyone had an opinion, and so do I. My corner of the internet was loudest about two things: how often large providers now bring the planet to a halt, and what Rust does or doesn't promise as a language.

Both conversations were more polarized than I expected, which was a useful reminder that no matter how technical the industry, people are still people and our biases shape the conversations we have. Watching how emotional people got about a programming language has changed how I think about software development generally.

## What Actually Went Wrong

Cloudflare's post-mortem says a bug in their memory allocation validation didn't handle allocation limits properly. I won't rewrite their technical blog here. In short: they had added machine learning features to their Bot Management system, and to keep performance reasonable they capped the number of features that could be processed.

The code managing that cap was written in Rust, a language that has become famous, or infamous depending on who you ask, for preventing exactly this kind of catastrophic failure.

It contained an `.unwrap()`, which is a declaration of absolute certainty that an operation will succeed, plus instructions to crash the program if it doesn't. That's common in development and dangerous in production. When the allocation limit was reached, the function returned an error as designed, and `.unwrap()` did what it was supposed to do: panicked, and took the service down. You can guess how that cascaded through systems serving a large share of the internet's traffic.

## The Part That Interests Me

The technical detail is interesting but it isn't the real question. The real question is why we've built an internet where one company's failure cascades into a global one.

The architecture drifted toward centralization not through conspiracy or negligence, but through millions of individually rational decisions that collectively produced a fragile system. When everyone independently picks the provider with the best uptime, the fastest performance and the best pricing, we concentrate risk. The better a company is at what it does, the more we centralize around it, and the worse its eventual failures become.

## The Two Articles

The first article looks at the structural side: why centralization feels inevitable, why Web3 failed to provide an alternative, why individually rational choices produce collective risk, and why arguments for diversification never seem to land. Good service is what enables dangerous concentration, and customers have almost no incentive to care.

The second looks at the language discourse: what Rust actually guarantees versus the inflated version of it, what the constraints genuinely cost you, why `unsafe` and `.unwrap()` exist, and how tribalism buried the useful lessons from the outage. Rust promises memory safety by construction, not immunity from failure. The outage wasn't a betrayal of its principles, and treating a programming language like a religion makes it impossible to talk honestly about its tradeoffs.

## Why I Care

Most of the infrastructure our digital lives run on has concentrated around a handful of providers. That brings real benefits: better performance, lower costs, more features. It also brings fragility, and that fragility shows up as outages affecting billions of people at once.

The question isn't whether we should stop using these services. It's what price we're willing to pay, in convenience or performance or cost, for a more resilient internet. And whether we can even have that conversation when every economic incentive points the other way.

I was optimistic about Web3 precisely because I saw it as a path toward resilient decentralized infrastructure. That vision got hijacked by speculation, but the underlying problem hasn't gone anywhere: we still need ways to organize critical infrastructure that don't collapse into concentration.

I don't have complete solutions. What I have is a clearer sense of the problem, which is where these two articles start.
