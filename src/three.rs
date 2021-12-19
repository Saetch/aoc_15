use std::collections::HashSet;

use crate::three_input:: get_input;

pub fn three(){
    let mut x: i32  = 0;
    let mut y = x;
    let mut x2 = x;
    let mut y2 = y;
    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let input = get_input();
    let mut count = 0;
    let mut bo = true;
    for direction in input.split(""){
        count +=1;

        if bo{
            match direction {
                "^" => y+=1,
                "v" => y-=1,
                "<" => x-=1,
                ">" => x+=1,
                _ => println!("unrecognized input at {}, input was {}", count+1, direction)
            }       
             houses.insert((x, y));
             bo = false;
        }else{
            match direction {
                "^" => y2+=1,
                "v" => y2-=1,
                "<" => x2-=1,
                ">" => x2+=1,
                _ => println!("unrecognized input at {}, input was {}", count+1, direction)
            }       
             houses.insert((x2, y2));
             bo = true;
        }

    }

    println!("houses visited: {} in {} moves!", houses.len(), count);
}