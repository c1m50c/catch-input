use catch_input::{input};


fn main() {
    let input = input!(|| {
        println!("Do you like Rust macros? [YES/NO]");
        print!(">> ");
    });

    match input.to_uppercase().as_str() {
        "YES" | "Y" | "1" => {
            println!("That's nice to hear! They really come in handy!");
        },

        "NO" | "N" | "0" => {
            println!("Why not?!! They're so much fun!");
        },

        _ => {
            println!("What? Huh? Come Again?");
        },
    }
}