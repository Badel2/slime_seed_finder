# PRNG Internals

There are 2 different PseudoRandom Number Generators (PRNGs) used in Minecraft.
We will call them `JavaRandom` and `McRng`.

Note: if you need a fast PRNG with less flaws than the ones described in this
document, I recommend to take a look at [xorshift generators][Xorshift].

Table of Contents
<!-- This is not an unresolved merge conflict -->==================

   * [PRNG Internals](#prng-internals)
      * [Notation](#notation)
   * [JavaRandom](#javarandom)
      * [LCG](#lcg)
      * [Speed](#speed)
      * [Why 48-bit state?](#why-48-bit-state)
      * [setSeed](#setseed)
      * [next](#next)
      * [nextInt](#nextint)
      * [previous](#previous)
      * [next n calls](#next-n-calls)
      * [Minecraft seed initialization](#minecraft-seed-initialization)
      * [Minecraft structures](#minecraft-structures)
   * [McRng](#mcrng)
      * [QCG](#qcg)
      * [Speed](#speed-1)
      * [setSeed](#setseed-1)
   * [Appendix A: Modulo bias](#appendix-a-modulo-bias)
   * [Appendix B: magic constants in the wild](#appendix-b-magic-constants-in-the-wild)
   * [Sources](#sources)

Table of Contents created by [gh-md-toc](https://github.com/ekalinin/github-markdown-toc)
<!-- Run ./ci/gh-md-toc docs/prng.md and copy the table here -->

## Notation

This is the notation used in this document:

* Ranges: `[0, 10]`: from 0 to 10 inclusive
* Bits: Counting from low to high, starting from 0. Example: the number 13 in
  binary is 1101. The bits are numbered 3210. The lowest or least significant
bit (LSB) is bit 0, with value 1. Bit 1 has value 0, bit 2 has value 1, and bit
3 has value 1. Bit 3 is the highest or most significant bit (MSB). When the
number 13 is converted to binary, bits [3, 2] are 1. The three lower bits are
bits [2, 0].
* Modular operations: 7 mod 4 = 7 % 4 = 3
* Exponentiation: 2^N is always 2 to the power of N, never 2 XOR N. Expect in
  code snippets, then it follows the syntax of the specific programming
  language, for example `2**n` in python.

Some rough definitions:

* Entropy: number of possible states.
* Parity: the value of bit 0. Even or odd.
* Integer: a number with no fractional part. Not a synonym to `int`, can also
  be 64-bit or n-bit.
* Candidate: a seed satisfying some conditions.

# JavaRandom

This is the default PRNG used by Java, found in the module `java.util.Random`.
[Oracle documentation][JavaUtilRandom]

In Minecraft it is used for everything except biome map generation.

## LCG

The `JavaRandom` PRNG is a [Linear Congruential Generator (LCG)][LCG] with
48-bit state.

Being a LCG means that the next state `s(n+1)` can be calculated given only the
current state `s(n)` using the following formula:

```
s(n+1) = A * s(n) + C (mod M)
```

And particularly, it uses the constants:

```
A = 25214903917 //0x5DEECE66D
C = 11 //0xB
M = 2^48
```

These constants ensure that the LCG has period M, meaning that it will visit
all possible 2^48 states regardless of the starting point.

```
s(n + 2^48) = s(n)
```

## Speed

The main advantage of LCGs is that they are very fast. To give you an idea, I
benchmarked a small program which creates a RNG with a given seed, and
generates a 31-bit integer, 2^N times. These are the results:

```
N: time
24:  0m00.113s
28:  0m00.353s
32:  0m04.112s
36:  1m04.673s
40: 16m42.524s
```

While being fast may be a desired property, this will allow us to very quickly
bruteforce the internal state of the PRNG once we know enough bits.
Extrapolating, it would take around 3 days to bruteforce the full 48-bit state.
This is on one CPU core: multithreaded code or modern GPUs can do it in a few
hours.

A crucial property (and weakness) of LCGs is that the value of bit `n` depends only on
the bits `[n, 0]` of the previous state. Bit 0, the least significant bit, only
depends on the previous value of the bit 0. In `A * x + C`, A and C are odd, so
in the 1-bit case the equation can be simplified to `1 * x + 1 (mod 2^1)`, or
`x + 1 (mod 2^1)`. In other words, bit 0 always oscillates between 0 and 1.

In general, bit n has period 2^(n + 1). A function which uses bit 47 (the most
significant bit) will use the full 48 bits of state, while a function which
uses bit 17 will only use 18 bits of state. A function which uses only 18 bits
of state only has 2^18 possible states, so bruteforcing becomes instant.

## Why 48-bit state?

In Java, integers are 32 bits and longs are 64 bits. The seed is defined as a
long, so why only 48 bits are used?

The answer is unclear, but it seems to be "historical reasons". Keep in mind
that Java 1.0a was released in 1994, and the oldest source of `java.util.Random`
that I could find dates back to 1997.

But It's probably just a copy of the POSIX function `drand48`.

## setSeed

When creating a new instance of `JavaRandom` you can manually set the seed.
For example, this will set the seed to 0:

```
Random r = new Random(0);
```

But that seed is not directly equivalent to the internal seed of the PRNG. It
is first XORed with the constant A. So in this case the internal seed will be
25214903917 . But luckly, the XOR is a reversible operation so this is not a
problem. If we want to set the internal seed to 0, we can just call:

```
Random r = new Random(0 ^ 25214903917);
```

And this XOR will cancel out the internal XOR. Because of the simplicity of
this operation, I may forget about it in this document and say that the
internal seed is equal to the world seed, but you have to know that that's not
exactly correct.

Note that if `setSeed` used a one-way function like for example a cryptographic hash,
the bruteforce process would need to have an extra step: reversing that hash.

Because of the linearity of the LCG, two random generators initialized with
consecutive seeds will produce very similar outputs for the first call:

```
Random r0 = new Random(0);
Random r1 = new Random(1);
r0.nextInt(); // -1155484576
r1.nextInt(); // -1155869325
```
[Try it online](https://repl.it/repls/MobileCheerfulPup)

This is because the constant A is a 35-bit number, so the difference between
consecutive seeds will only propagate to the lower 35 bits of the PRNG state.
(Except in some rare cases when `A*k` is `0xFFFF...` and `A*(k+1)` is
`0x0000...`).

In order to guarantee that all the bits can be affected, the difference between
seeds must be at least `11164`, or `math.ceil(2**48 / A)`.

## next

The main function of this PRNG is `next(bits)`. This function returns `n`
pseudo-random bits, where `n` can be up to 32.

```
protected int next(int bits) {
    // advance state
    seed = seed * A + C;
    // mod 2^48
    seed = seed & ((1L << 48) - 1);
    // 1 <= bits <= 32
    // when bits = 1, returns bit 47 of the internal state
    // when bits = 32, returns bits [47, 16] of the internal state
    return (int)(seed >>> (48 - bits));
}
```

Note that the bits are always the most significant bits of the internal state,
as they have more entropy than the lower bits.

If we know the result of one call of `next(32)`, we automatically know 32 bits
of the internal state. This only leaves 16 unknown bits.

So with two consecutive calls to `next(32)`, we can instantly recreate the PRNG
state in a unique way, giving us 100% certainity on future predictions.

This "attack" has been implemented multiple times on StackOverflow, but we cannot
use it for finding a Minecraft world seed because as far as I know there is no way
to get the 32 bits directly.

<https://franklinta.com/2014/08/31/predicting-the-next-math-random-in-java/>

<https://jazzy.id.au/2010/09/20/cracking_random_number_generators_part_1.html>

(Nice answer but spoilers for the next part of this document:)

<https://crypto.stackexchange.com/a/2087>

Note: the code posted in these links does a 2^16 bruteforce to find the 48-bit
seed given 2 results of calls to next(32). It is enough to check 6 seeds after
doing some arithmetic. More on this later.

## nextInt

Another important function is the generation of integers in a range. This is
commonly used to simulate a dice rool: if I want a 1 in 10 chance of something
happening I can do `nextInt(10) == 0`. Let's see the first line of this
function:

```
// Return integer between 0 and (n - 1) inclusive.
public int nextInt(int n) {
    if (n & -n) == n { // if isPowerOfTwo(n)
        return (int)((n * (long)next(31)) >> 31);
    }

    // [...]
}
```

The first thing we see is a check if n is a power of two. Why?

Let `n = 2^b`, a power of two. Doing `x % n` would only use the lowest b bits
of x, resulting in very low entropy as discussed previously.  For instance,
let's define a function `badRandomBool` which returns `next(31) % 2`:

```
public boolean badRandomBool(Random r) {
    return r.next(31) % 2;
}
```

This function uses the lowest bit of `next(31)`. `next(31)` returns bits `[47,
17]`, so this is the bit number 17 of the internal state (bits `[16, 0]` get
discarted by the bitshift). This would result in a PRNG with 18-bit state. To
fix this, the Java PRNG explicitly handles this case and returns the upper `b`
bits instead.

This also has a small problem because users can use the modulo operator outside
the call: a call to `nextInt(1024)` would return the 10 highest bits of the
internal state [47, 38], with 48-bit entropy. But doing `nextInt(1024) % 8`
would use bits [40, 38], resulting in 40-bit entropy. In general, we can
predict the lower bits of a call to `nextInt` more easily than the higher bits.

Let's keep analyzing `nextInt`:

```
// Return integer between 0 and (n - 1) inclusive.
public int nextInt(int n) {
    if ((n & -n) == n) { // if isPowerOfTwo(n)
        return (int)((n * (long)next(31)) >> 31);
    }

    int bits = next(31);
    // Check for modulo bias:
    // If bits is greater than or equal to the greateast multiple of n below
    // 2^31, the following check will overflow (since everything on the left
    // side is positive).
    while (bits - (bits % n) + (n - 1) < 0) {
        // Reroll
        bits = next(31);
    }
    return bits % n;
}
```

The modulo bias check may make it look complicated but we can just ignore it
for our purposes, if you want to read more about it see the [Appendix A](#appendix-a-modulo-bias).

Let's focus on the last line: `bits % n`. By doing modulo n, the only possible
results are [0, n-1], as expected. And since there was a check for modulo bias,
all of the outputs are equally likely. So what's the problem?

Since n cannot be a power of two, doing `bits % n` will use the full 48-bits of
state? But while n cannot be a power of two, it can be a *multiple* of two.

Let's look at the slime chunk algorithm from Minecraft. It is pretty simple:
10% of all chunks are slime chunks, so the code generates a random number in
[0, 9] which depends on the `(x, z)` coordinates of the chunk and the world
seed. If that number is zero, then this chunk can spawn slimes.

```
if (r.nextInt(10) == 0) {
    // Can spawn slimes
}
```

Well, it turns out that the slime chunk check can be optimized as follows:

```
int s = r.nextInt(10);
if (s % 2 == 0 && s % 5 == 0) {
    // Can spawn slimes
}
```

That's just mathematics, for `s` to be a multiple of 10, it must be a multiple
of 2 and a multiple of 5. But remember what happens when you take something to
the modulo of a power of two? `s % 2` only uses the lowest bit of `s`.

So, since `nextInt(10)` uses a call to `next(31)`, we lose the 17 least
significant bits `[16, 0]`. The lowest bit of `s` is the bit 17 of the internal
state, so the parity of `s` only depends on 18 bits from the internal state.

Now instead of having to bruteforce 2^48 seeds to find the one with matching
slime chunks, we can bruteforce 2^18 seeds, discard the ones where `s % 2 ==
1`, and bruteforce the remaining 30 bits, so in total we would have to check
2^18 + 2^30 seeds instead of 2^48.

To give you an idea of how big is the time save with this optimization, 2^30
seeds can be bruteforced in under 1 minute while 2^48 can take months on a weak
CPU. (Although a modern GPU can bruteforce 2^48 seeds in a few hours).

So, to wrap up:

> Calls to nextInt(2\*n) only use 18 bits to calculate the parity

Because when n is even the parity depends on the lowest 18 bits of the PRNG
state.

## previous

It is also important to mention that it is pretty trivial to create a function
`previous()` which will undo the effects of `next()`. Let's derive the formula:

```
s(n+1) = A * s(n) + C (mod M)
s(n) = A * s(n-1) + C (mod M)
s(n) - C = A * s(n-1) (mod M)
(s(n) - C) * A_INV = A * A_INV * s(n-1) (mod M)
(s(n) - C) * A_INV = s(n-1) (mod M)
s(n-1) = (s(n) - C) * A_INV (mod M)
```

We cannot divide by A in modular space, so we must multiply by the inverse of
A: A\_INV. This constant is not so trivial to calculate, as it is the [modular
multiplicative inverse][ModMultInv] of A mod M:

```
A * A_INV = 1 (mod M)
```

Luckly,
[WolframAlpha](https://www.wolframalpha.com/input/?i=x+*+25214903917+%3D+1+mod+2%5E48)
exists so we can just outsource the calculation, which yields:

```
246154705703781 //0xdfe05bcb1365
```

The result and its hexadecimal representation can be used in a google search to
find any projects that try to somehow reverse the Java PRNG.

## next n calls

Other interesting functions are `next_n_calls` and `previous_n_calls`, which
use modular exponentiation to instantly advance or rewind the PRNG by `n`
steps. Well, actually we only need to implement one of these, as they are
related by the modulus M:

```
next_n_calls(n) = previous_n_calls(M - (n % M))
```

For example, to advance 3 steps:

```
s(n+1) = s(n) * A + C (mod M)
s(n+2) = (s(n) * A + C) * A + C (mod M)
s(n+3) = ((s(n) * A + C) * A + C) * A + C (mod M)
s(n+3) = s(n) * A^3 + C * A^2 + C * A + C (mod M)
s(n+3) = s(n) * A^3 + C * (A^2 + A + 1) (mod M)
// A^2 + A + 1 = (A^3 - 1) / (A - 1)
s(n+3) = s(n) * A^3 + C * (A^3 - 1) * A_1_INV (mod M)
```

Where A\_1\_INV is the modular multiplicative inverse of (A - 1) mod M.

An important observation is that `s(n+3)` follows the same formula of `X * s(n)
+ Y`, for some constants X and Y, in this case `X = A^3` and `Y = C * (A^3 - 1)
* A_1_INV`. This means that for any n we can precompute these constants and
keep the runtime cost of "n calls to next" equal to "1 call to next", which
is great for bruteforcing.

Let's calculate A\_1\_INV.
This time WolframAlpha will not give us a solution because it does not exist.
That's because for a modular multiplicative inverse to exist, (A - 1) and M
must be coprime. In this case they have a common factor of 4:

```
A - 1 = 0 mod 4
```

So it is impossible to find `A_1_INV` such as:

```
A_1_INV * (A - 1) = 1 mod 2^48
```

But we can just factor the 4 out:

```
A_1_INV * ((A - 1) / 4) = 1 mod 2^48
```

And
[WolframAlpha](https://www.wolframalpha.com/input/?i=x+*+%2825214903917+-+1%29%2F4+%3D+1+mod+2%5E48)
solves the equation for us:

```
18698324575379 // 0x11018afe8493
```

This time there are very few results in Google, but in one result they mention
that when the Java PRNG has internal state equal to `A_1_INV`, the next two
calls to `next(32)` will return exactly the same integer:

```
A_1_INV >> 16 = (A_1_INV * A + C) >> 16 (mod M >> 16)
```

That's true, but the proof is left as an exercise to the reader.

## nextFloat

Aside from slime chunks, Minecraft also uses `JavaRandom` for generating
structures. This means that structures only use the lower 48 bits of the world
seed.

Unfortunately for us, the structure algorithms are slightly more advanced that
the one used for slime chunks.

For example, the buried treasure algorithm:

<details>
<summary>
Sidenote: there is a really simple way to always find the buried treasure when
given a treasure map.
</summary>
Buried treasures only generate if the biome is beach or cold beach. The biome
check is always performed at coordinate (chunkX * 16 + 9, chunkZ * 16 + 9), and
the buried treasure is generated at that coordinate. Which means that a really
simple way to find buried treasures is to start digging at block coordinate (9,
9) inside the chunk.
</details>

```
public boolean isTreasureChunk(long worldSeed, int x, int z) {
    long seed = x * SA + z * SB + worldSeed + SK;
    Random r = new Random(seed);
    return r.nextFloat() < 0.01;
}
```

The constants are not important, but here they are:

```
SA = 341873128712
SB = 132897987541
SK = 10387320
```

The main idea is that 1% of beach chunks should contain a buried treasure.
At first glance the algorithm may look pretty solid: if instead of using
`r.nextFloat() < 0.01` they had used `r.nextInt(100) == 0`, we could break
it the same way as the slime chunk algorithm. But let's look at the
implementation of `nextFloat()`:

```
public float nextFloat() {
    return next(24) / ((float)(1 << 24));
}
```

`next(24)` uses the top 24 bits of the PRNG state: bits [47, 24]. The function
converts this 24-bit integer into a float by dividing it by 2^24. This results
in a float between 0.0 (inclusive) and 1.0 (exclusive). We can get rid of the
floats to simplify our analysis, let's just multiply everything by 2^24:

```
r.nextFloat() < 0.01;
r.next(24) < ONE_PERCENT_OF_2_24
```

Where `ONE_PERCENT_OF_2_24` is a constant that can be calculated using python:

```py
>>> math.ceil(0.01 * 2**24)
167773
```

This results in a slightly optimized buried treasure check:

```
public boolean isTreasureChunk(long worldSeed, int x, int z) {
    long seed = x * SA + z * SB + worldSeed + SK;
    Random r = new Random(seed);
    return r.next(24) < 167773;
}
```

But now we can clearly see some implications. 167773 is a 18-bit number:

```
>>> math.log2(167773)
17.35615103347853
```

And since we are checking for `next(24)` to be lower than that, the output of
`next(24)` must also be a 18-bit number. In other words, the top 6 bits of
`next(24)` must be all zero.

> If we know at least one buried treasure location, we can optimize the
bruteforce from 2^48 to 2^42.

The process is simple: bruteforce the lower 42 bits [41, 0], and set the top 6
bits [47, 42] to zero. If the top 6 bits of next(24) are all zero, you're done.
Otherwise, just set the bits [47, 42] of the output of next(24) to zero and
call previous(). This will calculate the top 6 bits of the world seed for free!

Knowing more than one buried treasure location doesn't improve the bruteforce
speed, you still have to bruteforce 2^42 seeds, but you may be able to discard
more seeds. In theory, with 7 buried treasures (1 for the top bits
optimization and 6 more to remove false candidates) you should find 4.4 seeds
out of the total 2^48.

```py
>>> 2**42 * 0.01**6
4.398046511104001
```

But this assumes that the PRNG is good enough to not introduce any bias. In
practice, 7 buried treasures are not enough. A quick calculation shows that
instead of the expected 4.4 seeds we may find 240k seeds. So in practice it may
be very difficult to find the seed using buried treasures alone. But being able
to go from 2^48 to 2^42 with only one structure is a nice win.

This optimization can also be applied to all the other structures. I think they
all use variations of this same algorithm, expect mineshafts which will be
explained next.

### Float bias

Similar to modulo bias, there is also a small bias introduced when using
`nextFloat()`.

When using `nextFloat() < 0.01`, the actual probability is not exactly 1%, but
`167773 / 2^24`, or `0.010000050067901611328125`, but I guess the error is too
small to be useful.

This paper about the problems with "Generating Random Floating-Point Numbers by
Dividing Integers" mentions some problems with rounding, but they only affect
the least significant bit, so nothing useful to us:

<https://hal.archives-ouvertes.fr/hal-02427338/document>

## nextDouble

The mineshaft algorithm from version 1.7.2 upwards is:

```
public boolean isMineshaftChunk(long seed, int x, int z) {
    Random r = new Random(seed);
    long i = r.nextLong();
    long j = r.nextLong();
    Random r2 = new Random((x * i) ^ (z * j) ^ seed);
    return r2.nextDouble() < 0.004;
}
```

The first unusual thing is that the constants used to hash x, z, and seed into
one value are not constants: they depend on the world seed. This adds some
protection: now if we know the 10 lower bits of r2, we do not automatically
know the 10 lower bits of the world seed. But there are some special cases.

What happens if i or j turn out to be 0? If i is 0 then the formula turns into

```
    Random r2 = new Random((z * j) ^ seed);
```

and r2 seed does not depend on the x coordinate, which means that all the
chunks with equal z coordinate will have the same mineshaft seed: either they
all spawn a mineshaft, or none of them does. This results in mineshaft spawning
in lines along the z axis:

[Infinite Mineshaft](https://www.youtube.com/watch?v=RssvOCRdDJM)

But leaving that aside, let's take a look at nextDouble:

```java
public double nextDouble() {
    return (((long)next(26) << 27) + next(27)) / (double)(1L << 53);
}
```

It is similar to nextFloat, but it uses two calls to next instead of one.
The returned value is divided by 2^53, meaning that there should be 2^53
different doubles between 0 and 1.

Let's multiply everything by 2^53, similarly to nextFloat:

```
r.nextDouble() < 0.004;
((r.next(26) << 27) + r.next(27)) < ZERO_FOUR_PERCENT_OF_2_53
```

But the two calls to next are simply concatenated, so we can approximate it as:

```java
(r.next(26) << 27) < ZERO_FOUR_PERCENT_OF_2_53
// Multiply both sides by 2^27
r.next(26) < ZERO_FOUR_PERCENT_OF_2_(53 - 27)
r.next(26) < ZERO_FOUR_PERCENT_OF_2_26
```

This uses only one call to next and is accurate up to 2^-27. Technically it may
be possible that `x < 0.004` is true but `(x + 2^-27) < 0.004` is false, so
this approximation has a small probability of false positives, but that doesn't
matter because now we can apply the same optimization as nextFloat.

Fun fact: `r.nextFloat()` and `r.nextDouble()` return approximately the same
value (max error = 2^-29).

`ZERO_FOUR_PERCENT_OF_2_26` is a constant that can be calculated using python:

```py
>>> math.ceil(0.004 * 2**26)
268436
```

Which is a 19 bit number:

```py
>>> math.log2(268436)
18.034218639040017
```

So once again, if we know at least one mineshaft, we get the top (26 - 19 = 7)
bits for free. But note that this time this are not the top 7 bits of the world
seed, but the top 7 bits of the state of r2:

```java
Random r = new Random(seed);
long i = r.nextLong();
long j = r.nextLong();
Random r2 = new Random((x * i) ^ (z * j) ^ seed);
```

So even if we know the internal state of r2 we still need to reverse the
hash operation `(x * i) ^ (z * j) ^ seed`. That's explained in the
[hash functions docs][HashDocs].

# McRng

This other PRNG is only used for biome generation.

It looks unique to Minecraft, so it was probably written specifically with a
purpose in mind.

Its main purpose is probably to use the full 64 bits of the world seed.

Note: never write your own PRNG.

## QCG

The `McRng` is a [Quadratic Congruential Generator (QCG)][QCG] with 64-bit state.

The next state `s(n+1)` can be calculated with the following formula:

```
s(n+1) = A * s(n) * s(n) + C * s(n) + k (mod M)
```

And particularly, it uses the constants:

```
A = 6364136223846793005 // 0x5851f42d4c957f2d
C = 1442695040888963407 // 0x14057b7ef767814f
M = 2^64
```

A curious property is that there is a value `k` that should be constant, but it
is not. `k` is set during initialization, and it depends on the world seed and
on a known "base seed". The designers of this PRNG probably thought that doing
it that way will "add randomness".

The constants A and C are from Knuth's MMIX PRNG, which is not a QCG, but a
plain LCG. That is pretty strange: you can't just copy some constants from a
completely different PRNG and expect them do work. In the [QCG link][QCG] there
is a section about parameter selection:

```
Parameter selection:

If M is a power of 2, then A even, C == A + 1 mod 4, and k odd will guarantee period length = M.

No table of good parameters has been published.
```

M is a power of 2, but A is not even, so the period of this PRNG may not be
2^64. And the reason why no table of good parameters has been published is
simple: nobody uses this PRNG, since it provides zero benefits over a LCG.

## Speed

The speed of this QCG should be in the same order of magnitude that the speed
of the LCG. While a LCG consists of 1 multiplication and 1 addition, a QCG
consists of 3 multiplications and 2 additions. So we can estimate it to be ~3
times slower but in practice that doesn't matter. The important part is that it uses
64 bits of state instead of 48, so we cannot bruteforce it in reasonable time.

For example, if it takes 3 days to bruteforce 2^48, it will take more than 500
years to bruteforce 2^64. But don't worry, you can just buy the equivalent of
500 CPU cores and finish the bruteforce in 1 year. So while 2^64 is a big
number, it's not impossible to break.

## setSeed

This PRNG has a very strage initialization process. There are three different
`setSeed` functions that must be called in the correct order.

All the `setSeed` functions just advance the PRNG state, using the `nextState`
function:

```
static long nextState(long s, long k) {
    return A * s * s + C * s + k;
}
```

First there is a `baseSeed`. This seed depends on where will the PRNG be used.
For example, each of the biome layers has a different seed, but they are
hardcoded constants so we always know the exact value.

```
public long setBaseSeed(long seed) {
    baseSeed = seed;
    baseSeed = nextState(baseSeed, seed);
    baseSeed = nextState(baseSeed, seed);
    baseSeed = nextState(baseSeed, seed);
}
```

Then the `baseSeed` is updated with the `worldSeed`:

```
public long setWorldSeed(long seed) {
    worldSeed = seed;
    worldSeed = nextState(worldSeed, baseSeed);
    worldSeed = nextState(worldSeed, baseSeed);
    worldSeed = nextState(worldSeed, baseSeed);
}
```

And finally there is a `setChunkSeed` for calculating the probability at any
particular chunk:

```
public long setChunkSeed(long chunkX, long chunkZ) {
    chunkSeed = worldSeed;
    chunkSeed = nextState(chunkSeed, chunkX);
    chunkSeed = nextState(chunkSeed, chunkZ);
    chunkSeed = nextState(chunkSeed, chunkX);
    chunkSeed = nextState(chunkSeed, chunkZ);
}
```

Even if it looks complex, all the operations are simple additions and
multiplications, so they are fully reversible. This means that if we can get
the internal state of this PRNG when seeded for a known chunk coordinate, we
also get the world seed.

But there's a big flaw in the `nextState` function. Let's take a look at bit 0,
the least significant bit of the internal state. `A` and `C` are odd so we can
replace them with ones:

```
A * s * s + C * s + k
1 * s * s + 1 * s + k
s * s + s + k
// modulo 2, s * s is always equal to s: 0 * 0 = 0, 1 * 1 = 1
s + s + k
// s + s is always even, we can replace it with a 0
2 * s + k
0 + k
k
```

So in the end the least significant bit of the internal state does only depend
on `k`! And `k` is always known, so we always know 1 bit. This makes this PRNG
effectively 63-bit. That may not sound like a big improvement, but we can skip
half of the bruteforce. Although now we are trying to initialize a 63-bit PRNG
with a 64-bit seed, so this means that some seeds may map to the same PRNG. In
fact the mapping is exactly 2:1. For every 2 world seeds there is 1 PRNG.

Implications: for every world seed there is a different seed that generates
exactly the same biome map. The formula for getting that seed is:

```
similarBiomeSeed = -7379792620528906219 - seed
```

Where does that magic constant come from? Any two similar biome seeds will add
up to -7379792620528906219. So by subtracting one seed from that magic
constant you can find the other seed. But why do similar biome seeds add up to
that constant? Consider two different seeds, s1 and s2, that must result in the
same value:

```
A * s1 * s1 + C * s1 + k = x
A * s2 * s2 + C * s2 + k = x

A * s1 * s1 + C * s1 + k = A * s2 * s2 + C * s2 + k
A * s1 * s1 + C * s1 = A * s2 * s2 + C * s2
A * s1 * s1 + C * s1 - A * s2 * s2 + C * s2 = 0

# A * A_INV = 1 (mod 2^64)

A * s1 * s1 * A_INV + C * s1 * A_INV - A * s2 * s2 * A_INV + C * s2 * A_INV = 0
s1 * s1 + A_INV * C * s1 - s2 * s2 + A_INV * C * s2 = 0

# A_INV * C = IAC

s1 * s1 + IAC * s1 - s2 * s2 + IAC * s2 = 0
s1 * s1 + IAC * s1 - s2 * s2 + IAC * s2 = 0
IAC * (s1 - s2) = s2 * s2 - s1 * s1

# (a * a - b * b) = (a + b) * (a - b)

IAC * (s1 - s2) = (s2 + s1) * (s2 - s1)

# (s1 - s2) = (-1) * (s2 - s1)

IAC * (-1) * (s2 - s1) = (s2 + s1) * (s2 - s1)

# Assuming that (s2 - s1) is not 0:

IAC * (-1) = (s2 + s1)

# IAC * (-1) = 2^64 - IAC

2^64 - IAC = (s2 + s1)
```

This means that for two different seeds s1 and s2 to generate the same seed,
they must follow the formula `s1 + s2 = 2^64 - IAC`, where `s1, s2, IAC` are
64-bit unsigned integers. Java uses signed integers, but fortunately this
formula still applies.


Now, the concrete values:

```
A = 6364136223846793005
C = 1442695040888963407
A_INV = A^-1 = 13877824140714322085
IAC = A^-1 * C = 7379792620528906219
2^64 - IAC = 11066951453180645397
# Convert to 64-bit signed integer:
2^64 - IAC = -7379792620528906219
```

[WolframAlpha](https://www.wolframalpha.com/input/?i=PowerMod%5B6364136223846793005%2C+-1%2C+2%5E64%5D+*+1442695040888963407+mod+2%5E64) can calculate `IAC = A^-1 * C` directly.

In versions of Minecraft before 1.15 this two seeds resulted in identical biome
maps up to 1:1 resolution, but since 1.15 the last biome generation step uses a
different seed so the biomes only match up to 1:4 resolution.

# Minecraft seed initialization

There are 3 ways to set the seed of a new world:

* (1) Leave it blank (it will generate a random seed)
* (2) Input a string (it will hash the string to a 32-bit signed integer)
* (3) Input a 64-bit signed integer (it will use that number as the seed)

For a more in depth explanation, check [Panda4994's video about ways to enter a
seed][PandaEnterSeed].

Method (2) results in a 32-bit seed, which can be easily bruteforced.

Method (3) results in a 64-bit seed.

But does method (1) also result in a 64-bit seed? Well, I said that Minecraft
uses `JavaRandom` for almost everything. This includes generating a random
world seed when we create a new world:

```
this.seed = (new Random()).nextLong();
```

`nextLong` is a function which returns a 64-bit integer by combining two 32-bit
integers:

## nextLong

```
public long nextLong() {
    return ((long)next(32) << 32) + next(32);
}
```

But if the PRNG only has 48 bits of state, how can it generate 64-bit numbers?

Well, it obviously cannot generate all the possible 64-bit integers, only 2^48
of them. The most intuitive way to understand this is that after 2^48 calls to
`next()`, the PRNG will return to the initial state. So no matter what was the
initial state, there are only 2^48 possible initial states, and 2^48 possible
different 64-bit integers.

So, this results in a great optimization that we can use to reverse engineer
Minecraft seeds:

> If the seed was left empty when creating the world, there are only 2^48
  possible seeds

This implies that if we have 2^16 candidates, we can filter out the ones that
cannot be generated using `nextLong`.

We can create a function `canBeFromNextLong(long l)` which given a 64-bit
number `l` will return true if that number can be obtained from a call to
`nextLong`. Statistically speaking, this function will only return true once
for every 2^16 values.

```
public boolean canBeFromNextLong(long l) {
    // get ints from long
    int i0 = (l >> 32) + ((l >> 31) & 1);
    int i1 = l;

    int x = i1 - i0 * A;
    for (int i = 0; i < 6; i++) {
        int low16 = (x | (i << 32)) / (A >> 16);
        int y = ((long)low16 * A + C) >> 16;
        if (y == x) {
            return true;
        }
    }

    // It is impossible for nextLong() to return l
    return false;
}
```

As far as I know most custom minecraft servers also generate the world seed
this way, although if you want to make your server secure this is very easy to
fix: just use `java.security.SecureRandom` to generate the initial seed:

```
this.seed = (new SecureRandom()).nextLong();
```

`SecureRandom` is platform dependend: it will use `/dev/random` on Linux, and
similar random devices on other operating systems. If there are no available
random devices, it will use a fallback cryptographically secure PRNG (CSPRNG)
with 160 bits of state (the output of SHA1), but what's important is that it is
initialized from OS entropy (current time, PID, etc.), which probably has more
than 160 bits of entropy.

[More info about SecureRandom][JavaSecureRandom]

Note: if you have ever wondered about the internals of a CSPRNG, it's very
simple. They just use a cryptographically secure hash function to calculate the
next state:

```
state(n+1) = (state(n) + sha1(state(n)) + 1) % 2^160;


# Appendix A: Modulo bias

What is modulo bias?

Let's make a simple example with a 4-bit PRNG which returns integers in the
[0, 15] range. And we want a multiple of 5, to choose on item from the set
(A, B, C, D, E).

Let's take a look at all the possibilities:

```
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
A B C D E A B C D E A  B  C  D  E  A
```

As you can see, the sequence starts with A and ends with A, so choosing A is
more likely than the other items. In fact the probabilities are:

```
A = 4 / 16
B = 3 / 16
C = 3 / 16
D = 3 / 16
E = 3 / 16
```

The usual way to avoid this problem is to reroll when the PRNG returns 15, that
would update the probabilities to:

```
A = 3 / 15
B = 3 / 15
C = 3 / 15
D = 3 / 15
E = 3 / 15
```

But the cost is that there is a `(1 / 16)^n` probability of getting stuck in a
loop for `n` cycles.

The worst case for our example would be returning an integer modulo 9:

```
0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15
A B C D E F G H I A B  C  D  E  F  G
```

In this case, without the modulo bias correction, the probability of selecting
`H` or `I` is half of the probability of selecting anything else!

Now we have to reroll everytime the generated number is greater than or equal
to 9. There is a `(7 / 15)^n` probability of getting stuck for `n` cycles.

In general, we have to reroll every time the generated number is greater than
or equal to the last multiple of n below 2^4. That would be 15 for n=5 and 9
for n=9. You can quickly calculate that values in Python as:

```py
def last_multiple_of_n_below(n, m):
    return m // n * n

def probability_of_modulo_bias(n, m):
    return 1 - last_multiple_of_n_below(n, m) / m
```

In the scenario of minecraft we can ignore the modulo bias check, because the
worst case is when using `nextInt(2^30 + 1)` and we are usually dealing with n
below 1\_000, for which the probability of doing a modulo bias reroll is below
1 in 2.3 million.

# Appendix B: magic constants in the wild

When searching the LCG constants on Google I found some really cool projects:

<https://github.com/DaMatrix/bedrock>

<https://github.com/hube12/GPUExample>

And even some GPU seed finders posted on pastebins:

<https://ideone.com/uW9au0>

<https://pastebin.com/nhvWjJjN>

<!-- Sources -->
[PandaEnterSeed]: https://www.youtube.com/watch?v=OLS7CCgNcuY
[ModMultInv]: https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
[Xorshift]: https://en.wikipedia.org/wiki/Xorshift
[JavaSecureRandom]: https://metebalci.com/blog/everything-about-javas-securerandom/
[LCG]: https://en.wikipedia.org/wiki/Linear_congruential_generator
[QCG]: http://statmath.wu.ac.at/prng/doc/prng.html#QCG
[JavaUtilRandom]: https://docs.oracle.com/javase/8/docs/api/java/util/Random.html
[HashDocs]: https://github.com/Badel2/slime_seed_finder/blob/master/docs/hash_functions.md
