#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [(Usize1, char); m]
    }
    let mut ans = vec!['_'; n];
    for &(s, c) in s.iter() {
        if ans[s] != '_' && ans[s] != c {
            println!("-1");
            return;
        }
        ans[s] = c;
    }
    if ans[0] == '0' && n != 1 {
        println!("-1");
        return;
    }
    if ans[0] == '_' && n != 1 {
        ans[0] = '1';
    }
    for &c in ans.iter() {
        if c == '_' {
            print!("0");
        } else {
            print!("{}", c);
        }
    }
}
