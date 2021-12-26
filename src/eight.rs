use std::fs::read_to_string;

pub fn eight(){
    let mut code_characters = 0;
    let mut string_characters = 0;
    let inp = read_to_string("eight_input.txt").expect("Could not read file!");
    let input = inp.replace(" ", "");
    for line in input.lines(){


        let mut str = line.to_owned();
        str = str.replace("\\\\", "a").replace("\\x", "a").replace("\\\"", "a");

        string_characters+=str.len();
        code_characters+=line.len();
    }
    println!("Code: {}", code_characters);
    println!("String: {}", string_characters);
    println!("{}", code_characters-string_characters);
}