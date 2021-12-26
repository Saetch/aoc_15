use std::fs::read_to_string;

pub fn eight(){
    let mut code_characters = 0;
    let mut string_characters = 0;
    let inp = read_to_string("eight_input.txt").expect("Could not read file!");
    let input = inp.replace(" ", "");
    for line in input.lines(){


        let mut stri = line.to_owned();
        stri = stri.replace("\\\\", "a").replace("\\\"", "a").replace("h", "a");
        let oof = stri.replace("\\x", "h");

        let str = stri;
        if oof.contains("h"){
            let mut char_iter = oof.chars();
            let mut charo = char_iter.next();
            while charo.is_some(){
                let mut char = charo.unwrap();
                if char != 'h'{
                    charo = char_iter.next();
                    continue;
                }
                charo= char_iter.next();
                if charo.is_none(){
                    continue;
                }
                char = charo.unwrap();
                match char {
                    '0' ..= '9' => (),
                    'a'..= 'f' =>(),
                    'A' ..='F' =>(),
                    _ => {charo = char_iter.next(); continue}
                }

                charo= char_iter.next();
                if charo.is_none(){
                    continue;
                }
                char = charo.unwrap();
                match char {
                    '0' ..= '9' => (),
                    'a'..= 'f' =>(),
                    'A' ..='F' =>(),
                    _ => {charo = char_iter.next(); continue}
                }

                string_characters-=3;

                

            }
        }
        string_characters+=str.len() -2;
        code_characters+=line.len();
    }

    
    println!("Code characters - string characters: {}", code_characters-string_characters);
}