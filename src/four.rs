use crate::four_input::get_input;
use md5::{Md5,Digest};
pub fn four(){
    let input = get_input();

    let mut bo = true;
    let mut uint : u32 = 0;
    let mut ret ;
    // create a Md5 hasher instance

    // process input message
    loop {
        let mut hasher = Md5::new();
        hasher.update(input.clone() + &uint.to_string().as_str());
        ret =  hasher.finalize();

        let ret_string =  hex::encode(ret);
        if ret_string.starts_with("00000") && bo{
            println!("FOUND a start with 00000");
            println!("{}", ret_string);
            println!("necessary number: {}", uint);
            bo = false;
        }
        if ret_string.starts_with("000000"){
            println!("FOUND a start with 00000");
            println!("{}", ret_string);
            break;
        }
        uint +=1;

    }
    println!("necessary number: {}", uint);
}