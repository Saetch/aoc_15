use crate::two_input::{ get_input};

pub fn two(){
    let input : String = get_input();

    let lines = input.lines();
    let mut sum: u32 = 0;
    let mut counter  = 0;
    let mut numbers;
    let mut lp;
    let mut wp;
    let mut hp;
    let mut ribbon: u32 = 0;
    for line in lines{
        counter +=1;
        numbers = line.split("x");
        println!("Lines is: {}",line);
        //end of input
        let mut f_str = numbers.next();
        if f_str.is_none(){
            break;
        }
        lp = f_str.unwrap().replace(" ", "").parse::<u16>();
        f_str = numbers.next();
        if f_str.is_none(){
            break;
        }
        wp = f_str.unwrap().replace(" ", "").parse::<u16>();
        f_str = numbers.next();
        if f_str.is_none(){
            break;
        }
        hp = f_str.unwrap().replace(" ", "").parse::<u16>();

        let l = lp.unwrap();
        let w = wp.unwrap();
        let h = hp.unwrap();
        let to_add = 2*(l*w+l*h + w*h);
        sum += to_add as u32;
        if l >= w && l >= h{
            sum += (w*h) as u32;
            ribbon += ((2*w + 2*h) + w*l*h) as u32;
        }
        else if w>= l && w >=h{
            sum += (l*h) as u32;
            ribbon += ((2*l + 2*h) + w*l*h) as u32;

        }else{
            sum+= (l*w) as u32;
            ribbon += ((2*w + 2*l) + w*l*h) as u32;

        }
    }
    println!("Result: {}", sum);
    println!("Ribbon needed: {}", ribbon);
    println!("lines counted: {}",counter);

}