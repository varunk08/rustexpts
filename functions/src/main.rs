extern crate rand;

use rand::Rng;

fn main() {
    println!("Functions");

    print_num(5);
    println!("Random num: {}", get_rand_num())
}

fn print_num(x : i32) {
    println!("Value: {}", x);
}

fn get_rand_num() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}
