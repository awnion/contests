import kotlin.math.max
import kotlin.math.min

data class Solution constructor(
    val k: Int,
    val a: List<Int>,
    val b: List<Int>,
) {
    private lateinit var na: IntArray
    private lateinit var nb: IntArray
    private lateinit var cache: IntArray
    private var n: Int = 0
    private var m: Int = 0
    private var shift: Int = 0

    private fun makeRefs(k: Int, a: List<Int>): IntArray {
        val size = a.size + 2
        val r = IntArray(size * k) { a.size + 1 }
        var leftShift: Int
        var rightShift: Int
        for (j in a.size - 1 downTo 0) {
            leftShift = j * size
            rightShift = leftShift + size

            for (el in 0 until k) r[leftShift + el] = r[rightShift + el]
            r[leftShift + a[j] - 1] = j + 1
        }
        return r
    }

    fun init() {
        n = a.size
        m = b.size
        na = makeRefs(k, a)
        nb = makeRefs(k, b)
        shift = n + 2
        cache = IntArray(shift * (m + 2)) { Int.MAX_VALUE }
    }

    fun dp(i: Int, j: Int): Int {
        if (i > n && j > m) { return 0 }

        val h = shift*i + j
        var r = cache[h]
        if (r < Int.MAX_VALUE) { return r }

        r = Int.MAX_VALUE
        for (el in 0 until k) {
            r = min(r, dp(na[(n + 2)*i + el], nb[(m + 2)*j + el]) + 1)
        }
        cache[h] = r
        return r
    }

    fun backtrack(i: Int, j: Int, r: Int) {
        if (r == 0) return
        for (el in 0 until k) {
            if (r - 1 == dp(na[(n + 2)*i + el], nb[(m + 2)*j + el]) + 1) {
                print("${el + 1} ")
                backtrack(na[(n + 2)*i + el], nb[(m + 2)*j + el], r - 1)
                break
            }
        }
    }
}

fun main() {
    val k = readInt()
    readLine()
    val a = readInts()
    readLine()
    val b = readInts()

    if (k == 1) {
        val l = max(a.size, b.size) + 1
        println("$l")
        repeat(l) { print("1 ") }
        return
    }

    var solution = Solution(k, a, b)
    solution.init()
    val r = solution.dp(0, 0)
    println(r)
    solution.backtrack(0, 0, r)
}

private fun readInt() = readLine()!!.toInt()
private fun readInts() = readLine()!!.split(' ').map { it.toInt() }
