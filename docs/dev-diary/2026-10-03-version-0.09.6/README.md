# Version 0.09.6, the growth version

The map is [Map: version 0.09.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/435), and the
spec is [`docs/spec/version-0.09.6.md`](../../spec/version-0.09.6.md).

## The crash on a Colony Ship at a Venus station (ticket #436)

Taken headlessly with the new `venusstation:1` aid: a station of seat 0's at Aphrodite over Venus,
and a Colony Ship of theirs with four Colonists in its ring. The command was
`shot:<prefix> venusstation:1 stack:venus ship:1 seed:7`.

- **Red, before the fix:** the same command exited 101 with
  `panicked at src\ui.rs:8450:203: index out of bounds: the len is 0 but the index is 1`. That is
  the panic the designer's save gives.
- [`card.png`](ticket-436-venus-crash/card.png), **after the fix:** TSV Endeavour's card at
  Aphrodite. Under Load and unload, a slider at 4 and "Unload 4 Colonists into Aphrodite".
