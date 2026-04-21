mod rand;
mod core;
use rand::twister;
use rand::entropy;
use core::seed;

fn main() {
    // Seed rng with /udev/rand or with user input (NOT SECURE!)
    twister::twist(32);
    // Generate word list (integer index, 0->2047)
    let mut bipseed: seed::BIPSeed = seed::BIPSeed::new(12);

    bipseed.add_word(23);
    bipseed.add_word(0x152);
    bipseed.add_word(0xFF);

    println!("{:?}", bipseed);

    let src = entropy::rand_source();

    println!("Entropy -> {}", src);

}