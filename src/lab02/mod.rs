mod pp;
mod dec;
mod divs;
mod freq;
mod strm;
mod vgcd;

use std::fs::File;
use std::io::{Read, Write};
use std::collections::BTreeMap;

fn kasiski(contents: &mut String, mut nouts: usize)
{
    println!("Original Text:\n{}", contents);
    pp::preprocess(contents);
    println!("Pre-Processed Text:\n{}", contents);
    let mut repeated:
        BTreeMap<&str, (usize, Vec<usize>)> = BTreeMap::new();
    for i in 3..9
    {
        strm::findRepeatedSubstr(contents, i, &mut repeated);
    }
    println!("Pattern Repetitions:\n{:?}", repeated);
    let mut distances: Vec<usize> = Vec::new();
    for (_, (_, iref)) in repeated.iter()
    {
        for jref in iref.iter()
        {
            if *jref != 0 { distances.push(*jref); }
        }
    }
    distances.sort_by(|l: &usize, r: &usize| r.cmp(l));
    println!("Distances: {:?}", distances);
    println!("Distances GCD: {}", vgcd::vecGCD(&distances));
    let mut divisors: [(usize, usize); 21] = divs::computeDTable(&distances);
    divisors.sort_by(|l: &(usize, usize), r: &(usize, usize)| r.1.cmp(&l.1));
    println!("Ordered Best Divisors: {:?}", divisors);
    for (div, _) in divisors
    {
        if nouts == 0 { break; }
        let L: usize = div;
        let mut subCryptos: Vec<String> = Vec::new();
        for i in 0..L
        {
            subCryptos.push(String::new());
            let mut j: usize = i;
            while j < contents.len()
            {
                subCryptos[i].push(contents.chars().nth(j).unwrap());
                j += L;
            }
        }
        println!("Sub-Crytograms:\n{:?}", subCryptos);
        let key: String = String::from("DAVINCI");
        for i in 0..L
        {
            let higher: [(char, usize); 3] = freq::getAEOFrequencies(&subCryptos[i]);
            println!("Crypto{} Higher Frequencies:\t{:?}", i, higher);
        }
        println!("From 0, +4, +11 relationship | Possible key: {}", key);
        let originalContent: String = dec::decypher(contents, &key, 27);
        contents.replace_range(.., &originalContent);
        println!("Original Content is:\n{}", originalContent);
        nouts -= 1;
    }
}

pub fn run()
{
    let mut ifile: File = File::open("kasiski.txt")
        .expect("Unable to open file");
    let mut contents: String = String::new();
    ifile.read_to_string(&mut contents)
        .expect("Unable to read data");

    kasiski(&mut contents, 1);

    let mut ofile: File = File::create("kasiski_desc.txt")
        .expect("Unable to create/open file");
    let _ = ofile.write_all(contents.as_bytes());
}
