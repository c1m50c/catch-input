# **catch-input**

![Crates.io Version](https://img.shields.io/crates/v/catch-input?color=orange&style=for-the-badge)
![Crates.io License](https://img.shields.io/crates/l/catch-input?style=for-the-badge)
![Lines of Code](https://img.shields.io/tokei/lines/github/c1m50c/catch-input?style=for-the-badge)

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
>> Catch : Input : Crate
```