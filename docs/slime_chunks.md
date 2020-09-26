# Slime chunks

Each slime chunk reduces the expected runtime by 50%, until colisions start to
dominate. Non-slime chunks don't reduce the expected runtime, but are needed to
remove all the false positives.

Java Random uses 48 bits, so we can only find the lower 48 bits of the seed
using slime chunks.
But instead of bruteforcing 2^48, we bruteforce 2^18 + 2^30.
That's because those two lines are equivalent:

```c
i % 10 == 0
(i % 2 == 0) && (i % 5 == 0)
```

And the parity of the output of `Random.nextInt(10)` depends only on the
lower 18 bits, so if the parity is odd, we discard this 18-bit combination.

