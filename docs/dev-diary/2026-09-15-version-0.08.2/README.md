# 2026-09-15: version 0.08.2, the dealings version

The dev diary for version 0.08.2, charted as wayfinder map
[#215](https://github.com/whaleyjoshua2/Dying-Earth/issues/215). This folder collects the pictures and
measurements made **while the map's tickets were being grilled**, before the version's branch was cut, so
they are evidence behind decisions rather than a record of a build.

| folder | ticket | what is in it |
|---|---|---|
| [`ticket-216-ship-names/`](ticket-216-ship-names/mockups/README.md) | [A card's heading wears the Faction's glyph alone, and every Ship is listed by name](https://github.com/whaleyjoshua2/Dying-Earth/issues/216) | Three mockups of a per-hull map label, composited onto real headless captures, plus the captures themselves and the script that draws them. |
| [`ticket-217-faction-screen/`](ticket-217-faction-screen/README.md) | [The Faction selection screen: four cards of one height, and a Play Tutorial tick a fifth larger](https://github.com/whaleyjoshua2/Dying-Earth/issues/217) | The Faction selection screen captured at 1280x800, 1600x900 and 1920x1080. |

Everything here was captured headlessly in `shot:` mode against `main` at commit `5a2f2ac`, the merge of
version 0.08.1. Nothing was opened on the designer's desktop.

## What the pictures changed

Both sets of pictures corrected the ticket they were made for, which is the argument for making them
before the grilling rather than after:

- **Ticket 216** claimed the maps drew no Ship labels. The first capture showed **both maps already label
  Ships at a Body**, one line per Faction with a strength — so the question put to the designer changed
  from *whether* a map labels Ships to *whether that block becomes a per-hull list*. (The answer turned
  out to be neither: the designer meant Ships **in transit**.)
- **Ticket 217** was written about ragged card heights. The captures showed that at 1280x800 and 1600x900
  **two of the four Factions have no visible Play button at all** — the row is clipped and nothing says
  the screen scrolls. That is a larger problem than the raggedness, and it reframed the whole ticket.
