fn main() {
    /* Structs */

    // Create your own type
    // 3 options:

    // Unit struct (no body)
    //struct Struct;

    // Tuple struct (only types)
    //struct IntTriple(u8, u8, u8);

    // Named Struct (variable names with types)
    struct ColourRGB {
        red: u8,
        green: u8,
        blue: u8,
    }


    let magenta = ColourRGB {
        red: 255,
        green: 0,
        blue: 255,
    };

    println!("Magenta RGB: {} {} {}", magenta.red, magenta.green, magenta.blue);

    // Can also create with variables of same names:
    let (red, green, blue) = (100, 100, 100);
    let grey = ColourRGB {
        red,
        green,
        blue,
    };

    println!("Grey RGB: {} {} {}", grey.red, grey.green, grey.blue);

    /* Enums */

    // For holding choices
    enum Directions {
        North,
        East,
        South,
        West,
    }

    let north_pole_location = Directions::North;
    let _east_pole_location = Directions::East;
    let _south_pole_location = Directions::South;
    let _west_pole_location = Directions::West;

    match north_pole_location {
        Directions::North => println!("It's in the north!"),
        Directions::East => println!("It's in the east!"),
        Directions::South => println!("It's in the south!"),
        Directions::West => println!("It's in the west!"),
    }

    // Can also turn it into an integer (only if it doesn't have other data like North(String))
    println!("{}", Directions::North as u32);

    // You can give it different integers than default too: enum Dir { North = 100 };

    // Can use enums to create a thing with multiple possible types:
    enum Number {
        U32(u32),
        I32(i32),
    }

    impl Number {
        fn new(number: i32) -> Number {
            match number.is_positive() {
                true => Number::U32(number as u32),
                false => Number::I32(number),
            }
        }
    }

    let number = Number::new(100);

    match number {
        Number::I32(number) => println!("i32: {}", number),
        Number::U32(number) => println!("u32: {}", number),
    }

    /* Destructuring */
    // Getting values from a struct or enum by using 'let' backwards

    let ColourRGB { red: r, green: g, blue: b } = magenta;

    println!("{} {} {}", r, g, b);

    /* Loops */

    // Infinite loop
    //loop {

    //}

    // You can give nested loops a name:
    let mut counter = 0;
    'loop1: loop {
        loop {
            counter += 1;
            if counter == 10 {
                break 'loop1;
            }
        }
    }

    // there are also while loops

    // Ranges with .. or ..=
    for i in 0..10 { // can use _ if you don't need counter variable
        print!("{} ", i);
    }

    println!();

    // There is also for X in
    for i in vec![0, 1, 2, 3] {
        print!("{:?} ", i);
    }
}
