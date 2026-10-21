# Schrödinger's Deal

> Deal or No Deal where the cases don't just hide their values —
> they don't *have* values until you open them.

**Status: 🧪 planning / early development** — the design is complete
(see [DESIGN.md](DESIGN.md)), the Rust is being written by hand, on purpose.

## The idea

Classic Deal or No Deal, rebuilt around one rule borrowed from quantum
mechanics: **nothing exists until it is observed.**

The prize ladder lives in a bag. When you open a case, one value is
popped from the bag at random — and *that* is the moment the case
acquires its value. Not hidden. Not predetermined. Undetermined.
Your own case is the same: it resolves only when you (or the banker's
money) force the final measurement.

There are no spoilers, because there is nothing to spoil.

## Game modes

- **Schrödinger's Deal** *(primary)* — the full quantum experience,
  including spies (see below). Reveal language is observation-based:
  "the case resolves to $50,000."
- **Classic DOND** — the familiar show rules with traditional language,
  for purists and for learning the game. Both modes run on the same
  engine — the math is identical; only the flavor and features differ.

## The banker

Offers are computed from a real statistical model of the TV show,
fitted from hundreds of real games
(Ritcey & Ranjan, *Statistical Models for the Banker's Offer in
Deal or No Deal*, 2010):

```
Offer = p(round) × FairValue + noise
```

where `FairValue` is the average of all still-unobserved values and
`p` climbs from 0.17 to 0.96 across the nine rounds. Stingy early,
generous late — occasionally beating fair value at the very end,
exactly like the show's dramatic final offers.

## The spies *(roadmap twist)*

A user-selectable number of informants who leak hints about **your**
case — but never an exact value, only inequalities:

> *"It's greater than $50,000."*
> *"It's less than $12,000."*

Every clue must **add information**: each new bound strictly shrinks
the window of what your case can resolve to. A hint weaker than what
you already know is rejected and re-rolled. Technically, spies are
*weak measurements* — they shrink the box the cat is in without
opening it.

## The rules the universe obeys

1. **No replacement.** Values are never added, removed, or swapped
   mid-game. The bag stays honest so players can reason about odds.
2. **Nothing knows anything before observation** — not the player,
   not the banker, not the spies, not the code.
3. **Observations create truth.** Spy clues don't report a hidden
   fact; they constrain what the final observation may become.

## Project philosophy

This is a hands-on learning project: **the human writes the code.**
Design, specs, and review are assisted; implementation is deliberate,
manual, and the entire point. Expect tests alongside the code and a
slow, honest commit history.

## Built with

- Rust (terminal text UI)

## Development

```sh
cargo run
```

*(Not yet — the first commits are still being written by hand.
Watch this space.)*
