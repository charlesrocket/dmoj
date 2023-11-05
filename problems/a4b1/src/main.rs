#[macro_use]
extern crate dmoj;

fn main() {
    let mut s = vec![];
    let tc = scan!(u8);

    for _ in 0..tc {
        let n = scan!(u32);
        s.push(n);
    }

    s.sort_unstable();

    for i in s {
        println!("{i}");
    }
}
