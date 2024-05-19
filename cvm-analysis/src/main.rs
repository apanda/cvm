use count_unique_cvm::CountUnique;

use clap::command;
use clap::Parser;
use rand::prelude::*;
use rand::rngs::StdRng;
use statrs::statistics::Data;
use statrs::statistics::Distribution;
use statrs::statistics::Median;

use std::collections::HashSet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    treap_size: usize,
    tokens: u64,
    #[arg(short, long, default_value_t = 10)]
    repeat: usize,
    #[arg(short, long, default_value_t = 23)]
    min_token: u64,
    #[arg(short, long, default_value_t = 65535)]
    max_token: u64,
    #[arg(short, long)]
    plot: Option<String>,
}

struct Estimator<R: Rng> {
    cvm: CountUnique<u64, R>,
    pub estimates: Vec<f64>,
    pub actual_values: Vec<usize>,
    min_estimate: f64,
    max_estimate: f64,
    min_token: u64,
    max_token: u64,
    stream_rng: R,
    token_set: HashSet<u64>,
}

impl<R: Rng> Estimator<R> {
    pub fn new(stream_rng: R, min: u64, max: u64, treap_rng: R, sz: usize) -> Self {
        Estimator {
            cvm: CountUnique::new(treap_rng, sz),
            estimates: vec![],
            actual_values: vec![],
            min_estimate: f64::MAX,
            max_estimate: f64::MIN,
            stream_rng: stream_rng,
            min_token: min,
            max_token: max,
            token_set: Default::default(),
        }
    }
    pub fn estimate_tokens(&mut self, tokens: u64) -> (f64, usize) {
        self.cvm.reset();
        self.token_set.clear();
        for _ in 0..tokens {
            let tok = self.stream_rng.gen_range(self.min_token..=self.max_token);
            self.token_set.insert(tok);
            self.cvm.add_token(tok);
        }
        let estimate = self.cvm.estimate().unwrap();
        if self.min_estimate >= estimate {
            self.min_estimate = estimate
        };
        if self.max_estimate <= estimate {
            self.max_estimate = estimate
        };
        self.estimates.push(estimate);
        self.actual_values.push(self.token_set.len());
        (estimate, self.token_set.len())
    }
}

fn main() {
    let args = Args::parse();
    let mut estimator = Estimator::new(
        StdRng::from_entropy(),
        args.min_token,
        args.max_token,
        StdRng::from_entropy(),
        args.treap_size,
    );
    for _ in 0..args.repeat {
        let _ = estimator.estimate_tokens(args.tokens);
    }

    let estimated = Data::new(estimator.estimates);
    let real_as_vec: Vec<f64> = estimator
        .actual_values
        .iter()
        .map(|v| (*v) as f64)
        .collect();
    let real = Data::new(real_as_vec);
    println!(
        "Means: {} {}",
        estimated.mean().unwrap(),
        real.mean().unwrap()
    );
    println!("Medians: {} {}", estimated.median(), real.median());
    println!(
        "Variance: {} {}",
        estimated.variance().unwrap(),
        real.variance().unwrap()
    );
    // TODO: Add plotting.
}
