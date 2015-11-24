// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

extern crate clap;
use clap::ArgMatches;
use util;

fn split_str(split: Option<&str>, compare_str: &str, debug: bool) {
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
    if debug {
        println!("");
    }
}

fn compare_files(input_str: &str, compare: Option<&str>, find: Option<&str>, split: Option<&str>, debug: bool) {
    let compare_vec = util::read_to_vec(compare.unwrap());
    for compare_line in compare_vec.clone() {
        let compare_str: &str = &compare_line.trim();
        if compare_str.contains(input_str) {
            if compare_str.contains(find.unwrap_or("")) {
                split_str(split, compare_str, debug);
                break;
            }
        }
    }
}

pub fn input_file(matches: ArgMatches) {
    let compare = matches.value_of("COMPARE");
    let find =   matches.value_of("FIND");
    let split = matches.value_of("SPLIT");
    let input = matches.value_of("INPUT");
    let debug = matches.is_present("debug");

    let input_vec = util::read_to_vec(input.unwrap());

    for input_line in input_vec.clone() {
        let input_str: &str = &input_line.trim();
        if debug {
            print!("{} : ", input_str);
        }
        compare_files(input_str, compare, find, split, debug);
    }
}
