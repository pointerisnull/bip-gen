const STATE_VECTOR_LEN: usize = 624;
const STATE_VECTOR_M: usize = 397;

const UPPER_MASK: u32 = 0x80000000; 
const LOWER_MASK: u32 = 0x7fffffff; 
const TEMPERING_MASK_B: u32 =	0x9d2c5680;
const TEMPERING_MASK_C: u32 = 0xefc60000;

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

        __seed_rand(&mut rand, seed);

        return rand;
    }

    #[allow(dead_code)]
    fn next_int(&mut self) -> u32 {

        let mut y: u32 = 0; 

        static mag: [u32; 2] = [0x0, 0x9908b0df];

        if (self.index as u32) >= (STATE_VECTOR_LEN as u32) {

            if self.index >= STATE_VECTOR_LEN+1 {
                __seed_rand(self, 4357);
            }

            let mut kk: usize = 0;
            
            while kk < (STATE_VECTOR_LEN - STATE_VECTOR_M) {

                y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk+1] & LOWER_MASK);

                self.mt[kk] = self.mt[kk+STATE_VECTOR_M] ^ (y >> 1) ^ mag[(y & 0x1) as usize];

                kk += 1;
            }

            for kk in kk..(STATE_VECTOR_LEN-STATE_VECTOR_M) {

                y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk+1] & LOWER_MASK);
                
                self.mt[kk] = self.mt[kk+(STATE_VECTOR_M.wrapping_sub(STATE_VECTOR_LEN))] ^ (y >> 1) ^ mag[(y & 0x1) as usize];
            }

            y = (self.mt[STATE_VECTOR_LEN-1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);

            self.mt[STATE_VECTOR_LEN-1] = self.mt[STATE_VECTOR_M-1] ^ (y >> 1) ^ mag[(y & 0x1) as usize];

            self.index = 0;

        }
        
        // Tempering
        y = self.mt[self.index];
        self.index += 1;
        y ^= (y >> 11);
        y ^= (y << 7) & TEMPERING_MASK_B;
        y ^= (y << 15) & TEMPERING_MASK_C;
        y ^= (y >> 18);

        return y;
    }
}

fn __seed_rand(rand: &mut TwisterRNG, seed: u32) {
    rand.mt[0] = seed & 0xffffffff;

    for i in 1..STATE_VECTOR_LEN {

        rand.index = i;
        
        let prev = rand.mt[i-1];

        println!("mt[{i}] -> {prev}");

        rand.mt[i] = (prev.wrapping_mul(6069)) & 0xffffffff;

    }

}

pub fn twist(seed: u32) {

    let mut twist: TwisterRNG = TwisterRNG::from_seed(seed);

    let ts = twist.seed;

    println!("Twister seed -> {ts}");
    let int: u32 = twist.next_int();
    println!("First int -> {int}")

}