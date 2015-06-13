extern crate num;

use std::env;
use num::BigUint;

fn main() {
    for input in env::args().skip(1) {
        match input.parse::<BigUint>() {
            Ok(orig) => {
                let (result, count) = get_pal(orig.clone(), 0);
                println!("{} gets palindromic after {} steps: {}",
                    orig, count, result);
            }
            Err(e) => {
                println!("error parsing input: {} {:?}", input, e);
            }
        }
    }
}

fn get_pal(i: BigUint, count: i64) -> (BigUint, i64) {
    if i.is_palindrome() {
        (i, count)
    } else {
        get_pal(&i + &i.flip(), count+1)
    }
}

trait FlipInt {
    fn flip(&self) -> BigUint;
    fn is_palindrome(&self) -> bool;
}

impl FlipInt for BigUint {
    fn flip(&self) -> BigUint {
        reverse(self.to_string())
            .parse::<BigUint>().unwrap()
    }

    fn is_palindrome(&self) -> bool {
        is_palindrome(&self.to_string())
    }
}

fn reverse(s : String) -> String {
    s.chars().rev().collect()
}

fn is_palindrome(s: &str) -> bool {
    s == reverse(s.to_string())
}
