use std::io;
use std::io::prelude::*;
use std::slice::Iter;

#[macro_use] extern crate itertools;
extern crate radix_trie;

use radix_trie::Trie;

// The mapping from digit to potential chars
const phoneChars: [&[u8]; 10] = [
    b"----", // 0 - no mappable chars
    b"----", // 1 - no mappable chars
    b"abc", // 2 
    b"def", // 3
    b"ghi", // 4
    b"jkl", // 5
    b"mno", // 6
    b"pqrs", // 7
    b"tuv", // 8
    b"wxyz" // 9
 ];

// Computes the arrays from `phoneChars` which correspond to the 7 inputs
fn phoneToChars<'a>(phone: Vec<u8>) -> Vec<&'a [u8]> {
    let digits : Vec<u32> = phone.iter().map(|c| (*c as char).to_digit(10).unwrap_or(0)).collect();
    vec![
        phoneChars.get(digits[0] as usize).unwrap(),
        phoneChars.get(digits[1] as usize).unwrap(),
        phoneChars.get(digits[2] as usize).unwrap(),
        phoneChars.get(digits[3] as usize).unwrap(),
        phoneChars.get(digits[4] as usize).unwrap(),
        phoneChars.get(digits[5] as usize).unwrap(),
        phoneChars.get(digits[6] as usize).unwrap()]
}

// Computes the cartesian product of the input vector
fn phoneToWords(input: Vec<u8>) -> Vec<String> {
    let mut output = Vec::new();
    let mut cs : Vec<Iter<u8>> = phoneToChars(input).iter().map(|a| a.into_iter()).collect();
    // iproduct wants an Iterator
    // but if we use an index on the Vec we induce a move, and we fail to compile
    // if we take a reference to the index we now have a &std::slice::Iter and not an Iterator
    let c6 = cs.remove(6);
    let c5 = cs.remove(5);
    let c4 = cs.remove(4);
    let c3 = cs.remove(3);
    let c2 = cs.remove(2);
    let c1 = cs.remove(1);
    let c0 = cs.remove(0);

    for (a,b,c,d,e,f,g) in iproduct!(c0, c1, c2, c3, c4, c5, c6) {
        let word = vec![*a,*b,*c,*d,*e,*f,*g];
        output.push(std::string::String::from_utf8(word).unwrap());
    }

    output
}

fn main() {
    let words: Vec<&str> = include_str!("../res/wordlist.txt").lines().collect();
    let mut trie: Trie<&str, &str> = Trie::new();

    for w in words {
        trie.insert(w, w);
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        assert_eq!(7, line.len());
        println!("Words from {}", line);
        let ws = phoneToWords(line.into_bytes());
        for w in ws {
            match trie.get(w.as_str()) {
                None => println!("NONE"),
                Some(_) => println!("{}",w)
            }
        }
    }
}
