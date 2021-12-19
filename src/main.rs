mod one;
mod two;
mod two_input;
mod three;
mod three_input;

fn main() {
    let day = 3;
    match day{
        1 => one::one(),
        2 => two::two(),
        3 => three::three(),
        _  => println!("Not found!")
    }
}
