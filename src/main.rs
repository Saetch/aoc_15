mod one;
mod two;
mod two_input;
mod three;
mod three_input;
mod four;
mod four_input;
mod five;
mod five_input;
fn main() {
    let day = 5;
    match day{
        1 => one::one(),
        2 => two::two(),
        3 => three::three(),
        4 => four::four(),
        5 => five::five(),
        _  => println!("Not found!")
    }
}
