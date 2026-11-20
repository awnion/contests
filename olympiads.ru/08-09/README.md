# olympiads.ru 2008–09

> This is an AI-written review of the solutions in this folder, dated 2026-11-20.
> The original problem statements were not available, so the problems below are
> reconstructed from the code and may be inaccurate.

## Problems

- **A — Table tennis.** Given the score between Vasya and Petya in a game to 21
  with the serve changing every 5 points, say who serves now or who has won.
- **B — Waiting at the bus stop.** Vasya arrives at a stop at a given time.
  Several routes each have a travel time to the stop and a list of departure
  times. Find the shortest wait for the next bus, wrapping around midnight.
- **C — Anagrams.** Given a word and a list of words, print those made of
  exactly the same letters.
- **D — Splitting between two players.** Two players take turns picking the
  largest remaining number; print the two final totals, larger first.
- **E — Route with a timetable.** There are stations and one-way trips, each
  departing every `C` minutes and taking `D` minutes. Visit a given sequence of
  points in order as early as possible, or report that it is impossible.
- **G — Constructive sequence.** For a given `N`, print `N(N+1)/2` and the
  sequence `N..1, N..2, …, N, 1`. The property this sequence is meant to satisfy
  cannot be recovered from the code.
- **H — Shortest chain of operations.** Turn `a` into `b` using `+3` and `×4`
  in the fewest steps, or report that it is impossible.

## Review

**A, B, C, D** look correct.

### E — `e.cpp`

The algorithm is right: Dijkstra where each edge first waits for the next
departure is valid here. On the generated `e.in` it prints 98, the same as the
Python version. The implementation has a few weak spots:

- **One read too many.** The loop reads `M` points after the first one, so
  `M + 1` in total, while `e5.py` reads `M`. The last read fails at end of
  file, `p2` keeps its value, and the extra `get_path(p1, p1)` call changes
  nothing. It works, but only by accident.
- **Time stored as `double`.** `ceil(t / period)` can go wrong on large values.
  `long long` with `(t + C - 1) / C * C + D` is safer.
- **Memory leaks.** Every one of the `M` runs allocates `new Pair` objects that
  are never freed. A plain `dist[]` array would be simpler.
- **Array size.** `E[MAXE]` is indexed by vertex but sized by the number of
  edges.

### E — `e*.py`

The priority queue is a sorted list driven by `bisect`, `insert` and `pop(0)`,
which costs O(n) per operation. `Visited` is filled in but never checked. Five
versions plus `psyco` suggest a fight with the time limit; `heapq` (in the
standard library since Python 2.3) would have settled it.

### H

The layer-by-layer BFS gives the right answer, but slowly:

- `not in a` on a list and `pop(0)` are both linear, and there is no global
  visited set. For large `b` this will not fit the time limit; a `set` or a
  distance array is needed.
- With `a = 0` and an unreachable `b` the loop never ends, because `0 * 4 = 0`
  stays in the queue forever.

### G

Whether the construction is correct cannot be checked without the statement.
On the technical side, `res += ...` over roughly N²/2 numbers is quadratic;
collecting a list and calling `' '.join` is better.

### All Python solutions

`IN.pop(0)` in a loop costs O(n) per call, which becomes noticeable on large
inputs such as the 100,000 numbers produced by `d_gen.py`. Walking the list by
index or with an iterator avoids this.
