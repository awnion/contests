struct Solution {}

impl Solution {
    fn check(s: &String, i: usize, j: usize) -> (usize, usize) {
        let s = s.as_bytes();

        if s[i] != s[j] {
            return (i, i);
        }

        let max_iters = std::cmp::min(i, s.len() - 1 - j);
        for k in 1..=max_iters {
            if s[i - k] != s[j + k] {
                return (i - k + 1, j + k - 1);
            }
        }
        (i - max_iters, j + max_iters)
    }
    pub fn longest_palindrome(s: String) -> String {
        let mut left = 0;
        let mut right = 0;

        if s.len() < 2 {
            return s;
        }

        for i in 0..s.len() - 1 {
            let (l, r) = Solution::check(&s, i, i);
            if r - l > right - left {
                left = l;
                right = r;
            }
            let (l, r) = Solution::check(&s, i, i + 1);
            if r - l > right - left {
                left = l;
                right = r;
            }
        }
        s[left..=right].to_string()
    }
}

fn main() {
    println!(
        "{}",
        Solution::longest_palindrome("cbbd".to_string()),
    );
}
