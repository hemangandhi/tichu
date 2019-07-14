# Tichu

See [online](https://boardgamegeek.com/boardgame/215/tichu) for the game itself.

The question here is "why would you ever call grand Tichu?" I want to find out
experimentally.

## TODO

- [ ] Finish up probability calculations
- [ ] efficiently play good hands in various circumstances
- [ ] how to help out a partner?
- [ ] play a game taking points into consideration

## Future thoughts

Implement the [step trait](https://doc.rust-lang.org/std/iter/trait.Step.html) for the value and suit enums.
(Particularly the value one: it's `distance_to` function is perfect for it.)
I'll wait until it's stable...
