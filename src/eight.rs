pub fn eight(){

    //JUST A TEST 
    let mut vec = Vec::new();

    for i in 0..=5{
        vec.push(i);
    }

    for i in 0..vec.len(){
        println!("{} -> 1  ", vec[i]);
    }
    vec.insert(0, 24);
    for i in 0..vec.len(){
        println!("{} -> ", vec[i]);
    }
}