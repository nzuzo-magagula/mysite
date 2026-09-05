#####
date = "2025-11-25"
author = "Nzuzo Magagula"
summary = "Exploring why one bug broke the internet, the role of centralization in modern tech, and how our consumption choices shape internet infrastructure"
thumbnail = "https://i.postimg.cc/3wMCP1N0/cracked-white-plaster-wall-texture-background.jpg"
category = "Opinion"
show_references = true

[[article_series]]
name = "Cloudflare Outage & Infrastructure Fragility"
next = "Cloudflare Outage/02-rust-growing-pains"

[[references]]
title = "Web3 - Wikipedia"
url = "https://en.wikipedia.org/wiki/Web3"
description = "Overview of Web3 principles and philosophies for decentralizing the internet"

[[references]]
title = "What Happened to Web3 - Slidebean"
url = "https://slidebean.com/story/what-happened-to-web3"
description = "Analysis of the Web3 startup boom and its sustainability challenges"

[[references]]
title = "The Post-Hype Playbook: Web3 Marketing Credibility - Hackernoon"
url = "https://hackernoon.com/the-post-hype-playbook-unhashed-ceo-mia-p-on-marketing-web3-credibility"
description = "How Web3 value propositions became lost in hype cycles"

[[references]]
title = "Web3 & Fintech 2025 and Beyond - LinkedIn"
url = "https://www.linkedin.com/pulse/web3-fintech-2025-beyond-making-waves-transforming-industries-jelic-2fhzf"
description = "Signs of Web3 evolving beyond hype as the ecosystem matures"

[[references]]
title = "Artificial Intelligence Illiteracy - The Atlantic"
url = "https://www.theatlantic.com/culture/archive/2025/06/artificial-intelligence-illiteracy/683021/?gift=a488bXrqvMlx1958JHI5qDnArF6wxd8fux6Y1VNDFMc"
description = "Parallels between AI and Web3 hype cycles and technical illiteracy"

[[references]]
title = "Is Cloudflare a Monopolist? - Dev.ua"
url = "https://dev.ua/en/news/chy-ie-cloudflare-monopolistom-iz-zakhystu-saitiv-vid-atak-ni-os-piatirka-alternatyv-1763473659"
description = "Analysis of Cloudflare's market position and available alternatives"

[[references]]
title = "Cloudflare is Destroying the Open Internet - GoAuthentik"
url = "https://version-2024-2.goauthentik.io/blog/2023-02-07-cloudflare-is-destroying-the-open-internet"
description = "Critical examination of Cloudflare's role in internet infrastructure sustainability"

[[references]]
title = "Cloudflare CEO on Google Abusing Monopoly - Fortune"
url = "https://fortune.com/2025/11/13/cloudflare-ceo-google-abusing-monopoly-search-ai/"
description = "Discussion of monopolistic practices and competitive behavior in tech"

[[references]]
title = "The Consumer Dilemma - SSRN"
url = "https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115"
description = "Academic paper on consumer choices against personal interests for societal benefit"

[[references]]
title = "AI Innovation and Monopolization - arXiv"
url = "https://arxiv.org/abs/2405.21015"
description = "Research on unsustainable innovation models designed to monopolize markets"

[[references]]
title = "Subscription Economics - ACM Digital Library"
url = "https://dl.acm.org/doi/abs/10.1145/3366423.3380281"
description = "Analysis of business models where consumers pay more than they consume"
#####
# The Fragility of Centralized Infrastructure

## Three Outages in a Month. What Is Going On?

This feels like a good-faith question that is weirdly hard to answer. None of us are sitting in the offices of these providers watching the decisions get made. We can guess, and probably guess well, but unless an executive comes out and says "we all agreed to make things worse," we are working from assumptions.

I don't think a simple answer covers it anyway. The infrastructure of the internet is becoming fragile, and not because of technical limits. It's fragile because of how we organize and delegate services.

Imagine three cities drawing water from the same treatment plant, electricity from the same station, and food from the same distribution center. When the plant goes down, three cities lose water at once. The efficiency that made the arrangement attractive is the same thing that makes the failure enormous.

That is roughly where internet infrastructure sits. Plenty of articles have covered this already, so I want to focus on why it keeps happening, and why it might be inevitable under the economics we currently have.

## Concentration Keeps Winning

We live in an era where it is more or less expected that every industry ends up with a handful of default providers.

A few years ago the popular answer to this was [Web3](https://en.wikipedia.org/wiki/Web3). The vision was reasonable: distributed networks instead of a few companies holding the infrastructure, and no single point of failure big enough to take everything down.

What actually happened is that Web3 got treated as a new market to monopolize. Startups appeared [like crazy](https://slidebean.com/story/what-happened-to-web3), and it became obvious the trend wasn't sustainable. The movement turned into the thing it claimed to oppose.

Most of the criticism at the time was about how transparent the gold rush was. The value proposition got buried under [hype](https://hackernoon.com/the-post-hype-playbook-unhashed-ceo-mia-p-on-marketing-web3-credibility), though there are signs that is [settling](https://www.linkedin.com/pulse/web3-fintech-2025-beyond-making-waves-transforming-industries-jelic-2fhzf) now that the serious builders are the ones left. Decentralization as a principle became hard to even discuss through the noise. Sound [familiar](https://www.theatlantic.com/culture/archive/2025/06/artificial-intelligence-illiteracy/683021/?gift=a488bXrqvMlx1958JHI5qDnArF6wxd8fux6Y1VNDFMc)? Looking at you, AI discourse.

I still think decentralization is worth pursuing. What we don't need is a thousand crypto startups repeating the same idea, or companies racing to own a specific corridor of Web3 infrastructure. What we need is a different relationship with centralization itself.

Our economy rewards resource acquisition and scarcity so heavily that you can predict who will still be around in ten years by measuring the depth of their pockets today. That is obvious for consumer-facing services like search engines and operating systems, but it hits developers just as hard. Need a server? There are about four real options, and maybe six more if you know exactly what you want. Need AI infrastructure? You can count the reliable providers on your hands. Need DDoS protection and a CDN? There's Cloudflare, and then there are technically alternatives.

It is difficult to picture a version of this industry where that concentration doesn't happen, especially now that everything is a service. Infrastructure, platform, software: the whole model rewards consolidation, because economies of scale matter enormously in infrastructure.

## The Cloudflare Problem

Cloudflare [isn't really a monopoly](https://dev.ua/en/news/chy-ie-cloudflare-monopolistom-iz-zakhystu-saitiv-vid-atak-ni-os-piatirka-alternatyv-1763473659), but that is mostly a semantic argument. Competitors exist and customers can technically switch. They are also growing large enough that it's worth [asking what their role in internet infrastructure means long term](https://version-2024-2.goauthentik.io/blog/2023-02-07-cloudflare-is-destroying-the-open-internet).

Here is where it gets awkward. We want a provider that is genuinely good at what it does, and Cloudflare is. They aren't [leaning on their position to keep competitors out](https://fortune.com/2025/11/13/cloudflare-ceo-google-abusing-monopoly-search-ai/). They're popular for legitimate reasons: the services work, they're often free for small sites, they're fast, and the engineering has generally earned trust.

But those individual decisions add up. When everyone independently concludes that Cloudflare is the best option, we collectively build a single point of failure. And when you try to convince someone to choose differently, you run straight into an incentives problem.

### Individual rationality, collective risk

The [Consumer Dilemma](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115) describes situations where consumers have to choose against their immediate interests for a broader benefit. Usually these come up as ethical questions: should I avoid a company with harmful production processes even though the alternative costs more? Should I boycott a company whose products are better than everyone else's?

Those are relatively easy to engage with, because the harm is concrete and traceable. It is not hard to understand why more plastic means more environmental damage.

Technology is different. The product is usually far enough removed from its effects that the harm is hidden, or just hard to connect. And even when you can [make the connection](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4590115), the benefits of new technology tend to require that you keep consuming or fall behind. The effect gets worse when the product is intangible, when the harm is speculative, when it lands on "the internet" rather than on you, and when it seems inconsequential at your scale.

Say you're picking a hosting provider. BigCloud has 99.99% uptime, fast global performance, excellent documentation, a free tier, and 60% of the internet already on it. SmallCloud has 99.9% uptime, good regional performance, decent documentation, and 2% of the internet.

The rational choice for you is obvious. BigCloud is better on nearly every metric you care about right now, and the fact that it already serves most of the internet doesn't make your site slower. If anything, that scale is part of why it's good.

But if everyone reasons this way, 80% of the internet ends up on one provider, and when that provider goes down, 80% of the internet goes with it. The harm never shows up in the individual choice. It only exists in aggregate.

### Why the warning never lands

When a provider is doing everything right, convenience is a powerful pull and monopoly concerns feel abstract. The argument against centralization ends up sounding like this:

> "I know this is the best provider of Thing A right now, but don't use it, because one day, if many people use this objectively better product, a monopoly might form and..."

And then what? That's the part I want to sit with.

The argument fails for four reasons, and they compound. The harm is hypothetical: your service *might* go down someday. The alternative is certain: the inferior service *will* be slower today. The collective harm isn't personal: if Cloudflare goes down, everyone goes down, so you aren't uniquely disadvantaged. And history contradicts the warning: Cloudflare has a better outage record than most of the alternatives you'd switch to.

That last point is worth pulling on.

## Good Service Enables Dangerous Concentration

Consumers and providers have a trust relationship that runs on faith in future performance based on past behavior. It is hard to claim a company *will* become a bad monopoly when the evidence points the other way. Cloudflare gives away service to small sites, publishes detailed post-mortems, has taken public positions on free speech, and is generally transparent about how it operates.

It's just as hard to make the case about unintentional failure. The risk isn't unique to large providers, since small ones fail more often. The large provider's track record is better, which is how it got large. And individual customers have misaligned incentives anyway: if everyone's service goes down, your users won't blame you.

So you get a perverse dynamic. The better a company is at providing a service, the more we centralize around it. The more we centralize, the worse its eventual failures are.

Think of it as bathtubs. Many small providers means many small bathtubs, and one springing a leak is annoying for the people using that one. Centralizing means one enormous bathtub, and when it drains, it's a global event. The giant bathtub is genuinely better built, with better materials and more redundancy, and less likely to leak. But every system fails eventually, and when this one does the consequences are orders of magnitude worse.

We have optimized for the average case at the expense of the worst case.

## So What Do We Do?

Honestly, I'm not sure. If I were, I doubt I could explain it here without either getting very technical or proposing that we rebuild fundamental economic structures.

What I can do is explain what drew me to Web3 in the first place, because I think the underlying idea survived the implementation.

I liked the glimpse of a world that leaned toward ownership and collaboration rather than rent-seeking and dependency: networks where no single failure cascades globally, services that eventually shape how hardware gets built and sold in favor of interoperability, and an AI market where innovation is genuinely costly rather than [artificially cheap until the market is captured](https://arxiv.org/abs/2405.21015).

That last one deserves expansion. The standard playbook is to operate at a loss to gain market share, undercut competitors until they can't compete, achieve dominance, then raise prices or let quality slip while network effects and switching costs hold everyone in place. It's economically rational under current structures. It is also [openly intended to monopolize markets](https://arxiv.org/abs/2405.21015), and we act surprised when it works.

Here's the part that bothers me most. Personal photos and videos of my life could be lost because of a missed payment to a cloud storage provider. The business model for most services is built so that consumers [pay more than they could ever consume](https://dl.acm.org/doi/abs/10.1145/3366423.3380281), extracting revenue through subscriptions for resources that sit idle. I don't own my data. I rent access to it, forever, from companies that can change the terms or shut the service down.

We don't accept this anywhere else. Imagine a bookshelf that charged rent per book, and emptied itself when you stopped paying. It sounds absurd, and it's the model we've accepted for everything digital.

I would like our approach to new technology to focus on accessibility rather than perpetual reliance. Infrastructure designed for resilience instead of efficiency alone, accepting that some redundancy is necessary rather than wasteful. Protocols over platforms, with interoperability as the default. Real ownership of digital goods. Cooperative models where the people who depend on infrastructure have some stake in it.

The problem is that these models struggle against VC-funded companies willing to lose money for years to win a market. The incentives point away from the outcomes we want.

## Better Questions

I don't have complete answers, but I think there are better questions to be asking.

If you're a developer, ask whether you can reasonably diversify your dependencies. Even if Cloudflare is the best option, can you architect things so an outage there doesn't take you out entirely? Ask whether you're making the convenient choice or the resilient one, because they don't always align. Ask what you'd actually be willing to give up: slower performance, higher costs, more complexity?

If you're a provider, ask whether you're being honest about your role. Cloudflare's post-mortems deserve credit, but transparency alone doesn't solve concentration. Ask whether your service degrades gracefully or fails catastrophically. And ask what obligations come with your position, because once you serve a majority of the internet's traffic you aren't just a company anymore.

At the policy level, the question is whether critical internet infrastructure should be treated the way we treat utilities and telecoms. How do you incentivize resilience when markets optimize for efficiency and resilience looks like waste right up until it isn't? And what does meaningful decentralization actually look like, if not a thousand cryptocurrency startups?

## Living With Fragility

The Cloudflare outage, and the three others this month, aren't anomalies. They are what you get from an architecture that optimized toward centralization because every incentive pointed there.

We built a system that is fast and reliable almost all of the time, and we did it by concentrating risk so that the rare failures are catastrophic. That might be exactly what we collectively chose, through millions of individually rational decisions that produced an irrational whole. We wanted convenience, performance and low cost. We got all three, and the bill arrives as periodic internet-wide outages.

Whether we're willing to pay a different price is still open. But it's worth asking seriously, because the alternative is continuing to act surprised.

The internet didn't crash because of an `.unwrap()` in some Rust code. It crashed because a single company's mistake can break global infrastructure, and we built it that way ourselves.
