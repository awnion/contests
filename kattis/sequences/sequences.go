package main

import (
	"bufio"
	"fmt"
	"os"
)

func f1(x int64, p int64) int64{
	return (x << 1) % p
}

func f2(x int64, o int64, q int64, p int64) int64{
	return (x + o * q) % p
}

func main() {
	var mod int64 = 1000000007
	var q int64 = 500000004
	var r int64 = 0
	var o int64 = 0

	line, _ := bufio.NewReader(os.Stdin).ReadString('\n')
	
	for _, c := range line {
		if c == '?' {
			r = f1(r, mod)
			r = f2(r, o, q, mod)

			q = f1(q, mod)

			o += 1
		} else if c == '0' {
			r = f2(r, o, q, mod)
		} else {
			o += 2
		}
	}

	fmt.Printf("%d\n", r)
}
