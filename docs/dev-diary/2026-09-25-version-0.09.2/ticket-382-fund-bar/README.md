# Ticket #382: a condensed fund slider and fill bar in the top bar

The designer: *"for factions that have victory funds I'd like to see a condensed version of the
slider and fill bar in the top bar (second row to the right of the window buttons)."* Decided in one
round of four: the widget as recommended; the Archivists and the Prospectors only; the row wraps at
a narrow width; the full controls stay.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/382) is the authority;
[§16 of the spec](../../../spec/version-0.09.2.md#16-a-condensed-fund-slider-and-fill-bar-in-the-top-bar)
records it.

## What was built

- **One placer per order**, `place_research_directive` and `place_venture_share`, cut out of the
  two full controls' tails and called by both the full and the condensed rails, so the top bar's
  rail and the window's rail are one control drawn twice; the refusal memory of #380 rides with
  them.
- **`condensed_fund`** on the top bar's second row, after the buttons: for the Archivists the
  research glyph, the contribution figure, a 120-px rail dimmed below the floor as the full one is,
  and the Archive fund against its 125 Research; for the Prospectors the ducats glyph, the share,
  the rail dimmed above the four-fifths cap, and the Venture Capital Fund against its 2,500. The
  hover carries each full control's sentence, which the two now share as constants.
- **`fund_bar`**: the fill bar, 120 by 14, the fund in the Faction's colour with *"fund of bar"*
  written on it, the shape of `research_race_bar`.

No engine change, no save change, no sweep.

## The pictures

Headless, `panel:0`, from this folder. Four, as the ticket asked: both Factions at both widths.

| picture | aids | what it shows |
|---|---|---|
| [`arch-earth.png`](arch-earth.png) | `player:archivists window:1920x1080` | **The Archivists at 1920**: right of *Back (Esc)*, the research glyph, *100%*, the rail, and *0 of 125*. |
| [`pros-earth.png`](pros-earth.png) | `player:prospectors window:1920x1080` | **The Prospectors at 1920**: the ducats glyph, *0%*, the rail with its unreachable fifth dimmed, and *0 of 2500*. |
| [`arch1280-earth.png`](arch1280-earth.png) | `player:archivists window:1280x800` | **At 1280**, the widget still on the second row after the buttons; nothing wrapped. |
| [`pros1280-earth.png`](pros1280-earth.png) | `player:prospectors window:1280x800` | The same for the Prospectors. |

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 503 + 6, root 8.
