extern crate emd;
#[macro_use(array)]
extern crate ndarray;

use emd::*;
use ndarray::*;

let x = array![0., 1.];
let y = array![5., 3.];
assert_eq!(distance(&x.view(), &y.view()), 3.5);