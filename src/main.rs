use std::collections::HashSet;
mod bits;
use bits::Bits;
mod util;
use petgraph::{
    dot::{Config, Dot},
    graph::NodeIndex,
    Graph, Undirected,
};
use std::{collections::HashMap, fs::File, io::Write, str::FromStr};

#[derive(Clone, Debug)]
pub struct MonotoneFunction<const N: usize> {
    implicants: HashSet<Bits<N>>,
}

impl<const N: usize> MonotoneFunction<N> {
    pub fn new(implicants: Vec<Bits<N>>) -> Self {
        let mut reduced = HashSet::<Bits<N>>::new();

        'outer: for implicant1 in &implicants {
            for implicant2 in &implicants {
                if *implicant1 > *implicant2 {
                    continue 'outer;
                }
            }

            reduced.insert(*implicant1);
        }

        Self {
            implicants: reduced,
        }
    }

    pub fn call(&self, x: Bits<N>) -> bool {
        self.implicants.iter().map(|a| *a & x == *a).any(|b| b)
    }
}

pub struct Learner<const N: usize> {
    oracle: MonotoneFunction<N>,
    lower_frontier: HashSet<Bits<N>>,
    upper_frontier: HashSet<Bits<N>>,
    iterations: usize,
}

impl<const N: usize> Learner<N> {
    pub fn new(oracle: MonotoneFunction<N>) -> Self {
        Self {
            oracle,
            lower_frontier: HashSet::new(),
            upper_frontier: HashSet::new(),
            iterations: 0,
        }
    }

    pub fn iterate(&mut self) {
        if self.iterations == 0 {
            assert!(self.lower_frontier.is_empty());
            assert!(self.upper_frontier.is_empty());

            let eet = Bits::<N>::new(false);
            let tee = Bits::<N>::new(true);
            let x = eet.rand_midpoint(&tee).unwrap();

            if self.oracle.call(x) {
                self.upper_frontier.insert(x);
            } else {
                self.lower_frontier.insert(x);
            }
        }

        self.iterations += 1;
    }

    pub fn graph(&self) -> Graph<(Bits<N>, &str), (), Undirected> {
        let mut graph = Graph::<(Bits<N>, &str), (), Undirected>::new_undirected();
        let mut history = HashMap::<Bits<N>, NodeIndex>::new();

        for n in 0..2_u64.pow(N as u32) {
            let b: Bits<N> = n.try_into().unwrap();
            let mut done = false;

            for implicant in &self.lower_frontier {
                if b == *implicant {
                    let i = graph.add_node((b, "L"));
                    history.insert(b, i);
                    done = true;

                    break;
                } else if b < *implicant {
                    let i = graph.add_node((b, "X"));
                    history.insert(b, i);
                    done = true;

                    break;
                }
            }

            for implicant in &self.upper_frontier {
                if b == *implicant {
                    let i = graph.add_node((b, "U"));
                    history.insert(b, i);
                    done = true;

                    break;
                } else if b > *implicant {
                    let i = graph.add_node((b, "X"));
                    history.insert(b, i);
                    done = true;

                    break;
                }
            }

            if !done {
                let i = graph.add_node((b, ""));
                history.insert(b, i);
            }
        }

        for (b0, i0) in &history {
            for b1 in b0.horizon(false) {
                let i1 = history.get(&b1).unwrap();
                let _ = graph.add_edge(*i1, *i0, ());
            }
        }

        graph
    }
}

const K: usize = 4;

fn main() {
    let mut bs = Vec::<Bits<K>>::new();

    for n in 0..2_u32.pow(K as u32) {
        let b = n.try_into().unwrap();
        bs.push(b);
    }

    let f = MonotoneFunction::<K>::new(vec![Bits::<K>::from_str("1010").unwrap()]);

    let mut learner = Learner::<K>::new(f);

    let dot = format!(
        "{:?}",
        Dot::with_config(&learner.graph(), &[Config::EdgeNoLabel])
    );
    let mut out = File::create("./test0.dot").expect("Unable to create file");
    out.write_all(dot.as_bytes()).expect("Unable to write data");

    learner.iterate();
    learner.iterate();

    let dot = format!(
        "{:?}",
        Dot::with_config(&learner.graph(), &[Config::EdgeNoLabel])
    );
    let mut out = File::create("./test1.dot").expect("Unable to create file");
    out.write_all(dot.as_bytes()).expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const N: usize = 8;

    #[test]
    fn test_midpoint_distribution() {
        let eet = Bits::<N>::new(false);
        let tee = Bits::<N>::new(true);
        let count = 100000_usize;
        let mut counts = [0_usize; N];

        for _ in 0..count {
            let t0 = eet.rand_midpoint(&tee).unwrap();

            for j in 0..t0.len() {
                if t0[j] {
                    counts[j] += 1;
                }
            }
        }

        let mut dist = [0_f64; N];

        for i in 0..N {
            dist[i] = counts[i] as f64 / count as f64;
            assert!(approx_eq!(f64, dist[i], 0.5, epsilon = 0.01));
        }

        println!("{:?}", dist);
    }
}
