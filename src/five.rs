use crate::five_input::get_input;
pub fn five(){
    let mut count = 0;
    let input = get_input();
    for line in input.lines(){
        let mut voriger_buchstabe = "";

        let mut hits :u8 = 0b00000000;
        for ch in line.split(""){
            let buchstabe = ch;
            if buchstabe.replace(" ", "").len()>0 && buchstabe.eq(voriger_buchstabe){
                hits = hits | 0b00000001;
            }
            if buchstabe.eq("a") || buchstabe.eq("e") || buchstabe.eq("i") || buchstabe.eq("o") || buchstabe.eq("u"){
                hits += 0b00001000;
            }

            voriger_buchstabe = buchstabe;
        }
        if hits >= 0b11000{
            hits = hits | 0b00000010;
        }
        if !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy"){
            hits = hits | 0b00000100;
        }

        if hits & 0b000000111 == 0b00000111{
            count +=1;
        }
    }
    println!("found: {}", count);


    let mut count = 0;
    for line_d in input.lines(){
        let line = line_d.replace(" ", "");
        let mut hits :u8 = 0b00000000;
        let mut cur = line.chars();
        let mut plus_two = line.chars();
        plus_two.next();
        plus_two.next();

        if line.len() == 0{
            continue;
        }
        for i in 0..line.len()-2{
            if i < line.len()-3{
                let to_find = line.split_at(i).1.split_at(2).0;
                let rest = line.split_at(i).1.split_at(2).1;
    
                if rest.contains(to_find){
                    hits = hits | 0b1;
                }
            }

            if cur.next().unwrap() == plus_two.next().unwrap(){
                hits = hits | 0b10;
            }
            
        }
        if hits & 0b11 == 0b11 {
            count +=1;
        }

    }
    println!("found with second ruleset: {}", count);

}