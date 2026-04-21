#[allow(dead_code)]

#[derive(Debug)]
pub struct BIPSeed {
    words: Vec<u16>,
    word_count: u8,
}

impl BIPSeed {

    pub fn new(word_count: u8) -> BIPSeed {
        BIPSeed {
            words: Vec::new(),
            word_count: word_count,
        }
    }

    pub fn add_word(&mut self, val: u16) {

        self.words.push(val);

    }

    pub fn get_word(&mut self, index: u8) -> u16 {

        if (index as usize) >= self.words.len() {
            return 0xFFFF;
        }

        return self.words[index as usize];

    }

}