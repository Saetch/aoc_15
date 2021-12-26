pub fn ten(){
    let mut current = String::from("1321131112");

    for i in 0..50{
        let mut next = String::from("");
        let mut char_c = 'a';
        let mut char_count = 1;
        for char in current.chars(){
            if char_c != char{
                if char_c != 'a'{
                    next = next + char_count.to_string().as_str() + char_c.to_string().as_str();
                }
                char_c = char;
                char_count = 1;
            }else{
                char_count+=1
            }

            
        }
        next = next + char_count.to_string().as_str() + char_c.to_string().as_str();

        current = next;
        if i == 39{
            println!("Length after 40 iterations: {}", current.len());

        }
    }

    println!("Final numbers length is: {}", current.len());
}