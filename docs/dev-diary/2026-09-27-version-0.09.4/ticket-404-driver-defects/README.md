# The defects the headless driver shows (ticket #404)

[Ticket #404](https://github.com/whaleyjoshua2/Dying-Earth/issues/404); the spec is §1 of
[`docs/spec/version-0.09.4.md`](../../../spec/version-0.09.4.md). No rule moved, so no sweep.

**What the driver says now** is in [`driver-run.txt`](driver-run.txt), from a fresh Custodians game
at seed 7: Build Army's confirm naming its Widgets and its million people, a Factory's its Widgets,
a refused turn reading *"The turn did NOT end:"*, the Market line on the board, and Army rows
printing *seat 0* and *seat -*.

**Four tests were watched red before the change**, against stubs that returned the old text:

- `a_confirm_names_everything_an_army_pays`: `left: "25 Materials"`, `right: "25 Materials, 4 Widgets and 1M people"`.
- `a_move_that_spends_the_tank_says_so`: `left: "free"`, `right: "6 Fuel from the tank"`.
- `the_market_line_names_a_price_a_card_has_set`: no *"Materials 1 (Cheap Ore Offer, 1 more turn)"* in the line.
- `a_change_of_hands_to_or_from_the_player_is_also_under_your_works`: *a gain is under Your works* failed.

**The review** (an agent that did not build it) found the Market line printed between *Last
income* and its *from:* line, and twice under `show --costs`; both fixed. It found two cases no test
guarded, and two more tests were watched red against a build broken on purpose (a throw-off left
unmarked, a Ducat buy given Widgets) before they passed:

- `a_confirm_names_no_widgets_for_a_ducat_buy_and_a_launch_is_free`: *a Ducat buy starts done*.
- `a_throw_off_of_the_player_is_under_your_works`: the player's throw-off line unmarked.

**No picture.** The window's two changes are the list of the turn's orders, which reads the same
engine text as the driver's confirm, and the Report's Your works heading, which reads the same
`sections()` the fourth test checks. A player's change of hands cannot be staged in `shot:` mode
without a new aid, so the Report's heading was checked through the test and the driver's Report.
