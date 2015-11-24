// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
extern crate clap;
use clap::{Arg, App};

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_to_vec(path_str: &str) -> Vec<String> {
    let mut file_vec: Vec<String> = Vec::new();

    let f = File::open(path_str).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        file_vec.push(line.unwrap());
    }

    file_vec
}

fn main() {
    let matches = App::new("file_cmp")
    .version("0.1.0")
    .author("Chris Palmer <pennstate5013@gmail.com>")
    .about("compare two files to see if lines in first file exist in the second file")
    .arg(Arg::with_name("INPUT")
        .help("Sets the input file to use to check")
        .required(true)
        .index(1))
    .arg(Arg::with_name("COMPARE")
        .help("Sets the input file to use to compare against")
        .required(true)
        .index(2))
    .arg(Arg::with_name("FIND")
        .short("f")
        .long("find")
        .help("string to look for to narrow search")
        .takes_value(true))
    .arg(Arg::with_name("SPLIT")
        .short("b")
        .long("split")
        .help("get string before the passed character or string")
        .takes_value(true))
    .arg(Arg::with_name("debug")
       .short("d")
       .help("prints out the line being searched")
       .takes_value(false))
    .get_matches();

    let input = matches.value_of("INPUT");
    let compare = matches.value_of("COMPARE");
    let find =   matches.value_of("FIND").unwrap_or("");
    let split = matches.value_of("SPLIT");

    let input_vec = read_to_vec(input.unwrap());
    let compare_vec = read_to_vec(compare.unwrap());

    for input_line in input_vec.clone() {
        let line = input_line + " ";
        let input_str: &str = &line;
        if matches.is_present("debug"){
            print!("{} : ",input_str);
        }

        for compare_line in compare_vec.clone() {
            let compare_str: &str = &compare_line;
            if compare_str.contains(input_str ) {
                if compare_str.contains(find) {
                    match split {
                        None => {
                            println!("{}", compare_str);
                        }
                        Some(split_str) => {
                            if split_str.contains("whitespace") {
                                let split_vec: Vec<&str> = compare_str.split_whitespace().collect();
                                println!("{}",  split_vec[0]);
                            } else {
                                let split_vec: Vec<&str> = compare_str.split(split_str).collect();
                                println!("{}",  split_vec[0]);
                            }
                        }
                    }
                    break;
                }
            }
        }
    }
}
