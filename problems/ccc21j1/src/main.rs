#[macro_use]
extern crate dmoj;

fn main() {
    let atmp = 5 * scan!(u16) - 400;
    let slvl = match atmp.cmp(&100) {
        std::cmp::Ordering::Less => 1_i8,
        std::cmp::Ordering::Equal => 0_i8,
        std::cmp::Ordering::Greater => -1_i8,
    };

    println!("{}\n{}", &atmp, &slvl);
}
