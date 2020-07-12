#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [(usize, usize); m]
    }
    let mut ans = -1;
    let mut c: Vec<usize> = vec![10; n];
    for i in s.iter() {
        if c[n - i.0] == 10 || c[n - i.0] == i.1 {
            c[n - i.0] = i.1;
        } else if c[n - i.0] != i.1 {
            println!("-1");
            return;
        }
    }
    if c[0] == 0 {
        println!("-1");
        return;
    }
    let mut tmp = 0;
    for j in 0..n {
        if c[j] == 10 {
            continue;
        }
        tmp += c[j] * 10 ^ (n - j);
    }
    println!("{:?}", tmp);
}
