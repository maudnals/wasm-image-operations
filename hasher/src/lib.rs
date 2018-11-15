#![feature(use_extern_macros)]

#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::{
    Array,
    js_export
};
use stdweb::web::{
    TypedArray
};
use sha1::Sha1;

#[js_export]
fn sha1( string: &str ) -> String {
    let mut hasher = Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}

#[js_export]
fn reduce(arr: Array) -> f64 {
    // return arr.fold(0, |sum, x| sum + x);
    return 123.0;
}

#[js_export]
fn reduce_c(arr: TypedArray::<u8>) -> TypedArray::<u8> {
    // stdweb::initialize();
    // let b = TypedArray::<f64>::from(&arr);
    // let vertices = TypedArray::<u8>::from(&[
    //     255, 132][..]);
    // let k = TypedArray::<u8>::from(&arr);
    // return reduce8([1, 2, 3])
    // stdweb::console!(log, "IIIIII");
    // return arr.fold(0, |sum, x| sum + x);
    return arr;
}

// #[js_export]
// fn reduce8(arr: TypedArray<f64>) -> f64 {
//     return 456.0;
// }

// #[js_export]
// fn reduceU8(arr: stdweb::web::TypedArray) -> f64 {
//     return 123.0;
// }

// use stdweb::web::UnsafeTypedArray;
// use 
// use math::round;

// #[no_mangle]
// pub fn add(a: f64, b: f64) -> f64 {
//   return a + b
// }

// #[no_mangle]
// pub fn avg(arr: UnsafeTypedArray<f64>) -> UnsafeTypedArray<f64> {
//   // let a = vec![4, 5, 6];
//   // println!("hello there!");
//   // let sum = arr.iter().fold(0, |acc, x| acc + x);
//   return arr;
// }