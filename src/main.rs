use std::str::FromStr;

mod bits;
use bits::Bits;

fn main() {
    let b0: Bits<5> = Bits::from_str("10001").unwrap();
    let b1: Bits<5> = Bits::from_str("11111").unwrap();
    let t0 = Vec::from_iter(b0.upper_shadow());
    let t1 = Vec::from_iter(b0.lower_shadow());
    let t2 = Vec::from_iter(b0.ones());
    let t3 = Vec::from_iter(b0.zeroes());

    println!("{}\n{:?}\n{:?}\n{:?}\n{:?}", b0, t0, t1, t2, t3);

    let t4 = Vec::from_iter(b0.paths(&b1).unwrap());

    println!("{:?}\n", t4);

    let t5 = Vec::from_iter(b0.converge(&b1).unwrap());
    let t6 = Vec::from_iter(b1.converge(&b0).unwrap());

    println!("{:?}", t5);
    println!("{:?}\n", t6);

    let t7 = Vec::from_iter(b0.midpoints(&b1).unwrap());

    println!("{:?}", t7);
}
