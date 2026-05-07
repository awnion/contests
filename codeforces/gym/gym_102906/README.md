# Problem F: Not a Subsequence

[https://codeforces.com/gym/102906/problem/F](https://codeforces.com/gym/102906/problem/F)

Time limit: 1 second  
Memory limit: 512 megabytes

Given two integer sequences `A` and `B`, find the shortest possible sequence `C`
that is not a subsequence of either `A` or `B`.

A sequence `X = [x1, x2, ..., xt]` is a subsequence of a sequence
`Y = [y1, y2, ..., ys]` if `X` can be obtained from `Y` by deleting zero or more
elements without changing the order of the remaining elements. Equivalently,
there must be indices `1 <= i1 < i2 < ... < it <= s` such that
`xj = yij` for every `1 <= j <= t`.

All elements in `A`, `B`, and the answer sequence `C` must be integers from
`1` to `k`.

## Input

The input contains:

1. An integer `k` - the maximum allowed value of a sequence element.
2. An integer `m` - the length of sequence `A`.
3. `m` integers describing sequence `A`.
4. An integer `n` - the length of sequence `B`.
5. `n` integers describing sequence `B`.

## Output

Print the length `p` of the required sequence `C` on the first line.

On the second line, print the sequence `C` itself.

If there are several shortest valid sequences, print any one of them.

## Constraints

- `1 <= k <= 5000`
- `1 <= m <= 5000`
- `1 <= n <= 5000`
- Every element of `A` and `B` is between `1` and `k`.
