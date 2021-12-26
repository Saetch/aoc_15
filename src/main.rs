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
mod seven;
mod seven_input;
mod eight;
mod ten;
fn main() {
    let day = 10;
    
    match day{
        1 => one::one(),
        2 => two::two(),
        3 => three::three(),
        4 => four::four(),
        5 => five::five(),
        6 => six::six(),
        7 => seven::seven(),
        8 => eight::eight(),
        10 => ten::ten(),
        _  => println!("Not found!")
    }
}
