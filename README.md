number theory playground
port some logic from sympy to rust and run benchmakrs

```
miller rabin benchmark
trials: 5
MIN, MAX = 10**6, 10**7

sympy total:147.42906583601143 avg: 29.485813 seconds
rust total: 36.66850686294492 avg 7.333701372588985

```
from manifold_rs import miller_rabin_bool
from sympy.ntheory.primetest import mr as miller_rabin
miller_rabin(i, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 47, 53, 59, 67, 71, 73, 79, 97])
miller_rabin_bool(i)
```

this needs to use less bases, like so
```
if n < 2047:
    a = [2]
if n < 1373653:
    a = [2, 3]
if n < 9080191:
    a = [31, 73]
if n < 25326001:
    a = [2, 3, 5]
if n < 3215031751:
    a = [2, 3, 5, 7]
if n < 4759123141:
    a = [2, 7, 61]
if n < 1122004669633:
    a = [2, 13, 23, 1662803]
if n < 2152302898747:
    a = [2, 3, 5, 7, 11]
if n < 3474749660383:
    a = [2, 3, 5, 7, 11, 13]
if n < 341550071728321:
    a = [2, 3, 5, 7, 11, 13, 17]
if n < 3825123056546413051:
    a = [2, 3, 5, 7, 11, 13, 17, 19, 23]
else:
    a = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
```

run insructions:
```
use build_incremental.sh for faster builds
use build.sh for slower builds
use build_release.sh for production code (this should run faster, needed for benchmarks)
```
