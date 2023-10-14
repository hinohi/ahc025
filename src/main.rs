use std::collections::BTreeMap;
use rand::distributions::Distribution;
use rand_distr::Exp;
use rand_pcg::Mcg128Xsl64;

fn main() {
    let n = 50;
    let d = n / 8;
    let exp = Exp::new(1e-5).unwrap();
    let mut rng = Mcg128Xsl64::new(1);
    let mut hist = BTreeMap::<u32, u32>::new();
    let sample = 1000000000;
    for _ in 0..sample {
        let mut v = (0..n).map(|_| loop {
            let w = exp.sample(&mut rng);
            if w <= n as f64 * 1e5 / d as f64 {
                break (f64::round(w) as i64).max(1);
            }
        }).collect::<Vec<_>>();
        let mut c = 0;
        v.sort_by(|a, b| {
            c += 1;
            a.cmp(b)
        });
        *hist.entry(c).or_default() += 1;
    }
    for (c, h) in hist.iter() {
        println!("{} {}", c, h);
    }
}
