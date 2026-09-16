# The Faction selection screen at three sizes — ticket #217

Captures taken while grilling [The Faction selection screen: four cards of one height, and a Play Tutorial
tick a fifth larger](https://github.com/whaleyjoshua2/Dying-Earth/issues/217), on map
[#215](https://github.com/whaleyjoshua2/Dying-Earth/issues/215).

**These are the game, not drawings.** Real headless `shot:` output from `main` at commit `5a2f2ac`, the
merge of version 0.08.1.

| file | size | what it shows |
|---|---|---|
| `at-1280x800.png` | 1280x800 | The default window size. **Row 2 is sliced mid-sentence and neither the Arkwrights' nor the Archivists' Play button is visible at all.** |
| `at-1600x900.png` | 1600x900 | The same failure, less severely: row 2 is cut below the Archivists' Victory Condition, and again neither Play button is reachable without scrolling. |
| `at-1920x1080.png` | 1920x1080 | All four cards fit, all four Play buttons visible, with about a hundred rows to spare. |

## What the captures say that the ticket did not

The ticket read the designer's *"the cards should be symmetrically presented in a 2X2 matrix"* as a request
to **equalise card heights**, on the grounds that `ui.columns` lets each column find its own height. The
captures show a larger problem underneath that reading:

- **At two of the three sizes, half the Factions cannot be chosen without scrolling.** The screen is inside
  a vertical `ScrollArea`, so the buttons are reachable — but nothing on screen says so, and the first
  impression is a screen where two Factions have no button.
- **The game opens maximised** (`src/main.rs:89`, ticket #165), so what a player actually sees is their own
  screen size. 1920x1080 is fine; a 1366x768 laptop would be worse than the 1280x800 capture here.
- **Equal heights alone would make the small sizes worse**, since every card would take the height of the
  tallest — the Custodians', which is the longest rulebook and the only one carrying the `Play Tutorial`
  tick.

So the raggedness the designer noticed is real, and visible at 1920x1080 in two forms — the Play buttons
sit at different heights within a row (the Custodians' at 433, the Prospectors' at 454), and the row-2
card frames are unequal (the Arkwrights' ends about 40 rows below the Archivists'). But the fitting
problem is the one that costs a player something.
