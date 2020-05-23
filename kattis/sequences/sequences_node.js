const readline = require('readline');

function f1(x, p) {
    return (x << 1) % p;
}

function f2(x, o, q, p) {
    return (x + o * q) % p;
}

const rl = readline.createInterface({
    input: process.stdin
});

rl.on('line', (line) => {
    const mod = 1000000007
    var r = 0
    var o = 0
    var q = 500000004

    for (var i = 0; i < line.length; i++) {
        if (line[i] == '?') {
            r = f1(r, mod)
            r = f2(r, o, q, mod)
    
            q = f1(q, mod)
    
            o += 1
        } else if (line[i] == '0') {
            r = f2(r, o, q, mod)
        } else {
            o += 2
        }
    }
    console.log(r)
});
