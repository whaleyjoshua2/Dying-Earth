# The emissions breakdown, three seeds, all four seats computer-played

Printed by `engine/examples/breakdown.rs` (`cargo run -p dying-earth-engine --release --example breakdown -- <seed>`),
every fifth turn: Earth's emissions by source in ppm, the sink and the Scrubbers, and the Temperature.
The `factories` bucket holds every Factory and, since 0.09.0, every Mine Facility on Earth.

## 0.09.0 as first built (the Core at 1 Widget, 1 per Industry Level, Factory and Mine both at 1.0)

```
=== seed 3 (over at turn 34)
turn  industry factories power refin launch people cards permaf war | sink scrub | temp
   5      10.1       9.7   7.7   3.4    2.5    7.9   0.0    0.0  0.0 |  6.0   0.0 | +1.37
  10      11.2      13.2  11.5   5.2    0.0    8.4   0.0    4.0  0.0 |  6.0   0.0 | +1.67
  15      10.8      19.6  14.6   4.9    0.0    6.7   0.0    4.0  0.0 |  4.0   6.0 | +2.08
  20      10.1       5.0   6.9   3.9    0.0    4.8   0.0    4.0  0.0 |  4.0   0.0 | +2.42
  25      10.7       3.1   6.0   0.0    0.0    4.0   0.0    4.0  0.0 |  4.0   0.0 | +2.67
  30       9.9       2.4   5.6   0.6    0.0    3.3   0.0    4.0  0.0 |  4.0   0.0 | +2.88
=== seed 7 (over at turn 34)
   5      10.1       9.7   7.7   3.4    2.5    7.9   0.0    0.0  0.0 |  6.0   0.0 | +1.37
  10      10.8      12.2  13.3   5.2    0.0    7.6   0.0    4.0  0.0 |  6.0   0.0 | +1.68
  15      12.0      19.1  14.8   4.9    0.0    7.1   0.0    4.0  0.0 |  4.0   6.0 | +2.12
  20      11.0       4.4   6.2   3.0    0.0    5.1   0.0    4.0  0.0 |  4.0   6.0 | +2.45
  25      10.8       2.1   6.4   0.0    0.0    3.9   0.0    4.0  0.5 |  4.0   0.0 | +2.68
  30       9.4       1.5   5.2   0.0    0.0    3.4   0.0    4.0  0.0 |  4.0   0.0 | +2.88
=== seed 11 (over at turn 34)
   5      10.1       9.7   7.7   3.4    2.5    7.9   0.0    0.0  0.0 |  6.0   0.0 | +1.36
  10      10.9      11.3  12.0   5.2    0.0    8.0   0.0    4.0  0.0 |  6.0   0.0 | +1.67
  15      11.5      17.2  13.5   6.0    0.0    6.9   0.0    4.0  0.0 |  6.0   6.0 | +2.07
  20      11.2       3.6   6.9   3.4    0.0    5.1   4.0    4.0  0.0 |  4.0   9.0 | +2.42
  25      11.0       1.1   2.2   0.0    0.0    4.7   0.0    4.0  0.0 |  4.0   0.0 | +2.63
  30      13.4       1.5   2.2   0.0    0.0    4.2   0.0    4.0  0.0 |  4.0   0.0 | +2.85
```

## 0.08.8 (main at 471c165), the same probe

```
=== seed 3 (over at turn 36)
turn  industry factories power refin launch people cards permaf war | sink scrub | temp
   5      10.1       6.0   9.2   5.2    0.0    7.8   0.0    0.0  0.0 |  6.0   0.0 | +1.37
  10      10.8       8.8  11.5   8.2    0.0    7.8   0.0    0.0  0.0 |  6.0   0.0 | +1.65
  15      10.4       6.8   9.0   6.8    0.0    5.9   0.0    4.0  0.0 |  6.0   3.0 | +1.99
  20      10.0       5.4  10.4   6.4    0.0    4.7   0.0    4.0  0.0 |  4.0   0.0 | +2.31
  25       8.9       2.9   8.6   1.7    0.0    3.7   0.0    4.0  1.5 |  4.0   0.0 | +2.58
  30       8.8       2.4   5.2   2.2    0.0    3.0   0.0    4.0  0.0 |  4.0   0.0 | +2.80
  35       8.1       1.5   5.2   2.2    0.0    2.4   0.0    4.0  0.0 |  4.0   0.0 | +2.96
=== seed 7 (over at turn 33)
   5      10.1       6.0   9.2   5.2    0.0    7.8   0.0    0.0  0.0 |  6.0   0.0 | +1.37
  10      11.0       7.8  13.3   8.2    0.0    7.2   0.0    0.0  0.0 |  6.0   0.0 | +1.66
  15      12.6       9.6  15.6   6.8    0.0    7.0   0.0    4.0  0.0 |  6.0   0.0 | +2.06
  20      13.5       3.9   9.0   2.8    0.0    6.1   0.0    4.0  0.0 |  4.0   6.0 | +2.41
  25      12.7       2.0   6.2   0.0    0.0    5.2   0.0    4.0  4.0 |  4.0   0.0 | +2.69
  30      11.4       0.8   6.4   0.0    0.0    4.7   0.0    4.0  0.0 |  4.0   0.0 | +2.91
=== seed 11 (over at turn 36)
   5      10.1       6.0   9.2   5.2    0.0    7.8   0.0    0.0  0.0 |  6.0   0.0 | +1.36
  10      11.1       6.0  12.0   8.2    0.0    7.7   0.0    0.0  0.0 |  6.0   0.0 | +1.65
  15      11.7       7.9  14.8   7.1    0.0    6.4   0.0    4.0  0.0 |  6.0   3.0 | +2.02
  20      11.8       5.0   7.0   1.5    0.0    5.6   3.9    4.0  1.5 |  4.0   6.0 | +2.35
  25      12.1       2.2   4.3   0.0    0.0    5.5   0.0    4.0  1.5 |  4.0   0.0 | +2.56
  30      12.1       1.0   2.8   0.0    0.0    4.3   3.5    4.0  0.0 |  4.0   0.0 | +2.77
  35      10.7       0.9   2.8   0.0    0.0    3.4   0.0    4.0  0.0 |  4.0   0.0 | +2.94
```

**Reading.** The `factories` bucket runs 3.7 ppm a turn higher at turn 5 and 10 to 13 higher at turn 15
in every seed: the starting Mine beside every starting Factory, and the Factories the seats build for
Widgets, each emitting 1.0 as the Materials Factory did. Everything else is within noise. The 0.08.8
games end at +2.94 to +2.96, under the collapse line by a hair, so the extra tenth of a degree tips
the Custodians' Stabilization Runs and the collapse count together.
