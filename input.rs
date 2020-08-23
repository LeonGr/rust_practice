use std::io;

fn main() {

   println!("Input:");
   let input: String  = get_string().unwrap();
   println!("{}", input);
}

fn get_string() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer)
}
