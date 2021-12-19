mod one;
mod two;
mod two_input;
mod three;
mod three_input;
mod four;
mod four_input;
mod five;
mod five_input;
mod six;
mod six_input;
fn main() {
    let day = 6;
    
    match day{
        1 => one::one(),
        2 => two::two(),
        3 => three::three(),
        4 => four::four(),
        5 => five::five(),
        6 => six::six(),
        _  => println!("Not found!")
    }
}
