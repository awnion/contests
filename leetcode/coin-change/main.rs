struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = amount as usize + 1;
        let mut t = vec![amount + 1; n];
        t[0] = 0;
        for i in 0..n {
            let current = t[i];
            for &coin in &coins {
                let coin = coin as usize;
                if i + coin < n {
                    let x = t[i + coin];
                    if x > current + 1 {
                        t[i + coin] = current + 1;
                    }
                }
            }
        }
        if t[amount as usize] > amount {
            return -1;
        }
        t[amount as usize]
    }
}

fn main() {
    println!("{}", Solution::coin_change(vec![1, 5, 10], 192));
}
