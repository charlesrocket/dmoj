#[macro_use] extern crate dmoj;

fn main() {
    let atmp = 5 * scan!(u16) - 400;
    let lvl = match atmp.cmp(&100) {
         std::cmp::Ordering::Less => 1,
         std::cmp::Ordering::Equal => 0,
         std::cmp::Ordering::Greater => -1
     };

    println!("{}\n{}", &atmp, &lvl);
}
