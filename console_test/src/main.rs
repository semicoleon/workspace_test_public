use std::error::Error;

fn main()-> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let result = simple_rust_lib::add(12, 15);
    println!("Result: {}", result);
    let add_two = simple_rust_lib::nested::add_two(result.try_into().unwrap());
    println!("Added: {}", add_two);
    let print_result = link_to_c::print_int(55);
    println!("Got back {} from C/C++", print_result);
    Ok(())
}
