"""Mockups for ticket #219: the Research race bar on the Tech Tree.

Layout (b) as the designer chose it — the bar at the top, the legend directly under it, the tree
beneath — with a line of Faction symbols and percentages. Two variants differ only in what a Faction
that has contributed NOTHING does, which is the one question the words left open.

The window pixels, the legend and the Faction symbols are all lifted from real headless captures;
the bar and the percentage line are drawn. Run:

    py -3.12 make-mockups.py <captures-dir> <out-dir>
"""

import os, sys
from PIL import Image, ImageDraw, ImageFont

CAPS, OUT = sys.argv[1], sys.argv[2]
os.makedirs(OUT, exist_ok=True)
SCALE = 2

CUSTODIANS = (38, 166, 153)
PROSPECTORS = (217, 128, 51)
ARKWRIGHTS = (112, 70, 168)
ARCHIVISTS = (222, 82, 112)

FONT = "C:/Windows/Fonts/segoeui.ttf"
BOLD = "C:/Windows/Fonts/segoeuib.ttf"
f = lambda px, b=False: ImageFont.truetype(BOLD if b else FONT, px)

# The Tech Tree window in the 1600x900 capture, and the pieces taken from it.
WIN = (430, 120, 1212, 886)
BAR_Y = (160, 176)          # the race bar strip, window-relative once cropped
LEGEND = (437, 851, 1206, 874)
TREE_TOP = (437, 182, 1206, 300)

# The Faction symbols, as the game drew them on the setup screen at 1920x1080.
SYMS = {
    "custodians": (20, 100, 54, 134),
    "prospectors": (975, 100, 1009, 134),
    "arkwrights": (20, 512, 54, 546),
    "archivists": (975, 512, 1009, 546),
}


def sym(key, px):
    im = Image.open(os.path.join(CAPS, "w1920x1080-factions.png")).convert("RGBA")
    return im.crop(SYMS[key]).resize((px, px), Image.LANCZOS)


def piece(name, box):
    im = Image.open(os.path.join(CAPS, name)).convert("RGBA")
    return im.crop(box)


# Contributions of a plausible mid-game Tech: 24 of 30 paid, and the Arkwrights have put in nothing
# (which is what the Archivists' Fund the Archive, or simply having no Labs, produces).
SHARES = [("custodians", "Custodians", CUSTODIANS, 42),
          ("prospectors", "Prospectors", PROSPECTORS, 31),
          ("archivists", "Archivists", ARCHIVISTS, 27),
          ("arkwrights", "Arkwrights", ARKWRIGHTS, 0)]
PAID, COST = 24, 30


def header(width, quiet_zero):
    """The proposed top of the window: race bar, percentages line, then the real legend."""
    bar_h, line_h, pad = 14, 22, 6
    h = pad + bar_h + pad + line_h + pad
    im = Image.new("RGBA", (width, h), (27, 27, 27, 255))
    d = ImageDraw.Draw(im)
    # The bar: segments fill PAID/COST of the width, split by share; the rest is grey.
    x0, x1 = 6, width - 6
    filled = (x1 - x0) * PAID / COST
    d.rectangle([x0, pad, x1, pad + bar_h], fill=(70, 70, 70, 255))
    x = x0
    for _, _, colour, pct in SHARES:
        w = filled * pct / 100.0
        if w > 0:
            d.rectangle([x, pad, x + w, pad + bar_h], fill=colour + (255,))
        x += w
    # The percentages line.
    cx, cy = 6, pad + bar_h + pad
    for key, name, colour, pct in SHARES:
        if pct == 0 and quiet_zero:
            continue
        dim = pct == 0
        ink = tuple(int(c * 0.45) for c in colour) if dim else colour
        s = sym(key, 15)
        if dim:
            s = Image.blend(Image.new("RGBA", s.size, (27, 27, 27, 255)), s, 0.45)
        im.paste(s, (cx, cy + 2), s)
        text = f"{name} {pct}%"
        d.text((cx + 19, cy), text, font=f(13, b=not dim), fill=ink + (255,))
        cx += 19 + int(d.textlength(text, font=f(13, b=not dim))) + 22
    return im


def caption(im, title, sub):
    head = 46
    out = Image.new("RGBA", (im.width, im.height + head), (16, 16, 16, 255))
    d = ImageDraw.Draw(out)
    d.text((8, 6), title, font=f(15, True), fill=(235, 235, 235, 255))
    d.text((8, 26), sub, font=f(11), fill=(165, 165, 165, 255))
    out.paste(im, (0, head))
    return out


def stack(*pieces):
    w = max(p.width for p in pieces)
    out = Image.new("RGBA", (w, sum(p.height for p in pieces)), (27, 27, 27, 255))
    y = 0
    for p in pieces:
        out.paste(p, (0, y)); y += p.height
    return out


legend = piece("t-earth.png", LEGEND)
tree = piece("t-earth.png", TREE_TOP)
W = legend.width

now = stack(piece("t-earth.png", (437, 152, 1206, 182)), tree)
now = caption(now.resize((now.width * SCALE, now.height * SCALE), Image.LANCZOS),
              "NOW  —  the bar alone at the top of the window",
              "No percentages, no names. The legend is at the FOOT of the window, out of shot.")

for tag, quiet, title, sub in [
    ("a-quiet", True, "PROPOSED A  —  a Faction with nothing is not shown",
     "Three entries. The Arkwrights have contributed nothing and are quiet."),
    ("b-dimmed", False, "PROPOSED B  —  a Faction with nothing is shown dimmed at 0%",
     "Four entries always, the empty one greyed. The line never changes length."),
]:
    block = stack(header(W, quiet), legend, tree)
    block = block.resize((block.width * SCALE, block.height * SCALE), Image.LANCZOS)
    out = caption(block, title, sub)
    pair = Image.new("RGBA", (now.width + out.width + 28, max(now.height, out.height)), (16, 16, 16, 255))
    pair.paste(now, (0, 0)); pair.paste(out, (now.width + 28, 0))
    pair.save(os.path.join(OUT, f"{tag}.png"))

print("wrote:", ", ".join(sorted(x for x in os.listdir(OUT) if x.endswith(".png"))))
