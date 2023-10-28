#[macro_use]
extern crate dmoj;

fn prime_check(n: u16) -> bool {
    if n <= 1 {
        return false;
    }

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let tc = scan!(u16);

    for _ in 0..tc {
        let (a, b) = scan!(u16, u16);
        let mut ct = 0;

        for n in a..b {
            if prime_check(n) {
                ct += 1;
            }
        }

        println!("{ct}");
    }
}
