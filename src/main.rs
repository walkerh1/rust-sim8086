use std::{env, fs};

const WORD_SIZE: usize = 2; // 16 bit words (expressed here in bytes)

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");
    let f = fs::read(file_path).expect("Could not open file");
    for word in f.chunks_exact(WORD_SIZE) {
        let instruction = read_u16(word);
        parse_instruction(instruction);
    }
}

fn parse_instruction(instruction: u16) {
    let op_code = instruction >> 10;
    match op_code {
        0b100010 => parse_move(instruction), // MOV
        _ => panic!("unknown op code"),
    }
}

fn parse_move(instruction: u16) {
    let d = (instruction & (1 << 9)) >> 9;
    let w = (instruction & (1 << 8)) >> 8;
    let mode = (instruction & (0b11 << 6)) >> 6;
    let reg = (instruction & (0b111 << 3)) >> 3;
    let rm = instruction & 0b111;
    match mode {
        0b11 => {
            if d == 1 {
                println!(
                    "mov {}, {}",
                    decode_reg_field(w, reg),
                    decode_reg_field(w, rm)
                );
            } else {
                println!(
                    "mov {}, {}",
                    decode_reg_field(w, rm),
                    decode_reg_field(w, reg)
                );
            }
        }
        0b01 => todo!(),
        0b10 => todo!(),
        0b00 => todo!(),
        _ => panic!("invalid mode"),
    }
}

fn decode_reg_field(w: u16, reg: u16) -> &'static str {
    match (w, reg) {
        (0b0, 0b000) => "al",
        (0b0, 0b001) => "cl",
        (0b0, 0b010) => "dl",
        (0b0, 0b011) => "bl",
        (0b0, 0b100) => "ah",
        (0b0, 0b101) => "bh",
        (0b0, 0b110) => "ch",
        (0b0, 0b111) => "dh",
        (0b1, 0b000) => "ax",
        (0b1, 0b001) => "cx",
        (0b1, 0b010) => "dx",
        (0b1, 0b011) => "bx",
        (0b1, 0b100) => "sp",
        (0b1, 0b101) => "bp",
        (0b1, 0b110) => "si",
        (0b1, 0b111) => "di",
        _ => {
            panic!("unknown register codes");
        }
    }
}

fn read_u16(word: &[u8]) -> u16 {
    let high = (word[0] as u16) << 8;
    let low = word[1] as u16;
    return high | low;
}
