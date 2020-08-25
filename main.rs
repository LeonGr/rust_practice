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

    println!();


    /* Implementing structs and enums */
    // Use impl to create method for enum or struct
    // Regular methods: with &self or &mut self, use .
    // Associated (or static) methods, no self, use :: (usually used to create new variables)

    // #[] are rust attributes, e.g. #[derive(Debug)] to add {:?} printing

    #[derive(Debug)]
    struct Animal {
        age: u8,
        animal_type: AnimalType,
    }

    impl Animal {
        fn new() -> Self {
            Self {
                age: 10,
                animal_type: AnimalType::Cat,
            }
        }

        fn change_to_dog(&mut self) {
            println!("This animal is now a dog!");
            self.animal_type = AnimalType::Dog;
        }

        fn change_to_cat(&mut self) {
            println!("This animal is now a cat");
            self.animal_type = AnimalType::Cat;
        }

        fn check_type(&self) {
            match self.animal_type {
                AnimalType::Dog => println!("This is a dog"),
                AnimalType::Cat => println!("This is a cat"),
            }
        }
    }


    #[derive(Debug)]
    enum AnimalType {
        Cat,
        Dog,
    }

    let mut new_animal = Animal::new();

    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.check_type();


    /** Other Collections **/

    /* HashMap (and BTreeMap) */
    // Like a dictionary

    use std::collections::HashMap;

    let mut hash_map = HashMap::new();

    hash_map.insert("Hello", "World");
    hash_map.insert("int", "test");

    println!("{:?}", hash_map.get("Hello"));

    // Create HashMap from vector of tuples
    let tuple_vector = vec![("Test", "1"), ("test", "2")];

    let mut hash_from_vector: HashMap<&str, &str> = tuple_vector.into_iter().collect();

    println!("{:?}", hash_from_vector["test"]);

    // Can use .get("key") or ["key"], latter crashes if key is not in map

    // BTreeMap is sorted verson

    // with entry() you can check if something is in the map
    println!("{:?}", hash_from_vector.entry("test"));

    // You can use entry() to do things if there is no entry
    hash_from_vector.entry("10").or_insert("a");

    println!("{:?}", hash_from_vector["10"]);


    /* HashSet and BTreeSet */
    // Same as Map but only keys, value is ()
    // Can use contains() or get()

    /* BinaryHeap */
    // Collection with largest item in the front (max heap)
    // Hash push() to add and pop() to get largest item
    // peek() get first item or None

    use std::collections::BinaryHeap;

    let heap: BinaryHeap<u8> = BinaryHeap::new();
    println!("{:?}", heap.peek()); // -> None

    /* VecDeque */
    // A Vec that's good with pop() to remove the last element and remove() to remove the first element
}
