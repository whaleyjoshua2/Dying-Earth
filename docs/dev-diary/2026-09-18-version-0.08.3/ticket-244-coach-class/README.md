# Ticket #244: Steerage becomes Coach Class

The second half of the rename that [Emigrant becomes Colonist: the label, or the distinction
too?](https://github.com/whaleyjoshua2/Dying-Earth/issues/234) started. The designer's reason there
was the word rather than the rule — *"it's more about the word being politically loaded and racked
with connotation"* — and the same objection reaches the Arkwrights' signature rule, which was named
**Steerage**: literally the cheapest class of passage on an emigrant ship.

The glossary had known for versions. The entry's own `_Avoid_` list read *"mass transit, cattle
class, overcrowding, packing them in"*.

## What the name had to mean

The rule is unchanged:

> their Colony Ships carry twice the Colonists and cost less to build, and they recruit eight
> Pioneers a turn where others recruit four, but every Pioneer costs their Region twice the
> population.

So the name had to say **cheap passage in bulk, at a price paid by the homeland**.

**Stowaway** was the designer's first suggestion and was argued down rather than adopted: a
stowaway hides aboard **without permission or payment**, where the Arkwrights' people are recruited
openly, counted, and carried in ships built for the purpose. It would have named a rule the game
does not have, and sat oddly beside the crowding rule, which is about ships that are legitimately
full. It fixed the connotation and broke the meaning.

**Coach Class** keeps the travel-class metaphor — which is the honest description of the mechanic —
and drops the emigrant-ship history. The two-word form was chosen over a bare *Coach* because
"coach" alone also means a trainer, and a one-word rule name cannot disambiguate.

**Diaspora stays.** The Arkwrights' Victory Condition was looked at in the same breath and ruled
fine by the designer: *"Diaspora seems fine"*.

## What moved

Live source and the glossary only: the Arkwrights' `signature` line and four comments in
`assets/data/factions.toml`, four places in `CONTEXT.md` (the Arkwrights entry, the rule's own
entry, a cross-reference on another entry's avoid list, and the Pioneer entry), engine doc comments
in `ai.rs`, `data.rs`, `orders.rs` and `state.rs`, one comment in `src/ui.rs`, and one test.

**Historical records keep the old name.** `docs/spec/version-0.05.md` onward and the dev-diary
entries of 2026-09-09 through 2026-09-15 are the record of what was decided then; rewriting them
would make them lie about their own moment.

## A rename with no mechanic behind it needs its own guard

Every figure Coach Class controls is unchanged, so **the tests that guard the rule stayed green
throughout and witnessed nothing**. A rename with no guard is exactly how a word creeps back, so
one was added: it asserts the Arkwrights' card leads with `Coach Class`, and that the word
*Steerage* appears in no Faction's signature, victory, blurb or unique line.

Witnessed red by putting `Steerage.` back on the card — `300 passed; 1 failed`, the new test alone
— then restored. `301 passed; 0 failed`, `6 passed; 0 failed`, clippy clean with the denial.

## Looked at

| picture | what it shows |
|---|---|
| [`faction-cards.png`](faction-cards.png) | The Faction selection screen. The Arkwrights' **Signature rule** now reads *"Coach Class. A Colony Ship carries 8 Colonists…"*, and their **Unique Facility** line beside it already says *"for every Pioneer it lifts off Earth"* from the sibling ticket. The four cards still sit at one height with their Play buttons aligned, which is ticket #217's work holding. |

Captured with `target/release/dying-earth.exe shot:<prefix> menus:1 window:1920x1080`, off-screen,
exit 0. Nothing was opened on the designer's desktop.
