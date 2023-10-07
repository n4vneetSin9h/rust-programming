

fn main() {
    let _x = 25;
    
    // if elseif statements
    if _x < 20 {
        println!("less than 20 yo");
    } else if _x < 30 {
        println!("less than 30 yo");
    } else {
        println!("above 30 yo")
    }

    // instead of switch
    match _x {
        0..=20 => println!("less than 20 yo"),
        21..=30 => println!("less than 30 yo"),
        31|32|33|34|35|36|37|38|39 => println!("less than 40 yo"),
        _ => println!("above 30 yo"),
    }



}
