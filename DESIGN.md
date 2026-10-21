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
- [x] Value resolution happens at open-time (bag pop), not pre-assigned
- [x] Classic DOND structure (banker offers, take-it-or-keep-going)
- [x] Banker flavor: TV-style (Ritcey–Ranjan fitted schedule above)
- [x] Twist ideas deferred to post-basic-game
- [x] Stephanie writes the Rust; AI specs and reviews

## Parking lot (future twists)
- **Two bankers** (good-cop/bad-cop, argue about the offer)
- Mid-game "quantum event" chaos (careful: breaks odds reasoning)
- Case swap offer at the end (classic show twist)

## Open questions
- [ ] How many cases? (26 classic, or smaller for terminal UI)
- [ ] Prize ladder — amounts and top prize
- [ ] Round structure confirmed once case count is set
- [ ] Presentation: terminal text UI?
