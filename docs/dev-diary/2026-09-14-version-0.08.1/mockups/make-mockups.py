import re, os, sys

REPO = r"C:\Users\Josh\games\Dying-Earth"
OUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "html")
os.makedirs(OUT, exist_ok=True)

COLOUR = {
    "custodians": "#26a699",
    "prospectors": "#d98033",
    "arkwrights": "#7046a8",
    "archivists": "#de5270",
}
NAME = {
    "custodians": "Custodians",
    "prospectors": "Prospectors",
    "arkwrights": "Arkwrights",
    "archivists": "Archivists",
}


def glyph(kind, size, colour=None):
    """The faction symbol, with game-icons.net's full-canvas black rect dropped."""
    src = open(os.path.join(REPO, "assets", "icons", kind + ".svg"), encoding="utf-8").read()
    ds = re.findall(r'<path[^>]*\bd="([^"]+)"', src)
    ds = [d for d in ds if d.strip() != "M0 0h512v512H0z"]
    fill = colour or COLOUR[kind]
    paths = "".join('<path fill="%s" d="%s"/>' % (fill, d) for d in ds)
    return '<svg class="glyph" width="%d" height="%d" viewBox="0 0 512 512">%s</svg>' % (size, size, paths)


CSS = """
*{box-sizing:border-box}
body{margin:0;background:#101010;font-family:"Segoe UI",system-ui,sans-serif;padding:18px}
.win{width:524px;background:#1b1b1b;border:1px solid #3d3d3d;border-radius:5px;overflow:hidden}
.bar{background:#2a2a2a;border-bottom:1px solid #3d3d3d;display:flex;align-items:center;
     padding:5px 8px;color:#cfcfcf;font-size:14px}
.bar .caret{opacity:.75}
.bar .title{flex:1;text-align:center}
.bar .x{opacity:.75}
.body{padding:10px 12px 14px;color:#c2c2c2;font-size:12.5px;line-height:1.42}
.selrow{display:flex;align-items:center;justify-content:flex-end;gap:7px;margin-bottom:8px}
.selrow .lbl{color:#8b8b8b;font-size:12px}
.combo{background:#333;border:1px solid #4a4a4a;border-radius:3px;padding:3px 8px;color:#dcdcdc;
       font-size:12.5px;display:flex;align-items:center;gap:6px}
.combo .arrow{opacity:.65;font-size:10px}
.head{display:flex;align-items:center;gap:12px;margin-bottom:2px}
.head .nm{font-size:26px;font-weight:600}
.blurb{color:#9d9d9d;margin:2px 0 9px}
h4{margin:9px 0 2px;font-size:12.5px;font-weight:600;color:#e2e2e2}
.weak{color:#8b8b8b}
.mults{color:#c2c2c2}
.sig{color:#bdbdbd}
hr{border:0;border-top:1px solid #333;margin:10px 0}
.bar2{height:11px;background:#2f2f2f;border-radius:3px;margin:3px 0 7px;overflow:hidden}
.bar2 > i{display:block;height:100%;background:#4a7ab8}
.grid2{display:grid;grid-template-columns:1fr 1fr;gap:2px 16px}
.inc{display:flex;flex-wrap:wrap;gap:4px 14px;margin-top:2px}
.inc b{font-weight:600;color:#dedede}
.rel{display:grid;grid-template-columns:auto 1fr;gap:2px 10px;margin-top:2px}
.rel .who{text-align:right}
.pos{color:#7fbf7f}.neg{color:#d98282}.zero{color:#9a9a9a}
.blame{display:flex;align-items:center;gap:8px;margin-top:3px}
.blame .track{flex:1;height:13px;background:#2f2f2f;border-radius:3px;overflow:hidden}
.blame .track > i{display:block;height:100%;text-align:center;color:#111;font-size:10.5px;line-height:13px}
.note{color:#7e7e7e;font-style:italic;margin-top:4px}
.glyphrow{display:flex;align-items:flex-end;gap:34px}
.gcell{text-align:center}
.gcap{color:#8b8b8b;font-size:11.5px;margin-top:6px}
.tag{display:inline-block;background:#2c2c2c;border:1px solid #3d3d3d;border-radius:3px;
     padding:1px 6px;color:#a8a8a8;font-size:11.5px;margin-right:5px}
"""

FACTS = {
    "custodians": dict(
        blurb="Colonize the solar system while limiting ecological damage to Earth.",
        mults="Output x1 &nbsp;·&nbsp; Emissions x0.75 &nbsp;·&nbsp; Research x1.25 &nbsp;·&nbsp; Influence x1.2",
        extras="",
        sig="The Scrubber and Leapfrog. A Scrubber is a Facility only they build, in a Nation State they "
            "control, taking no build slot: 30 Materials, 2 turns, 4 Energy upkeep, no Emissions; while it is "
            "online it enlarges the Natural Sink by 3.0 ppm a turn and takes 1 off that state's Unrest. "
            "Leapfrog costs 50 Ducats and lowers a state's Emissions coefficient by 0.03 for good.",
        unique="Academy &mdash; replaces the School and the Institute, and pays +1 Ducat a turn.",
        vic="Stabilization (3 consecutive Climate phases with net Emissions below the Natural Sink) plus 12 "
            "Colonists living off Earth.",
        gate="Waits on Planetary Stewardship, a rung-3 Tech (45 Research).",
        p1=("Stabilization run: 1 of 3", 33),
        p2=("Off-world Presence: 8 of 12 Colonists living off Earth", 67),
        pct=50,
        blame=(29, "Blame 6 ppm, thresholds x1.04"),
        holds="4 Regions &nbsp;·&nbsp; 2 Colonies &nbsp;·&nbsp; 1 station &nbsp;·&nbsp; 3 Ships &nbsp;·&nbsp; 2 Armies",
    ),
    "archivists": dict(
        blurb="Build the Archive and upload everyone.",
        mults="Output x1 &nbsp;·&nbsp; Emissions x0.9 &nbsp;·&nbsp; Research x1.5 &nbsp;·&nbsp; Influence x1",
        extras="Research off Earth x1.75",
        sig="Provisional Findings: while their Research went to the shared Tech last turn, they already have "
            "half the effect of the Tech under research. A turn of funding the Archive switches it off for the "
            "turn after. Their Research is x1.5 on Earth and x1.75 off it.",
        unique="Reactor &mdash; replaces the Power Plant, and takes 75% off their buildings' Energy upkeep.",
        vic="Build the Archive at a Colony off Earth (50 Materials, three turns, four Colonists to begin it), "
            "pay 80 Research into it and keep it running, then upload 12 Colonists into it.",
        gate="Waits on The Upload, a rung-3 Tech (45 Research).",
        p1=("The Archive: 62 of 80 Research paid in", 78),
        p2=("Colonists uploaded: 4 of 12", 33),
        pct=55,
        blame=(14, "Blame 3 ppm, thresholds x1.00"),
        holds="3 Regions &nbsp;·&nbsp; 1 Colony &nbsp;·&nbsp; 2 stations &nbsp;·&nbsp; 2 Ships &nbsp;·&nbsp; 1 Army",
    ),
}

INCOME_OWN = [("Materials", "+32"), ("Fuel", "+6"), ("Energy", "+18"), ("Ducats", "+24"), ("Research", "+11")]
INCOME_RIVAL = [("Materials", "+27"), ("Fuel", "+4"), ("Energy", "+21"), ("Ducats", "+15"), ("Research", "+29")]

REL_OUT = [("Prospectors", -4), ("Arkwrights", 0), ("Archivists", -7)]
REL_IN = [("Prospectors", -2), ("Arkwrights", 0), ("Archivists", -5)]
REL_OUT_A = [("Custodians", -5), ("Prospectors", 0), ("Arkwrights", -1)]
REL_IN_A = [("Custodians", -7), ("Prospectors", -3), ("Arkwrights", 0)]


def relcls(v):
    return "pos" if v > 0 else ("neg" if v < 0 else "zero")


def rel_block(out, inn, who):
    def rows(pairs):
        return "".join(
            '<div class="who">%s</div><div class="%s">%+d</div>' % (n, relcls(v), v) for n, v in pairs
        )
    return (
        '<h4>Relations</h4>'
        '<div class="weak">What the %s think of the others</div>'
        '<div class="rel">%s</div>'
        '<div class="weak" style="margin-top:5px">What the others think of the %s</div>'
        '<div class="rel">%s</div>'
        '<div class="note">+10 to &minus;10 from a neutral 0. Falls 1 a turn they are crossed; '
        'recovers +1 every four quiet turns. Nothing in this version reads these figures.</div>'
        % (who, rows(out), who, rows(inn))
    )


def income_block(rows, rival):
    cells = "".join('<span><b>%s</b> %s</span>' % (v, n) for n, v in rows)
    note = ('<div class="note">A rival\'s income is shown as totals only; '
            'the building-by-building breakdown is yours alone.</div>') if rival else \
           ('<div class="weak" style="margin-top:3px">Hover for the building-by-building breakdown.</div>')
    return '<h4>Income last turn</h4><div class="inc">%s</div>%s' % (cells, note)


def rulebook(kind, gsize):
    f = FACTS[kind]
    extras = ('<div class="mults">%s</div>' % f["extras"]) if f["extras"] else ""
    return (
        '<div class="head">%s<div class="nm" style="color:%s">%s</div></div>'
        '<div class="blurb">%s</div>'
        '<h4>Multipliers</h4><div class="mults">%s</div>%s'
        '<h4>Unique Facility</h4><div class="sig">%s</div>'
        '<h4>Signature rule</h4><div class="sig">%s</div>'
        '<h4>Victory Condition</h4><div class="sig">%s</div>'
        '<div class="weak">%s</div>'
        % (glyph(kind, gsize), COLOUR[kind], NAME[kind], f["blurb"], f["mults"], extras,
           f["unique"], f["sig"], f["vic"], f["gate"])
    )


def live(kind, rival):
    f = FACTS[kind]
    inc = INCOME_RIVAL if rival else INCOME_OWN
    rel = rel_block(REL_OUT_A, REL_IN_A, NAME[kind]) if kind == "archivists" else rel_block(REL_OUT, REL_IN, NAME[kind])
    share, tail = f["blame"]
    return (
        '<h4>Victory progress &mdash; %d%% of the way there</h4>'
        '<div>%s</div><div class="bar2"><i style="width:%d%%"></i></div>'
        '<div>%s</div><div class="bar2"><i style="width:%d%%"></i></div>'
        '%s'
        '<h4>Blame</h4>'
        '<div class="blame"><div class="track"><i style="width:%d%%;background:%s">%d%%</i></div>'
        '<div class="weak">%s</div></div>'
        '%s'
        '<h4>Holdings</h4><div>%s</div>'
        % (f["pct"], f["p1"][0], f["p1"][1], f["p2"][0], f["p2"][1],
           income_block(inc, rival), share, COLOUR[kind], share, tail, rel, f["holds"])
    )


def selector(kind):
    return (
        '<div class="selrow"><span class="lbl">Faction</span>'
        '<div class="combo">%s <span>%s%s</span><span class="arrow">&#9660;</span></div></div>'
        % (glyph(kind, 14), NAME[kind], " (you)" if kind == "custodians" else "")
    )


def page(inner, width=None):
    w = ('<style>.win{width:%dpx}</style>' % width) if width else ""
    return "<!doctype html><meta charset='utf-8'><style>%s</style>%s%s" % (CSS, w, inner)


def window(kind, order, gsize, rival):
    parts = [rulebook(kind, gsize), '<hr>', live(kind, rival)]
    if order == "live-first":
        parts = [live(kind, rival), '<hr>', rulebook(kind, gsize)]
    return (
        '<div class="win"><div class="bar"><span class="caret">&#9660;</span>'
        '<span class="title">Factions</span><span class="x">&#10005;</span></div>'
        '<div class="body">%s%s</div></div>' % (selector(kind), "".join(parts))
    )


def write(name, html):
    p = os.path.join(OUT, name + ".html")
    open(p, "w", encoding="utf-8").write(html)
    print(p)


write("a-rulebook-first", page(window("custodians", "rulebook-first", 64, False)))
write("b-live-first", page(window("custodians", "live-first", 64, False)))
write("c-rival", page(window("archivists", "rulebook-first", 64, True)))

cells = "".join(
    '<div class="gcell">%s<div class="gcap">%d px</div></div>'
    % ('<div class="head" style="gap:12px"><div>%s</div><div class="nm" style="color:%s;font-size:%dpx">%s</div></div>'
       % (glyph("custodians", s), COLOUR["custodians"], max(18, int(s * 0.40)), "Custodians"), s)
    for s in (48, 64, 96)
)
write("d-glyph-sizes", page(
    '<div class="win" style="width:640px"><div class="bar"><span class="caret">&#9660;</span>'
    '<span class="title">Factions &mdash; glyph size</span><span class="x">&#10005;</span></div>'
    '<div class="body"><div class="glyphrow" style="flex-direction:column;align-items:flex-start;gap:18px">%s</div></div></div>'
    % cells))
