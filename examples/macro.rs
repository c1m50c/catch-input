use catch_input::input;


fn main() {
    let a = input!("This is A => ");
    let b = input!(|| { print!("This is B => "); });
    let c = input!((String::from("This is C => ")));
    println!("{} : {} : {}", a, b, c);
}