mod generator;

fn main() {
    // Generate word list (integer index, 0->2047)

    println!("\nYour new wallet... DO NOT SHARE WITH ANYONE:\n");

    generator::gen_entropy_seed(24);

    generator::gen_seeded_seed(0xDEADBEEF, 12)

}