#![feature(use_extern_macros)]

#[macro_use]
extern crate stdweb;

use stdweb::{
    Array,
    js_export,
};
use stdweb::web::{
    TypedArray
};


#[js_export]
fn reduce_sum_u8(arr: TypedArray<u8>) -> u32 {
    // need vec to iter() (there is no iter() on TypedArray)
    let vec: Vec<u8> = arr.to_vec();
    // need u32 since sum > max(u16) = 65536
    // so x needs to be casted
    // but only primitive types can be cast into each other, so x needs to be dereferenced
    let sum: u32 = vec.iter().fold(0, |sum, &x| sum as u32 + x as u32);
    // alternative if no need to cast: vec.iter().sum()
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
fn avg_vec_f64(vec: Vec<u8>) -> f64 {
    let len: u32 = vec.len() as u32;
    let sum: u32 = reduce_sum_u8_vec(vec);
    let avg: f64 = sum as f64 / len as f64;
    return avg as f64;
}

#[js_export]
fn avg_rgb_f64(arr: TypedArray<u8>) -> f64 {
    let rgbs: Vec<u8> = rgbas_to_rgbs(arr);
    return avg_vec_f64(rgbs);
}

// #[js_export]
// fn avg(arr: TypedArray<u8>) -> u8 {
//     // gotcha: ownership
//     // v1
//     // let length: u32 = arr.len() as u32;
//     // let sum: u32 = reduce_sum_u8(arr);

//     let sum: u32 = reduce_sum_u8_vec(arr.to_vec());
//     let len: u32 = arr.len() as u32;

//     let avg: u32 = sum.wrapping_div(len);
//     return avg as u8;
// }

// #[js_export]
// fn avg_vec_u8(vec: Vec<u8>) -> u8 {
//     let len: u32 = vec.len() as u32;
//     let sum: u32 = reduce_sum_u8_vec(vec);
//     let avg: u32 = sum.wrapping_div(len);
//     return avg as u8;
// }

// #[js_export]
// fn avg_rgb_u8(arr: TypedArray<u8>) -> u8 {
//     let rgbs: Vec<u8> = rgbas_to_rgbs(arr);
//     return avg_vec_u8(rgbs);
// }
