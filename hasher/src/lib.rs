#![feature(use_extern_macros)]

#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::{
    Array,
    console,
    js_export,
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
fn reduce_sum_u8(arr: TypedArray<u8>) -> u32 {
    let vec: Vec<u8> = arr.to_vec();
    // .fold(0, |sum, x| sum + x);
    // passed as reference
    // why vec
    // why reference
    // need u32 since u16 is max 65536, and the sum is a larger number
    // console!(log, "HI");
    let sum: u32 = vec.iter().fold(0, |sum, &x| sum as u32 + x as u32);
    return sum;
}
#[js_export]
fn reduce_sum_u8_vec(vec: Vec<u8>) -> u32 {
    let sum: u32 = vec.iter().fold(0, |sum, &x| sum as u32 + x as u32);
    return sum;
}

#[js_export]
fn rgbas_to_rgbs(arr: TypedArray<u8>) -> Vec<u8> {
    let vec: Vec<u8> = arr.to_vec();
    let rgbs: Vec<u8> = vec.iter().enumerate().filter(|&(i, _)| (i == 0 || (i + 1) % 4 != 0)).map(|(_, &v)| v).collect::<Vec<_>>();
    // .filter(|&(i, _)| (i == 0 || (i + 1) % 4 != 0)).collect::<Vec<u8>>();
    return rgbs;
}


#[js_export]
fn avg(arr: TypedArray<u8>) -> u8 {
    // gotcha: ownership
    // v1
    // let length: u32 = arr.len() as u32;
    // let sum: u32 = reduce_sum_u8(arr);

    let sum: u32 = reduce_sum_u8_vec(arr.to_vec());
    let len: u32 = arr.len() as u32;

    let avg: u32 = sum.wrapping_div(len);
    return avg as u8;
}

#[js_export]
fn avg_vec(vec: Vec<u8>) -> u8 {
    let len: u32 = vec.len() as u32;
    let sum: u32 = reduce_sum_u8_vec(vec);
    let avg: u32 = sum.wrapping_div(len);
    return avg as u8;
}

#[js_export]
fn avg_rgb(arr: TypedArray<u8>) -> u8 {
    let rgbs: Vec<u8> = rgbas_to_rgbs(arr);
    return avg_vec(rgbs);
}

// const rgbasToRgbs = data => data.filter((n, i) => i === 0 || (i + 1) % 4 !== 0);



// const calculator = (rgbaData, f, normalizer = 1) => {
//   const rgbData = rgbasToRgbs(rgbaData);
//   const arr = [];
//   for (let i = 0; i < rgbData.length; i += 3) {
//     arr.push(f([rgbData[i], rgbData[i + 1], rgbData[i + 2]]));
//   }
//   return average(arr) / normalizer;
// };

// export const saturation = rgbaData => {
//   return calculator(rgbaData, pixelSaturation);
// };

// export const brightness = rgbaData => {
//   return calculator(rgbaData, pixelBrightness, MAX_BRIGHTNESS);
// };

// const pixelSaturation = arr => {
//   const max = Math.max(...arr);
//   const min = Math.min(...arr);
//   return (max - min) / max;
// };


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