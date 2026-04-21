#![allow(dead_code)]

pub fn rand_source() -> u32 {

    let mut buf: [u8; 4] = [0u8; 4];
    
    getrandom::fill(&mut buf).expect("failed to get random bytes");

    u32::from_be_bytes(buf)

}