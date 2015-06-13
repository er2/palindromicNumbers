use std::env;

fn main() {
    for input in env::args().skip(1) {
        match input.parse::<i32>() {
            Ok(orig) => {
                let (result, count) = get_pal(orig);
                println!("{} gets palindromic after {} steps: {}",
                    orig, count, result);
            }
            Err(e) => {
                println!("error parsing intput: {} {:?}", input, e);
            }
        }
    }
}

fn get_pal(i: i32) -> (i32, i32) {
    get_pal_helper(i, 0)
}

fn get_pal_helper(i: i32, count: i32) -> (i32, i32) {
    if i.is_palindrome() {
        (i, count)
    } else {
        get_pal_helper(i + i.flip(), count+1)
    }
}

trait FlipInt {
    fn flip(&self) -> i32;
    fn is_palindrome(&self) -> bool;
}

impl FlipInt for i32 {
    fn flip(&self) -> i32 {
        reverse(self.to_string())
            .parse::<i32>().unwrap()
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
