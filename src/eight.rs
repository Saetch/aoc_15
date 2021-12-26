use std::fs::read_to_string;

pub fn eight(){
    let mut code_characters = 0;
    let mut string_characters = 0;
    let inp = read_to_string("eight_input.txt").expect("Could not read file!");
    let input = inp.replace(" ", "");
    for line in input.lines(){
        let mut last: Option<char> = None;
        let mut current = 'a' ;
        for char in line.chars(){
            current = char;
            if last.is_none(){
                string_characters+=1;
                last = Some(char);
                continue;
            }
            if last.unwrap()== '\\' {
                match current{
                    'x' => last = None,
                    '"' => last = None,
                    '\\' => last = None,
                    _ => {string_characters+=1; last = None},
                }
            }else{
                last = Some(char);
                string_characters+=1;
            }
        }


        code_characters+=line.len();
    }
    println!("Code: {}", code_characters);
    println!("String: {}", string_characters);
    println!("{}", code_characters-string_characters);
}