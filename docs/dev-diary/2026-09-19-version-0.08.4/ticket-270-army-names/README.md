# Names for Armies

Ticket [#270](https://github.com/whaleyjoshua2/Dying-Earth/issues/270) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`roster-army-row.png`](roster-army-row.png) | `shot: panel:0`, the roster's Armies row on turn 1, magnified: **the 1st Chinese Army, at China: strength 4 (Hold)** -- where it read *China: strength 4 (Hold)*. |
| [`china-card-armies-1080.png`](china-card-armies-1080.png) | `shot: select:eastasia panel:0 window:1920x1080`, the lower half of China's card: **the 1st Chinese Army (Custodians, standing): strength 4, damage 0/5**, then the Build Army button that would raise *the 2nd*. |

## What was decided, in the designer's words

- *"2"* -- a name **from the home**, not from a list: an ordinal and the Region's demonym, *the 1st
  Chinese Army*, *the 2nd Chinese Army*; a Colony's is its Garrison, *the Tycho Garrison*, *the 2nd
  Tycho Garrison*. No randomness is drawn, so the sweep's seeds stand.
- *"standing armies too"* -- the Standing Army is the first raised and so the 1st; a Standing Army
  raised again after it is destroyed takes the next number, since a Region counts every Army it has
  ever raised (`armies_raised`, saved).
- *"yes"* -- an Army may be renamed: the name is a field on the Army, and the rename ticket's
  control will reach it.
- *"those"* -- `demonym` on every Region's card in `nation_states.toml`: Nigerian, Egyptian,
  Chinese, Indian, Indonesian, Australian, European, American, Mexican, Brazilian, Russian,
  Iranian, Japanese, Saudi.
- *"everythere the army is named"* -- the roster, both cards' Army lists, the Army orders block,
  and the Carrier's cargo line all read the name. The Report's own lines (*{faction} Army moved
  from {from} to {to}*) still say *Army*; they are the next place to reach.

The name is the Army's, not any Faction's, and survives every change of hands -- the 2nd Chinese
Army is still Chinese under the Prospectors. Every Army is now raised through one door,
`Game::raise_army`, which names it; the Standing Army's spawn and the build queue both go through
it. A save from before this version reads the old form, *the Standing Army*, until its Armies are
replaced.

## Witnessed red

The field, the demonyms, the door and the test were put in place with the name left empty, and the
test run: *"the Standing Army is the first raised: left "the Standing Army", right "the 1st Chinese
Army""*. Then the naming went into the door; `332 passed`, clippy clean with `-D warnings`. Five
Army literals in the tests learned the new field.
