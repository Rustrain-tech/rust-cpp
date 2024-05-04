#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(special_module_name)]

use itertools::Itertools;
use num::traits::*;
use num::BigInt;
use proconio::{fastout, input_interactive, marker::Chars};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(feature = "local")]
mod lib;

const MOD: i64 = 998244353;
const MOD17: i64 = 1_000_000_007; //hello
const INF: i64 = 1 << 60; //こんにちは

#[fastout]
fn main() {}
