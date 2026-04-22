#[path = "./core/mod.rs"]
mod core;

#[path = "./rand/mod.rs"]
mod rand;

use core::seed;
use rand::twister;
use rand::entropy;

pub fn gen_entropy_seed(words: u8) {
    let mut wallet_seed: seed::BIPSeed = seed::BIPSeed::new(words);

    for i in 0..words {

        let src: u32 = entropy::rand_source();
        
        wallet_seed.add_word(src as u16);

        print!("{} ", wallet_seed.to_string(i as u8));

    }

    println!("\n");

}

pub fn gen_seeded_seed(seed: u32, words: u8) {

    let mut rng = twister::TwisterRNG::from_seed(seed);

    let mut wallet_seed: seed::BIPSeed = seed::BIPSeed::new(words);
    
    for i in 0..words {

        let src: u32 = rng.next_int();
        
        wallet_seed.add_word(src as u16);

        print!("{} ", wallet_seed.to_string(i as u8));

    }

    println!("\n");



}