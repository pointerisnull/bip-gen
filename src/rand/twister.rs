const STATE_VECTOR_LEN: usize = 624;
const STATE_VECTOR_M: usize = 397;
/*
const UPPER_MASK: u32 = 0x80000000; 
const LOWER_MASK: u32 = 0x7fffffff; 
const TEMPERING_MASK_B: u32 =	0x9d2c5680;
const TEMPERING_MASK_C: u32 = 0xefc60000;
 */

struct TwisterRNG {
    index: usize,
    mt: [u32; STATE_VECTOR_LEN],
    seed: u32,
}

impl TwisterRNG {

    fn from_seed(seed: u32) -> TwisterRNG {

        let mut rand: TwisterRNG = TwisterRNG {
            index: 0,
            mt: [0; STATE_VECTOR_LEN],
            seed: seed
        };

        rand.mt[0] = seed & 0xffffffff;

        for i in 1..STATE_VECTOR_LEN {

            rand.index = i;
            
            let prev = rand.mt[i-1];

            println!("mt[{i}] -> {prev}");

            rand.mt[i] = (prev.wrapping_mul(6069)) & 0xffffffff;

        }

        return rand;
    }

    #[allow(dead_code)]
    fn next_int(&mut self) -> u32 {

        self.index += 1;

        self.seed
    }
}

pub fn twist(seed: u32) {

    let twist: TwisterRNG = TwisterRNG::from_seed(seed);

    let ts = twist.seed;

    println!("Twister seed -> {ts}");

}