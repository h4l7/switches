use std::str::FromStr;

mod bits;
use bits::Bits;

fn main() {
    let b0: Bits<24> = Bits::from_str("000000000000000000000000").unwrap();
    let b1: Bits<24> = Bits::from_str("111111111111111111111111").unwrap();
    // let t0 = b0.upper_shadow();

    // for t in t0 {
    //     println!("{:?}", t);
    // }

    // let t1 = b0.lower_shadow();
    // println!("{}\n{:?}\n{:?}\n{:?}\n{:?}", b0, t0, t1, t2, t3);

    let t4 = b0.midpoints(&b1).unwrap();
    let mut count = 0_usize;

    for t in t4 {
        println!("{:?}", t);
        count += 1;
    }

    println!("COUNT {:?}", count);

    // let t5 = b0.converge(&b1).unwrap();
    // let t6 = b1.converge(&b0).unwrap();

    // println!("{:?}", t5);
    // println!("{:?}\n", t6);

    // let t7 = b0.midpoints(&b1).unwrap();

    // println!("{:?}", t7);
}
