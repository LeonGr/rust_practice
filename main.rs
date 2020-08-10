fn main() {
    let test = {};
    println!("{}", 10); // Display print
    println!("{:?}", test); // Debug print (also {:#?})

    println!("{}", std::i8::MIN + std::i8::MAX);

    let mut number = 10;
    println!("{}", number);
    number = 5;
    println!("{}", number);
    let number = 8; // Duplicate declaration (shadowing)

    println!("{}", number);

    let variable = 10; // pointer reference
    let reference = &variable;

    println!("{}", reference);

    println!("multi
line
statement");

    println!(r#"this is a "raw" string "#);
    //println!(r##"this is a "#raw" string "##);
    //println!(r###"this is a "##raw" string "###);
    //println!(r####"this is a "###raw" string "####);
    //println!(r#####"this is a "####raw" string "#####);

    let var = 12;
    let r#ref = &var; // special word ref used as variable name
    println!("{}", r#ref);

    println!("{:?}", b"byte string");
    println!("{:?}", br#"raw byte string"#);


    println!("{:p}", reference); // print reference pointer address

    println!("{:b}, {:x}, {:o}", var, var, var); // Different bases

    println!("different orders: {1} {0} {2}", 1, 0 ,2);

    println!("{name}", name="test");

    // String vs &str
    let simple_string: &str = "Hello there"; // dynamically sized, fast
    let complicated_string: String = "General Kenobi".to_string(); // more functions, slower, pointer with data on heap
    let different_declaration = String::from("aaaa");
    let another_declaration: String = format!("{}", "text");
    let last_declaration: String = "make me a String".into();

    println!("{} {}", simple_string, complicated_string);

    println!("{} {} {}", different_declaration, another_declaration, last_declaration);


    // std::mem::size_of::<Type>() gives you the size in bytes of a type
    //println!("A String is always {:?} bytes. It is Sized.", std::mem::size_of::<String>());
    //println!("And an i8 is always {:?} bytes. It is Sized.", std::mem::size_of::<i8>());
    //println!("And an f64 is always {:?} bytes. It is Sized.", std::mem::size_of::<f64>());
    // std::mem::size_of_val() gives you the size in bytes of a variable
    //println!("But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.", std::mem::size_of_val("서태지"));
    //println!("And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.", std::mem::size_of_val("Adrian Fahrenheit Țepeș"));

    const CONSTANT: &str = "Constant";
    static STATIC: &str = "Static"; // With fixed memory location

    println!("{} {}", CONSTANT, STATIC);


    // you can't have a reference to a variable that's not in memory:
    /*
    let error = {
        let source = "a";
        let reference = &source;
        reference
    };

    &source >> borrowed value does not live long enough
    */

    let mut mutable_num = 8;
    //let nonmutable_ref = &mutable_num; // Not Allowed
    let mutable_ref = &mut mutable_num; // mutable reference
    *mutable_ref = 10; // & is referencing and * is dereferencing

    //println!("{}", nonmutable_ref);

    // Rules:
    // 1 - can have as many immutable references as you want
    // 2 - if you have a mutable reference that's it, you can't also have an immutable one

    // Shadowing & referencing:
    let country = String::from("Austria");
    let country_ref = &country;
    let country = 8;
    println!("{}, {}", country_ref, country); // "Austria, 8" because the original country still  exists


    // Giving references to functions: a variable can only have one owner
    // This is because variables are in charge of freeing their own resources
    // when doing variable assignments or passing function arguments by value,
    // the ownership of the resources is transferred, a "move" in rust
    // After a move the previous owner can no longer be used to avoid dangling pointers
    let other_country: String = String::from("Netherlands");
    print_country(other_country);
    //print_country(other_country);  // Not allowed
    //print_country(other_country.clone());  // Allowed, see Copy Types

    let test = "Hello";
    print_test(test);
    print_test(test); // Allowed because not a reference


    let fixed_country: String = String::from("Austria");
    fixed_print_country(&fixed_country);
    fixed_print_country(&fixed_country); // Allowed because borrowed
    // borrow cheacker guarantees that while references exist, an object cannot be destroyed

    let mut string = String::from("Hello");
    add_world(&mut string);
    println!("{}", string);

    // fn name(x: String)  takes a string and owns it
    // fn name(x: &String)  takes a string and borrows it
    // fn name(x: &mut String)  takes a string and borrows it and can change it

    /* Copy Types */
    // simple types on the stack, compiler knows size

    // Rust documentation: Trait Implementations
    // If 'Copy' implemented the value is copied when sent to a function

    let number = 8; // copy implemented
    print_number(number);
    print_number(number);

    let country = String::from("Belgium");
    print_country(country.clone()); // Need clone because String doesn't implement Copy
    print_country(country);

    // Clone can use a lot of memory, so a reference is faster

    // Variables without values (uninitialized)
    let my_var;

    {
        my_var = 50;
    }

    println!("{}", my_var);

    /* Collection types */

    // Arrays (fixed size, single type) very fast
    let array = [1, 2, 3, 4];
    //let () = array; // to test for type let compiler fail

    let aaa = ["a"; 3]; // declaring array with same value
    println!("{:?}", aaa);

    // let mut buffer = [0; 640]; // buffer example

    // Array slicing:
    let first_two = &array[0..2]; // Exclusive
    let also_first_two = &array[0..=1]; // Inclusive
    let another_first_two = &array[..2];

    println!("{:?}", first_two);
    println!("{:?}", also_first_two);
    println!("{:?}", another_first_two);

    // &array[..] is everything

    /* Vectors */
}

fn print_number(mut number: i32) {
    number += 1;
    println!("{}", number);
}

fn fixed_print_country(country_name: &String) {
    println!("{}", country_name);
}

fn print_country(country_name: String) {
    println!("{}", country_name);
}

fn print_test(test: &str) {
    println!("{}", test);
}

fn add_world(text: &mut String) {
    text.push_str(" world");
}
