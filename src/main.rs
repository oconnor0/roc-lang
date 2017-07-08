extern crate roc_lang;

use roc_lang::ast::*;
use roc_lang::syntax::*;

fn main() {
  // println!("Hello, world!");
  println!("{:?}", parse_Field("and = ...").unwrap());
}
