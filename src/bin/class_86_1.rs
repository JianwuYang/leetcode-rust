use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());

    let mut buf = String::new();

    br.read_line(&mut buf)?;

    let s1: Vec<char> = buf.chars().collect();
    buf.clear();

    br.read_line(&mut buf)?;

    let s2: Vec<char> = buf.chars().collect();

    let mut n = s1.len();
    let mut m = s2.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];

    compute(&s1, &s2, &mut dp);

    let mut len = dp[n][m];

    if len == 0 {
        Write::write_all(&mut bw, "-1".as_bytes())?;
    } else {
        let mut ans = vec!['.'; len];

        while len > 0 {
            if s1[n - 1] == s2[m - 1] {
                len -= 1;
                ans[len] = s1[n - 1];
                n -= 1;
                m -= 1;
            } else if dp[n - 1][m] >= dp[n][m - 1] {
                n -= 1;
            } else {
                m -= 1;
            }
        }

        Write::write_all(&mut bw, ans.iter().collect::<String>().as_bytes())?;
    }

    bw.flush()?;
    Ok(())
}

pub fn compute(s1: &[char], s2: &[char], dp: &mut [Vec<usize>]) {
    for i in 1..=s1.len() {
        for j in 1..=s2.len() {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
}
