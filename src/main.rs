use std::env;

fn main() {
    for input in env::args().skip(1) {
        match input.parse::<i64>() {
            Ok(orig) => {
                let (result, count) = get_pal(orig, 0);
                println!("{} gets palindromic after {} steps: {}",
                    orig, count, result);
            }
            Err(e) => {
                println!("error parsing input: {} {:?}", input, e);
            }
        }
    }
}

fn get_pal(i: i64, count: i64) -> (i64, i64) {
    if i.is_palindrome() {
        (i, count)
    } else {
        get_pal(i + i.flip(), count+1)
    }
}

trait FlipInt {
    fn flip(&self) -> i64;
    fn is_palindrome(&self) -> bool;
}

impl FlipInt for i64 {
    fn flip(&self) -> i64 {
        reverse(self.to_string())
            .parse::<i64>().unwrap()
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
