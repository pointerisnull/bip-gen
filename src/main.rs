mod rand;
use rand::twister;

fn main() {
    // Seed rng with /udev/rand or with user input (NOT SECURE!)
    twister::twist(32);
    // Generate word list (integer index, 0->2047)


}