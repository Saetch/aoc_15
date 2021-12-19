mod one;
mod two;
mod two_input;

fn main() {
    let day = 2;
    match day{
        1 => one::one(),
        2 => two::two(),
        _  => println!("Not found!")
    }
}
