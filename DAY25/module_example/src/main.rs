use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::prelude::*;

/*  same
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs;
use std::fs::File;

use std::io::prelude::Read;
use std::io::prelude::Write;
use std::io::prelude::BufRead;
use std::io::prelude::Seek;
 */

use std::io::Result as IOResult;

mod proteins {
    pub enum AminoAcid{}

    pub mod synthesis {
        // use super::AminoAcid;  or
        use crate::proteins::AminoAcid;

        pub fn synthesize(seq: &[AminoAcid]) {}
    }

    use synthesis::synthesize;
    use AminoAcid::*;
}

mod image {
    pub struct Sampler {}
}

// use image::Pixels;     // error!
use ::image::Pixels;      // ok!     경로가 ::로 시작한다면 항상 외부 크레이트를 참조
use self::image::Sampler; // ok!

fn main() {
    println!("Hello, world!");
}
