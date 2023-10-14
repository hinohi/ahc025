use std::collections::HashMap;
use rand::distributions::Distribution;
use rand_distr::Exp;
use rand_pcg::Mcg128Xsl64;

fn main() {
    let n = 50;
    let d = n / 8;
    let exp = Exp::new(1e-5).unwrap();
    let mut rng = Mcg128Xsl64::new(1);
    let mut hist = HashMap::new();
    let sample = 100000000;
    for _ in 0..sample {
        let mut v = (0..n).map(|_| loop {
            let w = exp.sample(&mut rng);
            if w <= n as f64 * 1e5 / d as f64 {
                break (f64::round(w) as i64).max(1);
            }
        }).collect::<Vec<_>>();
        v.sort_unstable();
        for (i, &w) in v.iter().enumerate() {
            hist.entry(w).or_insert_with(|| vec![0u32; n])[i] += 1;
        }
    }
    let mut keys = hist.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    for w in keys {
        print!("{} ", w);
        for h in hist[w].iter() {
            print!("{} ", *h as f64 / sample as f64);
        }
        println!();
    }
}
