use crate::six_input::get_input;

pub fn six(){
    
    let mut arr : [u8; 1000000] = [0; 1000000];

    let input = get_input();
    let mut first_number;
    let mut second_number;
    let mut _skip = 0;
    let mut dec: u32=0;
    let mut target_x;
    let mut target_y;
    for mut line in input.lines(){
        if line.len() < 5{
            continue;
        }
        while line.starts_with(" ") {
            line = line.split_at(1).1;
        }
        if line.starts_with("toggle"){
            _skip = 1;
            dec = 0;
        }else{
            if line.starts_with("turn on"){
                dec = 1;
            }
            else {
                dec = 2;
            }
            _skip = 2;
        }
        first_number = line.split(" ").into_iter().skip(_skip).next().unwrap().split(",").into_iter().next().unwrap().parse::<u16>().unwrap();
        second_number = line.split(",").skip(1).next().unwrap().split(" ").next().unwrap().parse::<u16>().unwrap();
        target_x = line.split(" ").into_iter().skip(_skip+2).next().unwrap().split(",").into_iter().next().unwrap().parse::<u16>().unwrap();
        target_y = line.split(",").skip(2).next().unwrap().split(" ").next().unwrap().parse::<u16>().unwrap();

        match dec {
            0 => toggle_through( &mut arr, &first_number, &second_number, &target_x,&target_y),
            1 => turn_on_through( &mut arr, &first_number, &second_number, &target_x,&target_y),
            2 => turn_off_through( &mut arr, &first_number, &second_number, &target_x,&target_y),
            _ => println!("Could not decide what to do!")
        }

    }
    let mut dec : u32 = 0;

    
   
   for x in 0..1000{
       for y in 0..1000{
           dec += get(&arr, x, y) as u32;
       }
   }
    println!("{} lights are burning!", dec);


}

fn turn_on_through(arr: &mut [u8; 1000000], x_start : &u16, y_start: &u16, x_end :&u16, y_end:&u16){

    for y in *y_start as usize..=*y_end as usize{
        for x in *x_start as usize..=*x_end as usize{
            turn_on(arr, x, y);
        }
    }

}

fn turn_off_through(arr: &mut [u8; 1000000], x_start : &u16, y_start: &u16, x_end :&u16, y_end:&u16){

    for y in *y_start as usize..=*y_end as usize{
        for x in *x_start as usize..=*x_end as usize{
            turn_off(arr, x, y);
        }
    }

}

fn toggle_through(arr: &mut [u8; 1000000], x_start : &u16, y_start: &u16, x_end :&u16, y_end:&u16){

    for y in *y_start as usize..=*y_end as usize{
        for x in *x_start as usize..=*x_end as usize{
            toggle(arr, x, y);
        }
    }

}



fn turn_on( arr: &mut [u8; 1000000], x : usize, y :usize) {
     arr[(x+ y*1000)] += 1;
}
fn turn_off( arr: &mut [u8; 1000000],  x : usize, y :usize) {
    if  arr[(x+ y*1000)] > 0{
            arr[(x+ y*1000)] -= 1;

    }

}
fn toggle( arr: &mut [u8; 1000000], x : usize, y :usize)  {
    arr[(x+ y*1000)] += 2;
}
fn get(arr: &[u8; 1000000], x : usize, y :usize) -> u8{
    return arr[(x+ y*1000)];
}