number theory playground
port some logic from sympy to rust and run benchmakrs


```
trials: 5 iteration MIN: 341550071728321 MAX: 341550072728321
sympy total:42.77055257698521 avg: 8.554111 seconds
rust total: 1.2769192990090232 avg 0.25538385980180467
```

```
from manifold_rs import miller_rabin_bool
from sympy.ntheory.primetest import mr as miller_rabin
miller_rabin(i, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 47, 53, 59, 67, 71, 73, 79, 97])
miller_rabin_bool(i)
```

run insructions:
```
use build_release.sh for production code (this should run faster, needed for benchmarks)
if you dont know what do use, default to build_release.sh
use build_incremental.sh for faster builds
use build.sh for slower builds

```
