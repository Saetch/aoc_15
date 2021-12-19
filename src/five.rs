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
            println!("{}", line);
        }
    }
    println!("found: {}", count);


}