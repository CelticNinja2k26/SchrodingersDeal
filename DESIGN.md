# Schrödinger's Deal — Design Plan (no-code)

Status: planning. No Rust written yet — this document is the source of truth.

## Project principle
**Stephanie writes the code.** The AI specs, explains, and reviews.
This is a learning project — the struggle is the point.

## Vision
Deal or No Deal with quantum flavor: case contents are not merely hidden,
they are *undetermined until observed*.

## Core mechanic — the observation bag
- The prize ladder exists as a **vector of values**.
- When a case is opened, **pop one value at random** from the bag and show it.
- A case literally has no value until the player observes it.
- Constraint: **no replacement** — values are never added/removed/swapped
  mid-game. The bag stays honest so players can reason about odds
  (same probability structure as classic DOND, more Schrödinger flavor).
- The player's own case is also undetermined: its value is popped
  from the bag at final reveal.

## Win condition
Classic DOND: open cases round by round, receive banker offers,
take the deal or keep going to the end.

## Prize ladder (classic show, 26 values)
$0.01 · $1 · $5 · $10 · $25 · $50 · $75 · $100 · $200 · $300 ·
$400 · $500 · $750 · $1,000 · $5,000 · $10,000 · $25,000 ·
$50,000 · $75,000 · $100,000 · $200,000 · $300,000 · $400,000 ·
$500,000 · $750,000 · $1,000,000

## Banker offer algorithm (TV-style)

Chosen flavor: the TV banker — climbing generosity, may even beat fair
value near the end (the drama is the game). Grounded in Ritcey & Ranjan,
"Statistical Models for the Banker's Offer in Deal or No Deal" (2010),
fitted on real NBC TV games.

**Model: `Offer = p(round) × FairValue + noise`**

Steps, each end of round:

1. **FairValue F** = mean of all still-unobserved values.
   With our bag: the player's case hasn't popped yet either,
   so `F = mean(remaining bag)`. (One line of arithmetic.)
2. **Look up p** for the round (fitted TV medians):

   | Round | p |
   |---|---|
   | 1 | 0.17 |
   | 2 | 0.27 |
   | 3 | 0.40 |
   | 4 | 0.51 |
   | 5 | 0.62 |
   | 6 | 0.74 |
   | 7 | 0.83 |
   | 8 | 0.93 |
   | 9 | 0.96 |

   (Classic 26-case round shape: 6-5-4-3-2-1-1-1-1 → 9 rounds.)
3. **Raw offer** = p × F.
4. **Noise** — the banker is moody. Paper models ε ~ N(0, σ²) with σ
   growing late. Simple version: uniform random ±5% of raw offer,
   growing to ±10% in the last three rounds.
5. **Clamp** to [lowest unobserved value, highest unobserved value].
   (Observed from real data: offers stay inside the remaining range.)
6. **Round pretty** — real offers look like $42,000, not $42,318.77.
   Suggestion: nearest $1,000 above $10,000, nearest $100 below.
7. Present: "Deal or No Deal?"

### Rules the banker must NOT break
- The offer only *reads* the bag. It never pops or mutates it.
- Early offers must be below fair value; only the late game may exceed it
  (TV data: overpaying happens only in the final ~3 rounds).
- Taking a deal ends the game and awards the offer amount.
- Rejecting all nine offers → pop the player's case → win that value.

### If the case count ever changes (compact mode)
Don't hardcode round numbers — map the TV curve by game progress
(fraction of cases opened) and interpolate p. Keeps the feel at any size.

## Decisions so far
- [x] **Two game modes: Schrödinger's Deal (primary) and Classic DOND.**
  Both run on ONE engine — bag-pop without replacement is
  probabilistically identical to hidden pre-assignment, so modes
  differ by features/flavor, not math. Classic = no spies, show-style
  reveal language; Schrödinger = spies available, observation
  language ("the case resolves to...").
- [x] Value resolution happens at open-time (bag pop), not pre-assigned
- [x] Classic DOND structure (banker offers, take-it-or-keep-going)
- [x] Banker flavor: TV-style (Ritcey–Ranjan fitted schedule above)
- [x] Twist ideas deferred to post-basic-game
- [x] Stephanie writes the Rust; AI specs and reviews
- [x] **Universe philosophy: nothing knows any value before observation** —
  not the player, not the banker, not the spies. Spy clues are
  retroactive constraints (weak measurements): the final draw is
  made consistent with all clues. Truth is created by observation,
  not revealed by it. No hidden state anywhere.

## Parking lot (future twists)
- **THE LEAD TWIST — spies/leakers** (don't hold her to it, but it's good):
  - 1 banker + a **user-selectable number of spies** (leakers/informants).
  - Spies never reveal exact values — only **inequalities**
    ("greater than $50,000", "less than $12,000"), about
    the **player's own case** (confirmed).
  - **Every clue must be strictly informative**: it must shrink the
    interval of possible values. A weaker bound than a previous clue
    adds nothing and is rejected (spy tries again).
  - Clues must also be consistent: a bound is only legal if some
    unobserved bag values satisfy it and some don't.
  - Quantum reading: spy clues are **weak measurements** — partial
    collapse that shrinks the box without opening it.
  - Implementation hook: all clues define [lo, hi]; each new clue
    must strictly refine it; final case value drawn consistent
    with the full interval.
  - Open: does the banker hear the spies? (a provably-high case
    could trigger a lowball or a bidding war)
- Mid-game "quantum event" chaos (careful: breaks odds reasoning)
- Case swap offer at the end (classic show twist)

## Open questions
- [x] Case count: **26** (original/standard).
- [x] Prize ladder: **classic show ladder**, $0.01 → $1,000,000 (all 26 values).
- [x] Round structure: **classic 6-5-4-3-2-1-1-1-1** (9 rounds, TV banker
  schedule applies as-is).
- [x] Presentation: terminal text UI (first version).

**The plan is complete. Ready for Rust.**
