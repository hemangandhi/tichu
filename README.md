# Tichu

See [online](https://boardgamegeek.com/boardgame/215/tichu) for the game itself.

The question here is "why would you ever call grand Tichu?" I want to find out
experimentally.

## TODO

- [x] Finish up probability calculations
- [ ] efficiently play good hands in various circumstances
- [ ] how to help out a partner?
- [ ] play a game taking points into consideration

## Future thoughts

Implement the [step trait](https://doc.rust-lang.org/std/iter/trait.Step.html) for the value and suit enums.
(Particularly the value one: it's `distance_to` function is perfect for it.)
I'll wait until it's stable...

# Probability docs

This tries to be a bayesian bot: just updates the expectations it has based on all the probabilities to
maximise the score.

## Scoring Hands

One of the first things is, given a hand, and the set of cards out, what's the odds that the hand is beaten?
If `b` is this probability, we know that taking the lead is of probability `1 - b`. Hence, a basic hearts player
is born.

If we restrict to one opponent, we have that
`b' = # of hands that beat * ncr(n = other unseen cards, r = cards left in hand / ncr(n = unseen cards, r = hand size)`.
If the hand is not a bomb, we can just sum on the two bomb cases (thankfully they're mutually exclusive).

Doing more might not be wise: the notion of a partner and other things would come in the way.

## Expected Values of Plays

At the end, the bot should seek to maximise score, so the filtering algorithm would need a mechanic to play for
other players as well. Then, simply, the bot would execute a search, assuming that everybody plays like it, and
tries to maximise its own reward.

In theory, the search cannot be deeper than 56!.
