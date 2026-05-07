#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckVerdict {
    Accepted,
    WrongAnswer,
    CheckerFailure,
}

impl CheckVerdict {
    pub fn label(self) -> &'static str {
        match self {
            CheckVerdict::Accepted => "OK",
            CheckVerdict::WrongAnswer => "WA",
            CheckVerdict::CheckerFailure => "FL",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CheckOutcome {
    pub verdict: CheckVerdict,
    pub message: String,
}

impl CheckOutcome {
    fn accepted(message: impl Into<String>) -> Self {
        Self {
            verdict: CheckVerdict::Accepted,
            message: message.into(),
        }
    }

    fn wrong_answer(message: impl Into<String>) -> Self {
        Self {
            verdict: CheckVerdict::WrongAnswer,
            message: message.into(),
        }
    }

    fn checker_failure(message: impl Into<String>) -> Self {
        Self {
            verdict: CheckVerdict::CheckerFailure,
            message: message.into(),
        }
    }
}

#[derive(Clone, Copy)]
enum BadInputVerdict {
    WrongAnswer,
    CheckerFailure,
}

struct Scanner<'a> {
    name: &'static str,
    data: std::str::SplitWhitespace<'a>,
    bad_verdict: BadInputVerdict,
}

impl<'a> Scanner<'a> {
    fn new(name: &'static str, text: &'a str, bad_verdict: BadInputVerdict) -> Self {
        Self {
            name,
            data: text.split_whitespace(),
            bad_verdict,
        }
    }

    fn next_i64(&mut self, what: &str) -> Result<i64, CheckOutcome> {
        let Some(token) = self.data.next() else {
            return Err(self.bad(format!("missing {} in {}", what, self.name)));
        };

        token
            .parse::<i64>()
            .map_err(|_| self.bad(format!("invalid {} in {}: {}", what, self.name, token)))
    }

    fn bad(&self, message: String) -> CheckOutcome {
        match self.bad_verdict {
            BadInputVerdict::WrongAnswer => CheckOutcome::wrong_answer(message),
            BadInputVerdict::CheckerFailure => CheckOutcome::checker_failure(message),
        }
    }
}

pub fn check(inf_text: &str, ouf_text: &str, ans_text: &str) -> CheckOutcome {
    match check_inner(inf_text, ouf_text, ans_text) {
        Ok(outcome) => outcome,
        Err(outcome) => outcome,
    }
}

fn check_inner(
    inf_text: &str,
    ouf_text: &str,
    ans_text: &str,
) -> Result<CheckOutcome, CheckOutcome> {
    let mut inf = Scanner::new("inf", inf_text, BadInputVerdict::CheckerFailure);
    let mut ouf = Scanner::new("ouf", ouf_text, BadInputVerdict::WrongAnswer);
    let mut ans = Scanner::new("ans", ans_text, BadInputVerdict::CheckerFailure);

    let ja = read_len(&mut ans, "answer length")?;
    let pa = read_len(&mut ouf, "answer length")?;
    if ja < pa {
        return Ok(CheckOutcome::wrong_answer(format!(
            "expected: {}, found: {}",
            ja, pa
        )));
    }

    let k = inf.next_i64("k")?;
    let m = read_len(&mut inf, "m")?;
    let mut a = Vec::with_capacity(m);
    for idx in 0..m {
        a.push(inf.next_i64(&format!("a[{}]", idx))?);
    }

    let n = read_len(&mut inf, "n")?;
    let mut b = Vec::with_capacity(n);
    for idx in 0..n {
        b.push(inf.next_i64(&format!("b[{}]", idx))?);
    }

    let mut c = Vec::with_capacity(pa);
    for idx in 0..pa {
        let value = ouf.next_i64(&format!("c[{}]", idx))?;
        if value < 1 || value > k {
            return Ok(CheckOutcome::wrong_answer(format!(
                "invalid subsequence element {}",
                value
            )));
        }
        c.push(value);
    }

    if is_subsequence(&c, &a) {
        return Ok(CheckOutcome::wrong_answer("subsequence of a"));
    }
    if is_subsequence(&c, &b) {
        return Ok(CheckOutcome::wrong_answer("subsequence of b"));
    }

    if ja > pa {
        return Ok(CheckOutcome::checker_failure(format!(
            "expected: {}, found: {}",
            ja, pa
        )));
    }

    Ok(CheckOutcome::accepted(format!(
        "k = {}, n = {}, m = {}, ans = {}",
        k, n, m, ja
    )))
}

fn read_len(scanner: &mut Scanner<'_>, what: &str) -> Result<usize, CheckOutcome> {
    let value = scanner.next_i64(what)?;
    if value < 0 {
        return Err(scanner.bad(format!("invalid {} in {}: {}", what, scanner.name, value)));
    }
    Ok(value as usize)
}

fn is_subsequence(needle: &[i64], haystack: &[i64]) -> bool {
    let mut pos = 0usize;
    for &x in haystack {
        if pos < needle.len() && needle[pos] == x {
            pos += 1;
        }
    }
    pos == needle.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_non_subsequence() {
        let inf = "2\n1\n1\n1\n2\n";
        let ans = "2\n1 2\n";
        let ouf = "2\n1 2\n";

        let outcome = check(inf, ouf, ans);

        assert_eq!(outcome.verdict, CheckVerdict::Accepted);
    }

    #[test]
    fn rejects_subsequence_of_a() {
        let inf = "2\n1\n1\n1\n2\n";
        let ans = "1\n2\n";
        let ouf = "1\n1\n";

        let outcome = check(inf, ouf, ans);

        assert_eq!(outcome.verdict, CheckVerdict::WrongAnswer);
    }
}
