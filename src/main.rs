#![allow(non_snake_case)]

mod activity01;
mod activity02;
mod activity03;
mod activity04;
mod activity05;
mod activity06;
mod activity09;

use std::fs::File;
use std::io::Read;
use std::io::Write;

fn main()
{
    let mut ifile: File = File::open("HERALDOSNEGROS.txt")
        .expect("Unable to open file");
    let mut contents: String = String::new();
    ifile.read_to_string(&mut contents)
        .expect("Unable to read data");

    activity01::run(&mut contents);
    activity02::run(&mut contents);
    activity03::run(&mut contents);
    activity04::run(&mut contents);

    let mut ofile: File = File::create("HERALDOSNEGROS_pre.txt")
        .expect("Unable to create/open file");
    let _ = ofile.write_all(contents.as_bytes());

    activity05::run(&contents);
    activity06::run(&contents);

    contents.clear();
    ifile = File::open("HERALDOSNEGROS_pre.txt")
        .expect("Unable to open file");
    ifile.read_to_string(&mut contents)
        .expect("Unable to read data");

    activity09::run(&mut contents);

    ofile = File::create("HERALDOSNEGROS_post.txt")
        .expect("Unable to create/open file");
    let _ = ofile.write_all(contents.as_bytes());
}
