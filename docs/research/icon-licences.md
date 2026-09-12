# Icons and art for the resources, under licences that permit use

Research for ticket [#102](https://github.com/whaleyjoshua2/Dying-Earth/issues/102), version 0.07.0.
The designer's line: *"look for open source icons and art with permissive licenses to use as
placeholders for the resources and use them in popups"*.

What the game needs symbols for: **Materials, Fuel, Energy, Research, Ducats**, and ideally the
Facilities, Modules and Bodies.

## What the game can actually load

`assets/textures/` holds **PNG** only (`earth.png`, `mars.png`, `earth_states.png` and so on), read
through the `image` crate at version 0.25 and handed to `bevy_egui`. **Nothing in the build renders
SVG.** Any SVG set therefore needs a rasterising step before it can be used, which is a real cost to
weigh and not a detail: it is the difference between dropping files into a folder and adding a
conversion to the build.

## The candidates

### 1. game-icons.net — the only set that has the right subjects

- **Licence: CC BY 3.0.** Not CC0.
- **Attribution: required, and visible.** The site's own suggested form is
  `Icons made by {author}. Available on https://game-icons.net`. The icons have many different
  authors (Lorc, Delapouite and others), so a credit has to name the ones actually used.
- **Commercial use: yes. Modification: yes** — recolouring to a Faction's colour and resizing are
  both permitted.
- **Format: SVG**, with PNG rendering offered through their own "Studio" tool rather than as a bulk
  download.
- **Coverage: excellent, and the reason it leads.** It is organised by concept and runs to thousands
  of icons; the *energy* tag alone holds 62. Minerals, canisters, flasks, coins and the rest of this
  game's vocabulary are all present.

**The product consequence:** the game has **no credits screen**. Taking this set means adding one,
or putting the credit somewhere equally visible. That is a design decision, not just a legal box.

### 2. Kenney — the safest licence, but the wrong contents

- **Licence: CC0.** **Attribution not required** ("if you choose to credit… refer to 'Kenney'").
  Commercial use explicitly allowed.
- **Format: PNG**, among others — no rasterising step.
- **Coverage: does not fit.** The obvious pack, *Game Icons* (105 icons, 2014), is **gamepad and
  interface prompts** — joysticks, buttons, key glyphs — not resource symbols. Kenney's strength is
  sprites, UI panels and 3D kits, and there is no resource-icon set of the kind this ticket wants.

Worth knowing anyway: if the interface later needs buttons, panels or prompt glyphs, this is the
no-strings answer.

### 3. OpenGameArt CC0 collections — free of strings, uneven in quality

- **Licence: CC0** on the collections filtered that way, so no attribution.
- **Format: PNG**, typically small raster (the currency set is 32×32).
- **Coverage: piecemeal.** Individual contributed sets rather than one coherent family, so the five
  resources would likely come from different hands and not sit together. Small raster art also does
  not scale up into a popup cleanly.

### 4. freegameui.net — claims 2,000+ CC0 game UI SVGs

**Unverified.** The site returned HTTP 403 and its licence could not be read first-hand, so it is
recorded here as a lead and not as a recommendation. If the CC0 claim holds it would be the ideal
answer — CC0 *and* the right subjects — and it is worth one manual look.

## The shape of the choice

It comes down to a trade the designer has to make, not a fact to look up:

| | licence | credit needed | format | has the right subjects |
|---|---|---|---|---|
| game-icons.net | CC BY 3.0 | **yes, visible** | SVG (rasterise) | **yes** |
| Kenney | CC0 | no | PNG | no |
| OpenGameArt CC0 | CC0 | no | small PNG | piecemeal |
| freegameui.net | claims CC0 | unverified | SVG | unknown |

**Recommendation: game-icons.net**, and accept that it brings a credits screen with it. It is the
only set measured here that actually carries Materials, Fuel, Energy, Research and Ducats as a
coherent family, and a credits screen is a thing the game will want eventually in any case. The
rasterising step is one-off: render the handful of icons chosen to PNG at the size the popups want
and commit them beside the existing textures.

**If a credits screen is unwanted**, the honest answer is that no CC0 set measured here covers the
need, and the fallback is either commissioning or drawing five simple glyphs — which, for
placeholders, is a smaller job than it sounds.

## Sources

- [game-icons.net — About and licence](https://game-icons.net/about.html)
- [game-icons.net — the energy tag, 62 icons](https://game-icons.net/tags/energy.html)
- [Kenney — support and licence](https://kenney.nl/support)
- [Kenney — Game Icons pack](https://kenney.nl/assets/game-icons)
- [OpenGameArt — CC0 resources](https://opengameart.org/content/cc0-resources)
- [OpenGameArt — CC0 currency icons](https://opengameart.org/content/cc0-currency-icons)
- [freegameui.net](https://freegameui.net/) (unverified; returned 403)
