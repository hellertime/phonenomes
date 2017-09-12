use std::io;
use std::io::prelude::*;
use std::slice::Iter;

#[macro_use] extern crate itertools;

extern crate aho_corasick;
use aho_corasick::{Automaton, AcAutomaton, MatchesOverlapping, Match};

// The mapping from digit to potential chars
const phone_chars: [&[u8]; 10] = [
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

// Computes the arrays from `phone_chars` which correspond to the 7 inputs
fn phone_to_chars<'a>(phone: Vec<u8>) -> Vec<&'a [u8]> {
    let digits : Vec<u32> = phone.iter().map(|c| (*c as char).to_digit(10).unwrap_or(0)).collect();
    vec![
        phone_chars.get(digits[0] as usize).unwrap(),
        phone_chars.get(digits[1] as usize).unwrap(),
        phone_chars.get(digits[2] as usize).unwrap(),
        phone_chars.get(digits[3] as usize).unwrap(),
        phone_chars.get(digits[4] as usize).unwrap(),
        phone_chars.get(digits[5] as usize).unwrap(),
        phone_chars.get(digits[6] as usize).unwrap()]
}

// Computes the cartesian product of the input vector
fn phone_to_words(input: Vec<u8>) -> Vec<String> {
    let mut output = Vec::new();
    let mut cs : Vec<Iter<u8>> = phone_to_chars(input).iter().map(|a| a.into_iter()).collect();
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

fn is_covered<'a>(ms: MatchesOverlapping<&'a str, AcAutomaton<&'a str>>, spanning: usize) -> bool {
    let mut low = 0;
    let mut hi = 0;
    let mut span = 0;

    for m in ms {
        //println!(">low: {}; hi: {}; span: {}; match {:?}", low, hi, span, m);
        if m.start > hi {
            low = m.start;
            hi = m.end;
        }
        else if m.start == hi {
            span = span + (hi - low);
            low = m.start;
            hi = m.end;
        } else if m.start <= low {
            low = m.start;
            if m.end > hi {
                hi = m.end;
            }
        }
        //println!("<low: {}; hi: {}; span: {}; match {:?}", low, hi, span, m);
    }

    span = span + (hi - low);
    //println!("=low: {}; hi: {}; span: {}", low, hi, span);

    span == spanning
}

fn main() {
    println!("Loading wordlist...");
    let words: Vec<&str> = include_str!("../res/wordlist2.txt").lines().collect();

    println!("Compiline automaton...");
    let aut = aho_corasick::AcAutomaton::new(words);

    let stdin = io::stdin();
    println!("Ready...");
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        assert_eq!(7, line.len());
        let ws = phone_to_words(line.clone().into_bytes());
        for w in ws {
            if is_covered(aut.find_overlapping(&w), 7) {
                println!("{}: {}", line, w);
            }
        }
    }
}
