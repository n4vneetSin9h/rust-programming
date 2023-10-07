// cannot decalre variables outside a function scope as a global variable
use std::{i8, u8};

const THE_CONST_AGE: u8 = 24;

fn main() {

    let new_age: i8 = 32;
    let new_age = 34; // shadows the previous declration. `let` variables are runtime only

    println!("Age is {}", THE_CONST_AGE);
    println!("New age is {}", new_age);
    
}
