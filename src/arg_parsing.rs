use clap::Parser;

use crate::Int;

#[derive(Parser, Debug)]
pub struct Args {
    i1: Int,
    i2: Int,
    i3: Int,
    i4: Int,
}

impl From<Args> for [Int; 4] {
    fn from(args: Args) -> Self {
        [args.i1, args.i2, args.i3, args.i4]
    }
}
