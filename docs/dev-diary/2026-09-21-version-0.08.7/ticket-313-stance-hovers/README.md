# Stance words that say what they do on hover

Ticket [#313](https://github.com/whaleyjoshua2/Dying-Earth/issues/313) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`ships-intercept-hover.png`](ships-intercept-hover.png) | `shot: stack:1 panel:0 tip:Intercept: Fights`. The Custodians' Ship stack card at Mars with the hover on the **Intercept** label: *Intercept: Fights what arrives this turn, before it can land.* / *A stance persists until it is changed.* The sentence the review said nobody could discover. |
| [`china-dig-in-hover.png`](china-dig-in-hover.png) | `shot: select:eastasia army:1 panel:0 window:1280x1080 tip:Dig In: Dug in`. China's card with the hover on the **Dig In** label of the Armies block's stance row: the Dig In sentence and the persistence line. |
| [`roster-ship-stance.png`](roster-ship-stance.png) | `shot: panel:0`. The roster: *TSV Valiant (frigate) at Mars - strength 3, 30/30 (Hold)*, the Ship's stance word in brackets as an Army's row has had it; its hover carries the stance's sentence above the fuel rule. |

No batch was run: an interface change; the engine gains a table of sentences and no rule.

## What was decided, in the designer's words

*"q1 as recommended q2 yes q3 yes q4 yes"*: one sentence per stance in the engine, in the words
recommended; the persistence line on every label; the roster reading the same table, Ships' rows
gaining the stance word and the hover; *Stance:* kept.

## Settled by the builder, to be corrected if wrong

- **`Stance::one_liner(ships)`** beside `Stance::name()`, and **`Stance::PERSISTS`** for the shared
  line, so both sentences live once; an Army handed a Ship's stance (or the reverse) says it holds
  instead.
- **The label's hover** reads *Name: sentence* then the persistence line, two lines; through
  `rule_tip`, so the `tip:` aid can photograph it, and with no marker (#233).
- **The Army roster row's tip** keeps its own first and last sentences and reads the stance's from
  the table between them.
- The **Stance** glossary entry lists the six sentences.

## Looked at, not tested

The three pictures above, each opened and read before this was committed. The first Dig In picture
was taken at 800 pixels with the stance row below the fold and the hover floating at the foot; it
was retaken at 1080.
