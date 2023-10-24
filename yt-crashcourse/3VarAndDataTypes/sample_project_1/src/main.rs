#![allow(non_snake_case)]
use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::num;
// use std::io::stdin;

fn main() {
    println!("");println!("");println!("");
    callMehtod();
    println!("");println!("");println!("");
}

fn callMehtod() {
    println!("Hello, Rust! 🦀");

    let num: i8 = 10; // by default i32

    println!("Value of `num` is: {}", num); // {} are the escaping sequence for Rust

    let name: &str = "John";
    let sur_name: &str = "Doe";

    println!("My name is {} {}", name, sur_name);

    let mut number = 10;
    println!("My number is {}", number);
    number = 99;
    println!("My number now is {}", number);

    // Minimum and Maximum values for various data types
    println!("Minimum value for i8 is {}", i8::MIN);
    println!("Maximum value for i8 is {}", i8::MAX);
    println!("Minimum value for i16 is {}", i16::MIN);
    println!("Maximum value for i16 is {}", i16::MAX);
    println!("Minimum value for i32 is {}", i32::MIN);
    println!("Maximum value for i32 is {}", i32::MAX);
    println!("Minimum value for i64 is {}", i64::MIN);
    println!("Maximum value for i64 is {}", i64::MAX);
    println!("Minimum value for u8 is {}", u8::MIN);
    println!("Maximum value for u8 is {}", u8::MAX);
    println!("Minimum value for u16 is {}", u16::MIN);
    println!("Maximum value for u16 is {}", u16::MAX);
    println!("Minimum value for u32 is {}", u32::MIN);
    println!("Maximum value for u32 is {}", u32::MAX);
    println!("Minimum value for u64 is {}", u64::MIN);
    println!("Maximum value for u64 is {}", u64::MAX);
    
    println!("Minimum value for isize is {}", isize::MIN);
    println!("Maximum value for isize is {}", isize::MAX);
    println!("Minimum value for usize is {}", usize::MIN);
    println!("Maximum value for usize is {}", usize::MAX);

    println!("Minimum value for f32 is {}", f32::MIN);
    println!("Maximum value for f32 is {}", f32::MAX);
    println!("Minimum value for f64 is {}", f64::MIN);
    println!("Maximum value for f64 is {}", f64::MAX);

    // Boolean
    let _is_valid: bool = true;
    println!("This is a boolean {}", _is_valid);

    // Float
    let _the_float: f32 = 354.5;
    println!("This is a float {}", _the_float);

    // Character
    let character: char = 'a';

    println!("My name is {} {}, my favourite character is {}", name, sur_name, character);

    // multi-variables
    let (country, capital) = ("India", "New Delhi");
    println!("{} {}", country, capital);

    println!("20 + 4 = {}", 20 + 4);
    println!("20 - 4 = {}", 20 - 4);
    println!("20 * 4 = {}", 20 * 4);
    println!("20 / 4 = {}", 20 / 4);
    println!("Remainder of 22 / 4 = {}", 20 % 4);
    println!("20 to the power of 4 = {}", 20_i32.pow(4));
}