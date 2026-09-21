# 2026-09-21: version 0.08.6, the defence version

The dev diary for version 0.08.6, charted as wayfinder map
[#289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289). As 0.08.5's, this folder is a
record of the **build**: one folder per ticket, written as each was resolved, holding the pictures
and the measurements that decided it. The rules as decided are in
[`docs/spec/version-0.08.6.md`](../../spec/version-0.08.6.md), written by the closing ticket.

**Every folder here is listed.**

| folder | ticket | what is in it |
|---|---|---|
| [`sweeps/`](sweeps/) | [Write the 0.08.6 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/301) | `final-0.08.6.txt`: the closing sweep, 20 seeds × four seatings at the shipped climate cell. Win column 43 / 21 / 2 / 0 against 28 / 26 / 9 / 2, collapses 14 of 80 against 15, five marches on a held Region against 91. |
| [`../../playtest/PLAYTEST.txt`](../../playtest/PLAYTEST.txt) | [Write the 0.08.6 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/301) | The playtest note that ships in the kit as `README.txt`. It lives at `docs/playtest/` while this is the current version, because the kit workflow copies it from there; 0.08.5's is filed under [`playtest/`](../2026-09-20-version-0.08.5/playtest/PLAYTEST.txt) in its own folder. |
| [`ticket-290-starting-people/`](ticket-290-starting-people/README.md) | [Two Colonists on every starting station, and two Pioneers for the Arkwrights](https://github.com/whaleyjoshua2/Dying-Earth/issues/290) | The ISS's card with two aboard and two free tiles, China's card with two Pioneers waiting, the two new tutorial notes; a batch and a control that found the computer's opening was not an opening until it was made to fire once. |
| [`ticket-291-building-in-its-box/`](ticket-291-building-in-its-box/README.md) | [A building shown in its box the moment it is ordered](https://github.com/whaleyjoshua2/Dying-Earth/issues/291) | A Factory ordered on China's card, the Power Plant the turn after as *building, 1 turn*, a Habitat ordered on the ISS. |
| [`ticket-292-trading-window/`](ticket-292-trading-window/README.md) | [The Trading window opens below the top bar](https://github.com/whaleyjoshua2/Dying-Earth/issues/292) | Trading and Victory under the bar, both beside the Faction window, the bar with its Pick a Tech button, the Tech Tree at 800 and 1400 pixels, the Climate Panel. |
| [`ticket-293-smear-greenwash-sliders/`](ticket-293-smear-greenwash-sliders/README.md) | [The Smear and the Greenwash on sliders](https://github.com/whaleyjoshua2/Dying-Earth/issues/293) | The Smear rail on the Prospectors' page, the Greenwash rail with its Ducat bound, the rail greyed end to end once Max has taken the Allotment. |
| [`ticket-294-command-cluster/`](ticket-294-command-cluster/README.md) | [The command cluster: a slider for the spend, Max kept, a tenth larger, and End Turn a sphere on the right](https://github.com/whaleyjoshua2/Dying-Earth/issues/294) | The cluster with China selected, with nothing selected, with a Tech pick owed (the sun as embers), with Max standing. |
| [`ticket-295-296-defence-and-disengage/`](ticket-295-296-defence-and-disengage/README.md) | [The disengage chance nudged down](https://github.com/whaleyjoshua2/Dying-Earth/issues/295) and [A Region's defence is its people](https://github.com/whaleyjoshua2/Dying-Earth/issues/296) | China's and Egypt's Army rows under the first defence rule; the escape baseline; the batch that found the two people's points stopped the computer's ground war, and the two controls that attributed it. |
| [`ticket-297-dig-in/`](ticket-297-dig-in/README.md) | [Dig In: a third live stance for Armies](https://github.com/whaleyjoshua2/Dying-Earth/issues/297) | The stance row with Dig In, the map with trench lines under every neutral shield, Egypt's row *dug in: +2 defending*. |
| [`ticket-298-take-it-whole/`](ticket-298-take-it-whole/README.md) | [A place taken whole](https://github.com/whaleyjoshua2/Dying-Earth/issues/298) | No picture; a batch in which nothing was taken by force. |
| [`ticket-299-occupation-held/`](ticket-299-occupation-held/README.md) | [An Occupation that must be held](https://github.com/whaleyjoshua2/Dying-Earth/issues/299) | No picture; a batch in which no Occupation began or broke. |
| [`ticket-300-landing/`](ticket-300-landing/README.md) | [A landed Army may Attack on the turn it lands](https://github.com/whaleyjoshua2/Dying-Earth/issues/300) | No picture; the landing baseline and the batch after, no Army landed in either. |
| [`ticket-302-one-army-system/`](ticket-302-one-army-system/README.md) | [One Army system](https://github.com/whaleyjoshua2/Dying-Earth/issues/302) | China's row at *strength 4, damage 0/4, defends at 5*, Egypt's at *strength 2, damage 0/2, defends at 5, dug in*, the shields at the bare strengths; the batch in which a little of the ground war returns. |

Everything here was captured headlessly in `shot:` mode. **Nothing was opened on the designer's
desktop.**

## What the pictures changed

Three times in this version a picture found something reading the code had not. **The Facilities
header still counted an ordered slot as free** ([#291](https://github.com/whaleyjoshua2/Dying-Earth/issues/291)):
the first capture read *5 of 9 slots free* over four free boxes, and the header now subtracts the
order. **The Tech Tree opened over the bar** ([#292](https://github.com/whaleyjoshua2/Dying-Earth/issues/292))
once its guessed top was retired, because its scroll bound was summed from that guess; the bound is
measured where the tree starts now. And **the Arkwrights' Pioneers line was below the fold**
([#290](https://github.com/whaleyjoshua2/Dying-Earth/issues/290)) on an 800-pixel window, which is a
fact about the card, not a defect, and the picture was retaken taller. Nothing in the suite reads a
pixel.

## What the batches found

- **The computer's opening was not an opening** until it was made to fire once
  ([#290](https://github.com/whaleyjoshua2/Dying-Earth/issues/290)): written as "push the Habitat
  while the station has room under four", it fired every time the muster refilled the station, and
  the Prospectors put Habitat after Habitat on Tiangong for 330 Ducats a game against 2287.
- **Two points of defence stopped the computer's ground war outright**
  ([#296](https://github.com/whaleyjoshua2/Dying-Earth/issues/296)): marches on a held Region from
  18 to 0 on one seating, and a control with the points at nought brought them back. The designer
  answered with the one-Army ticket, which moved the points off the hit points; a little of the war
  returned, and over the closing sweep the computer marched five times in eighty games where it
  marched 91.
- **Nothing on the computer's board exercises three of the rules**: no Occupation broke, no Army
  landed at a Colony, no neutral was threatened. They are a human's until the balance version.
- **The win column moved to the Custodians, 43 of 80.** The Prospectors' war paid, and it is gone.
