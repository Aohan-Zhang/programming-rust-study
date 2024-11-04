use std::env;
use std::str::FromStr;

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[allow(unused)]
pub fn gcd_for_console() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }
    let d = multi_gcd(&numbers);
    println!("GCD of {numbers:?} is {d}");
}

#[allow(unused)]
fn multi_gcd(numbers: &Vec<u64>) -> u64 {
    if numbers.len() == 0 {
        eprintln!("Usage: gcd2 <number>");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    d
}


#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}

#[test]
fn test_multi_gcd() {
    let d = multi_gcd(&vec![14, 24, 34]);
    assert_eq!(d, 2);
}

