use easyinput::input;

fn main() {
    let user_input = input("What is your name? ");
    println!("Hello, {}!", user_input);
}
