// Copyright (c) 2015 Chris Palmer <pennstate5013@gmail.com>
// Use of this source code is governed by the GPLv3 license that can be
// found in the LICENSE file.

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

pub fn read_to_vec(path_str: &str) -> Vec<String> {
    let mut file_vec: Vec<String> = Vec::new();

    let f = File::open(path_str).unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        file_vec.push(line.unwrap());
    }

    file_vec
}
