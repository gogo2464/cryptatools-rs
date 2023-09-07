use r2pipe::R2Pipe;
use r2pipe::open_pipe;
use cryptatools_core::utils::alphabets::Alphabet;
use cryptatools_core::cryptanalysis::custom::general_cryptanalysis_methods::frequency_analysis::distribution_algorithms::statistical::Statistical;
use std::u8;

fn read_plain_text(cipher_text: String) -> Vec<u8> {
  let mut bytes = Vec::new();
  for o in (0..cipher_text.len()).step_by(2) {
	let left = cipher_text.chars().nth(o).unwrap();
	let right = cipher_text.chars().nth(o+1).unwrap();
	let mut opcode = String::from(left);
	opcode.push(right);
	bytes.push(u8::from_str_radix(&opcode, 16).unwrap());
  }	
  
  bytes
}

fn main() {
  let mut r2p = open_pipe!(Some("bin")).unwrap();
  let mut cipher_text = String::from(r2p.cmd("p8 0x1e @ 0x2c ;").unwrap());
  cipher_text.remove(cipher_text.len()-1);
  cipher_text.remove(cipher_text.len()-1);
  
  println!("cipher text: {:?}", cipher_text);
  
  let unknow_opcode_alphabet = Alphabet::new_empty().unknow_opcodes();
  
  let bytes = read_plain_text(cipher_text);
  
  let stat = Statistical::new(unknow_opcode_alphabet.clone());
  let stat_percentage = stat.guess_statistical_distribution(bytes);
  
  for character in stat_percentage {
	  for opcode in character.0 {
		  println!("opcode {:x}, statistic: {:?}", opcode, character.1);
	  }
  }
  
  r2p.close();
}