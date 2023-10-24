enum Compass {
    North,
    South,
    East,
    West
}

fn main() {
    let solder1: Compass = Compass::North;
    let solder2: Compass = Compass::South;
    let solder3: Compass = Compass::East;
    let solder4: Compass = Compass::West;

    match solder2 {
        Compass::North => println!("Solder2 : To Winterfell!");
        Compass::South => println!("Solder2 : To Westros!");
        Compass::East => println!("Solder2 : To King's landing!");
        Compass::West => println!("Solder2 : To Casterly Rock!q");
    }
}
