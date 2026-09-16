"""Mockups for ticket #216, question 4: should a map label a Ship by name?

Takes REAL headless captures of the Solar System Map and the Mars Body Surface Map and
composites candidate label blocks onto them, so label density and collisions can be judged
by looking rather than by describing. The captures are the game; the label blocks are drawings.

Run:  py -3.12 make-mockups.py <captures-dir> <out-dir>
"""

import os, sys
from PIL import Image, ImageDraw, ImageFont

CAPS = sys.argv[1]
OUT = sys.argv[2]
os.makedirs(OUT, exist_ok=True)

# The Faction colours as the game draws them (assets/data/factions.toml).
CUSTODIANS = (38, 166, 153)
PROSPECTORS = (217, 128, 51)
ARKWRIGHTS = (112, 70, 168)
ARCHIVISTS = (222, 82, 112)
GREY = (190, 190, 190)

FONT = "C:/Windows/Fonts/segoeui.ttf"
BOLD = "C:/Windows/Fonts/segoeuib.ttf"
SCALE = 2  # the crops are doubled so the text is legible in a committed picture


def font(px, bold=False):
    return ImageFont.truetype(BOLD if bold else FONT, px)


def crop(name, box):
    """A region of a real capture, doubled."""
    im = Image.open(os.path.join(CAPS, name)).convert("RGBA")
    c = im.crop(box)
    return c.resize((c.width * SCALE, c.height * SCALE), Image.LANCZOS)


def block(draw, x, y, lines, title=None, tail=None, line_h=17, pad=4):
    """A label block in the game's own style: a dark panel, a small glyph, coloured text.

    The leading glyph is a PLACEHOLDER square: the real one is the Kind Glyph or the
    Faction symbol, and this drawing is about how much room the lines take, not about art.
    """
    f = font(12 * SCALE)
    ft = font(12 * SCALE, bold=True)
    w = 0
    for _, text in lines:
        w = max(w, draw.textlength(text, font=f))
    if title:
        w = max(w, draw.textlength(title, font=ft))
    if tail:
        w = max(w, draw.textlength(tail[1], font=f))
    w = int(w) + 18 * SCALE + pad * 2 * SCALE
    rows = len(lines) + (1 if title else 0) + (1 if tail else 0)
    h = rows * line_h * SCALE + pad * 2 * SCALE
    draw.rectangle([x, y, x + w, y + h], fill=(12, 12, 12, 215), outline=(70, 70, 70, 255))
    cy = y + pad * SCALE
    if title:
        draw.text((x + pad * SCALE, cy), title, font=ft, fill=GREY + (255,))
        cy += line_h * SCALE
    for colour, text in lines:
        # the placeholder glyph
        g = 8 * SCALE
        draw.rectangle([x + pad * SCALE, cy + 3 * SCALE, x + pad * SCALE + g, cy + 3 * SCALE + g],
                       fill=colour + (255,))
        draw.text((x + pad * SCALE + g + 5 * SCALE, cy), text, font=f, fill=colour + (255,))
        cy += line_h * SCALE
    if tail:
        # Orbital Control rides INSIDE the panel, as the game draws it inside its own label group.
        draw.text((x + pad * SCALE, cy), tail[1], font=f, fill=tail[0] + (255,))
    return w, h


def caption(im, text, sub=None):
    """A strip above the picture saying which option it is."""
    head = 30 * SCALE if not sub else 48 * SCALE
    out = Image.new("RGBA", (im.width, im.height + head), (16, 16, 16, 255))
    d = ImageDraw.Draw(out)
    d.text((8 * SCALE, 6 * SCALE), text, font=font(14 * SCALE, bold=True), fill=(235, 235, 235, 255))
    if sub:
        d.text((8 * SCALE, 25 * SCALE), sub, font=font(11 * SCALE), fill=(165, 165, 165, 255))
    out.paste(im, (0, head))
    return out


def side_by_side(a, b, gap=14):
    w = a.width + b.width + gap * SCALE
    h = max(a.height, b.height)
    out = Image.new("RGBA", (w, h), (16, 16, 16, 255))
    out.paste(a, (0, 0))
    out.paste(b, (a.width + gap * SCALE, 0))
    return out


# ---------------------------------------------------------------- the four hulls at Mars
# One Ship per Faction, which is what the shot plants and what a typical mid-game Mars looks like.
FOUR = [
    (CUSTODIANS, "TSV Valiant (frigate)"),
    (PROSPECTORS, "PMV Magellan (colony ship)"),
    (ARKWRIGHTS, "ARK Challenger (carrier)"),
    (ARCHIVISTS, "ACV Endeavour (colony ship)"),
]

# --- A: the Body Surface Map's orbit block -------------------------------------------------
SURF = (250, 70, 780, 330)
now = caption(crop("cap-mars.png", SURF), "NOW  —  Mars Body Surface Map",
              "One line per Faction: how many hulls and their strength. No hull is named.")
prop = crop("cap-mars.png", SURF)
d = ImageDraw.Draw(prop, "RGBA")
d.rectangle([(406 - SURF[0]) * SCALE, (95 - SURF[1]) * SCALE,
             (600 - SURF[0]) * SCALE, (205 - SURF[1]) * SCALE], fill=(16, 16, 16, 255))
block(d, (406 - SURF[0]) * SCALE, (95 - SURF[1]) * SCALE, FOUR,
      title="In orbit around Mars", tail=(CUSTODIANS, "Orbital Control: Custodians"))
prop = caption(prop, "PROPOSED  —  one line per hull",
               "Four hulls, four lines. The block grows by nothing at this size.")
side_by_side(now, prop).save(os.path.join(OUT, "a-surface-map.png"))

# --- B: the Solar System Map's stack block -------------------------------------------------
SOL = (860, 380, 1260, 620)
now = caption(crop("cap-solar.png", SOL), "NOW  —  Solar System Map at Mars",
              "The same per-Faction block, above the Body's own label.")
prop = crop("cap-solar.png", SOL)
d = ImageDraw.Draw(prop, "RGBA")
d.rectangle([(975 - SOL[0]) * SCALE, (403 - SOL[1]) * SCALE,
             (1145 - SOL[0]) * SCALE, (487 - SOL[1]) * SCALE], fill=(16, 16, 16, 255))
block(d, (975 - SOL[0]) * SCALE, (403 - SOL[1]) * SCALE, FOUR,
      tail=(CUSTODIANS, "Orbital Control: Custodians"))
prop = caption(prop, "PROPOSED  —  one line per hull",
               "Note how close Mars, Phobos and Deimos already sit: this block is between them.")
side_by_side(now, prop).save(os.path.join(OUT, "b-solar-map.png"))

# --- C: the stress case -------------------------------------------------------------------
# The worst fleet seen in 200 measured games: Arkwrights, 24 Colony Ships parked at Earth.
COLONY = ["Magellan", "Challenger", "Endeavour", "Beagle", "Discovery", "Resolution", "Endurance",
          "Terra Nova", "Fram", "Nautilus", "Vostok", "Zarya", "Kon-Tiki", "Golden Hind", "Victoria",
          "Santa Maria", "Erebus", "Investigator", "Adventure", "Bounty", "Rattlesnake", "Astrolabe",
          "Belgica", "Aurora"]
BIG = [(ARKWRIGHTS, "ARK %s (colony ship)" % n) for n in COLONY]
EARTH = (470, 170, 1010, 700)

full = crop("cap-solar.png", EARTH)
d = ImageDraw.Draw(full, "RGBA")
block(d, (612 - EARTH[0]) * SCALE, (180 - EARTH[1]) * SCALE, BIG)
full = caption(full, "STRESS  —  24 hulls, one line each",
               "The worst fleet in 200 measured games: 24 unlaunched Colony Ships at Earth.")

capped = crop("cap-solar.png", EARTH)
d = ImageDraw.Draw(capped, "RGBA")
block(d, (612 - EARTH[0]) * SCALE, (270 - EARTH[1]) * SCALE,
      BIG[:5] + [(GREY, "and 19 more — click the stack")])
capped = caption(capped, "STRESS  —  capped at five, then a tail",
                 "The same fleet with the list bounded.")
side_by_side(full, capped).save(os.path.join(OUT, "c-stress-24-hulls.png"))

print("wrote:", ", ".join(sorted(os.listdir(OUT))))
