// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.
mod util;
mod compare;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new(util::get_name())
    .version(util::get_version())
    .author(util::get_author())
    .about(util::get_about())
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
        .short("s")
        .long("split")
        .help("get string before the passed character or string")
        .takes_value(true))
    .arg(Arg::with_name("debug")
       .short("d")
       .help("prints out the line being searched")
       .takes_value(false))
    .get_matches();

    compare::input_file(matches);
}
