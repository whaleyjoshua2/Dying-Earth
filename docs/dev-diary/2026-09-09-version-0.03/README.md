# 2026-09-09: version 0.03, ticket by ticket

Work on map [#30](https://github.com/whaleyjoshua2/Dying-Earth/issues/30), on the branch `version-0.03`.

## #31: six housekeeping items

1. **Only Climate cards scale with the Temperature**, as before; the Event popup now says which it
   was ("A Climate card: x1.35 at this Temperature" or "Not a Climate card: the Temperature does not
   change it"), and a test pins a Solar, Discovery and Failure card to a scale of 1 at +3.0 C.
2. **Hover a resource in the top bar** for last Income by source: each building with its state or
   Colony and its amount, upkeep as negatives, Ships and Armies as one line.
3. **A place taken by Influence keeps every Facility and Module.** The 1-in-4 destruction roll now
   fires only when a place is attacked and when Occupation transfers it. A test takes Africa with
   eight Factories on five seeds; all eight stand each time.
4. **Free build slots** on every Earth Map label ("4 Facilities, 0 free slot(s)") and at the head of
   the card's Facilities list ("Facilities (2 of 5 slots free)").
5. **End Turn asks once when Influence is unspent**: "12 Influence is unspent; it is lost at End
   Turn. End the turn anyway?" with End Turn and Back.
6. **Army shields on the Earth Map**: one per Faction with Armies in a state, in the Faction's
   colour, grey for a neutral state's Standing Army, the stack's strength written on it, clickable.

![Turn 7: shields under Europe (Prospectors, 5), the Middle East (neutral, 3) and Africa (neutral, 2); labels with free slots](housekeeping.png)

Both rule changes were watched red first: with the destruction roll restored for Influence the
eight-Factory test failed, and with every card scaled the Meteor Shower test failed.
