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

    println!("{:?}", hash_map.get("Hello")); // .get returns Option

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
    // On a normal Vec you use pop() to remove the last element and remove() to remove the first element
    // has push_front(), push_back(), pop_front(), pop_back()


    /* Generics */
    // Denoted by <T> where T can be anything, usually T, U, V.

    return_generic(5);
    return_generic(&hash_from_vector);

    // Sometimes you need the generic to implement a certain Trait
    // Then you can use <T: Trait>
    // Use <T: Trait1 + Trait2> if more are required

    print_debug(5);
    print_debug(&hash_from_vector);
    print_debug(&new_animal);

    // Can also use where notation (especially for more generics):

    /*
    fn print_debug<T>(generic: T)
    where
        T: Debug,
        ...
    {
        ...
    }
    */


    /* Option and Result */
    // Used to make code safer

    /* Option */
    // Used when you have a value that might exist.
    // Option<T> means Some(i32) if it's there, None otherwise.

    let new_vec = vec![1, 2];
    println!("{:?}", take_fifth(new_vec));

    let longer_vec = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", take_fifth(longer_vec));

    // You can get the value from Some() with unwrap()
    // To prevent panic by unwrapping None you can use a match with None/Some(x)
    // Or just use .is_some() on the returned value

    /* Result */
    // Ok/Err instead of Some/None
    // can unwrap Ok, Can't unwrap Err
    // fn function() -> Result<T, E> { if this Ok(...) else Err(...) }

    let new_vec = vec![0, 1];
    //let new_vec = vec![0, 1, 2, 3, 4, 5];

    // Can use if let "do something if match"
    if let Some(number) = new_vec.get(5) {
        println!("Got {}", number);
    } else {
        println!("Got nothing");
    }

    /* the ? operator */
    // can place it after a function that returns a result (in a Result/Option function)
    println!("{:?}", parse_str("10.0"));


    /* panic and assert */
    // panic!()         will stop program
    // assert!(x)       will panic if !x
    // assert_eq!(x, y) will panic if x != y
    // assert_ne!(x, y) will panic if x == y
    // these last 3 also take a custom message as argument


    /* Traits */
    // something a struct can do, added with impl
    trait Dog {
        fn bark(&self) {
            println!("Woof!");
        }
    }

    impl Dog for Animal {};

    let dog = Animal {
        age: 3,
        animal_type: AnimalType::Dog,
    };
    dog.bark();

    // Can also do this without writing the whole function
    trait Cat {
        fn meow(&self);
    }

    impl Cat for Animal {
        fn meow(&self) {
            println!("Meow!");
        }
    }

    let cat = Animal::new();
    cat.meow();


    /* The From trait */
    /*
    impl From<T> for U {
        fn from(x: T) -> Self {
            Self { ... }
        }
    }
    */


    /* Chaining methods */

    // .collect() can make collections
    let new_vec = (1..=10).collect::<Vec<i32>>(); // vec with [1, 10]

    let another_vec = new_vec
        .into_iter()
        .skip(3)
        .take(4)
        .collect::<Vec<i32>>();

    println!("{:?}", another_vec);


    /* Iterators */
    // A collection that can give you the items in the collection one at a time
    // iter() iterator of references
    // iter_mut() iterator of mutable references
    // into_iter() iterator of values (no reference so takes ownership)

    // Works by using .next() -> Option. if Some it continues, of None it stops

    // Can impletement Iterator trait.
    // Needs:
    // - type Item = T; (associated type)
    // - fn next(&mut self) -> Option<Self::Item> (implement method)


    /* Closures */
    // like quick functions without a name, sometimes called lambda
    // denoted with ||
     let doubler = |x: i32| x * 2;

     assert_eq!(doubler(5), 10);

     // Can also add code blocks:
     /*
     let closure = || {

     }
     */

     // Counter example
     let make_counter = || {
         let mut counter = 0;
         return move || {
             counter += 1;
             counter
         }
     };

     let mut x = make_counter();

     assert_eq!(x(), 1);
     assert_eq!(x(), 2);


     /* Helpful methods for closures and iterators */

     // often used with .map()
     // also with .for_each(), for example after .enumerate() -> (index, item)
     let num_vec: Vec<i32> = (0..10).collect();

     num_vec
         .iter()
         .enumerate()
         .for_each(|(index, item)| println!("{}: {}", index, item));

     // .filter(f: F) takes a function and keeps items in an iterator if true
     // .filter_map(f: F) closure must return Option, then only Some() results are kept
     // .ok() turns Result into Option, Err is discarded
     // .ok_or(err: E) turns Option into Result, maps Some() to Ok() and None to Err(err)
     // .ok_or_else(err: F) does the same but instead takes a closure for more advanced errors
     // .and_then(f: F) -> Option takes an option and applies closure to it if its input is an option
     // .and(optb: Option<U>) -> takes 2 Options and returns the second if both are Some()
     // .any() takes iterator and returns true if closure returns true for any item
     // .all() same as any() but for all items
     // .rev() reverses an iterator (reverse() for vec)
     // .find(f: F) returns Option with Some(item) if it was found
     // .positition(f: F) returns Option with Some(index) if it was found
         // .contains() returns true of item is found in vec (not iterator!)
     // .chunks(size) will give slices of given size + whatever is left with unique items
        // [1, 2, 3, 4] -> [[1, 2, 3], [4]]
     // .windows(size) will give slices of given size shifted by 1 each time
        // [1, 2, 3, 4] -> [[1, 2, 3], [2, 3, 4]]


     /* the dbg! macro and .inspect */

     // dbg!() prints info about its content
     // .inspect is similar but for iterators


     /* Types of &str */
     let string_literal: &'static str = "test"; // last for whole program = static lifetime
     let borrowed_str: &str = &String::from("test"); // regular &str
     println!("{} {}", string_literal, borrowed_str);


     /* Lifetimes */
     // how long a variable lives, only relevant for references
     // instead of static you can also create your own lifetime
     // usually 'a, 'b, 'c

     #[derive(Debug)]
     struct City<'a> {
         name: &'a str,
     }

     let city_name = String::from("Amersfoort");
     let amersfoort = City {
         name: &city_name,
     };

     println!("{:?}", amersfoort);

     // <'_> is the anonymous lifetime.


     /* Interior mutability */

     // ways to change values inside an immutable struct

     /* Cell */
     use std::cell::Cell;

     #[derive(Debug)]
     struct Car {
         company: String,
         model: String,
         for_sale: Cell<bool>,
     }

     let seat_leon = Car {
         company: "Seat".to_string(),
         model: "Leon".to_string(),
         for_sale: Cell::new(true),
     };

     assert!(seat_leon.for_sale.get());

     seat_leon.for_sale.set(false);

     assert!(!seat_leon.for_sale.get());

     /* RefCell */
     // Same as Cell but adds exrta methods
     // .borrow(), .bottom_mut() etc


     /* Mutex */
     // Stands for Mutual Exclusion
     // safe because it only lets one process change it at a time
     use std::sync::Mutex;

     let my_mutex = Mutex::new(5);

     let mut mutex_changer = my_mutex.lock().unwrap(); // lock() returns a LockResult so we can unwrap it

     *mutex_changer = 6;

     // my_mutex still locked, until the MutexGuard goes out of scope
     // can be done with a code block or std::mem::drop(mutex_changer)

     drop(mutex_changer);

     // If Mutex is locked and another lock() is called the program will wait
     // can use try_lock() which will try once and give up otherwise, returns TryLockResult
     // Will fail with unwrap if it's locked

     // Can also do shorthand
     *my_mutex.lock().unwrap() = 7;


     /* RwLock */
     // Read Write Lock
     // has read(), write()
     // many read()s are okay but only a single write()
     // When you write() while it's being read() the program will wait

     use std::sync::RwLock;

     let my_rwlock = RwLock::new(5);

     println!("{:?}", my_rwlock.read().unwrap());

     let mut write_rwlock = my_rwlock.write().unwrap();
     *write_rwlock = 6;

     drop(write_rwlock);

     println!("{:?}", my_rwlock.read().unwrap());

     // There is also try_read() and try_write()


     /* Cow */
     // Clone or Write
     // Lets you return &str if you don't need String and String if you need it
     // same with other elements such as arrays/vecs
     // Can be a return type which automatically decides if it should be borrowed or owned
     // fn mod(input) -> Cow<'static, str> { will return either &str or String
     // has methods into_owned(), into_borrowed()
}

fn parse_str(input: &str) -> Result<i32, std::num::ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    Ok(parsed_number)
}


fn take_fifth(vec: Vec<i32>) -> Option<i32> {
    if vec.len() < 5 {
        None
    } else {
        Some(vec[5])
    }
}

use std::fmt::Debug;
fn print_debug<T: Debug>(generic: T) {
    println!("Here's your T: {:#?}", generic);
}

fn return_generic<T>(generic: T) -> T {
    println!("Here's your T");
    generic
}
