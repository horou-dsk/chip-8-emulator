use std::ops::Shr;

fn main() {
    // let (a, b) = (100u8, 200u8);
    // println!("{}", (a as usize + b as usize) as u8);
    // println!("{}", rand::random::<u8>());
    // let arr = [1, 2, 3, 4];
    // println!("{:?}", arr);
    // println!("{}", 2047 / 64);
    let mut diff = 200i32;
    diff += if 1 == 3 {
        4
    } else {
        2
    };
    println!("{}", diff);
}