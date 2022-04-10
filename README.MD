# **catch-input**

Rust library implementing a macro for retrieving user input from the console.


## **Example**
```rs
use catch_input::input;

fn main() {
    let a = input!("PromptA => ");
    let b = input!(|| { print!("PromptB => ") });
    let c = input!((String::from("PromptC => ")));

    assert!(a, String::from("Catch"));
    assert!(b, String::from("Input"));
    assert!(c, String::from("Crate"));

    println!(">> {} : {} : {}", a, b, c);
}
```

```bash
$ cargo run
...

PromptA => Catch
PromptB => Input
PromptC => Crate
Catch : Input : Crate
```