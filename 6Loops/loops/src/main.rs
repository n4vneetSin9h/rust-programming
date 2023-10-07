

fn main() {
    let mut n = 0;
    // simple loops
    loop {
        n += 1;

        // break
        if n == 100 {
            break;
        }

        // continue
        if n % 5 == 0 {
            continue;
        }

        println!("The value of n is {}", n);
    }

    let x = 1..6;

    // for loops
    for i in x {
        println!("index is {}", i);
    }

    let fruits = vec!["apple", "orange", "banana", "dragon-fruit"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("{} is at {} position", fruit, index);
    }

    let mut score: i32 = 1;

    // while score <= 20 {
    //     println!("The number is {}", score);
    //     score += 1;
    // }

    while score <= 40 {
        if score % 2 == 0 {
            println!("Number {} is even.", score);
        } else {
            println!("Number {} is odd.", score);
        }
        score += 1;
    }

}
