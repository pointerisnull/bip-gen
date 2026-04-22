#[allow(dead_code)]

#[path = "./wordlist.rs"]
mod wordlist;

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

        if (self.word_count as usize) <= self.words.len() {

            println!("There was a problem adding word {} to seed phrase.", wordlist::to_string(val));

            return;
        }

        self.words.push(val & 2047);

    }

    fn get_word(&mut self, index: u8) -> u16 {

        if (index as usize) >= self.words.len() {
            return 0xFFFF;
        }

        return self.words[index as usize];

    }

    pub fn to_string(&mut self, index: u8) -> String {

        let word: u16 = self.get_word(index);

        wordlist::to_string(word)
        
    }

}