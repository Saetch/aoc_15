
use crate::seven_input::get_input;

pub struct Action{
    _target : String,

    _first_argument : String,
    _second_argument :String,
    _command: String,
    solved : bool,
    _value: u32,
}

struct Vector{
    vec : Vec< Action>
}

impl Vector{
    pub fn solve_flat(&mut self, act : String) -> bool{
        let mut selfo =  Action::new(String::from(""), String::from(""), String::from(""), String::from("")) ;
        let mut index = 0;
        for i in 0..self.vec.len()  {
            let action = &mut self.vec[i];
            

            if action._target.replace(" ", "") == act{
                index = i;
                if self.vec[i].solved{
                    return true;
                }
               // selfo = &mut self.vec[i];

            }
            
        }
        
        selfo._target = (self.vec[index]._target).clone();
        selfo._first_argument = (self.vec[index]._first_argument).clone();
        selfo._second_argument = (self.vec[index]._second_argument).clone();
        selfo.solved = self.vec[index].solved;
        selfo._value = self.vec[index]._value;
        selfo._command = self.vec[index]._command.clone();
        if selfo.solved{
            return true;
        }
        let mut arg1 = 0;
        let mut arg2 = 0;
        

        if selfo._second_argument.replace(" ", "") == "NOT"{
            if selfo._first_argument.replace(" ","").parse::<u32>().is_err(){
                for act in &mut self.vec  {
                    if act._target.replace(" ", "") == selfo._first_argument.replace(" ", "") {
                        if act.solved{
                            selfo.solved = true;
                            selfo._value = ! act._value;
                            self.vec.remove(index);
                            self.vec.push(selfo);
                            return true;
                        }else{
                            return true;
                        }
                    }
                }
                println!("ACTION {} not found in array!", self.vec[index]._to_string());
                return true;
            }else{
                arg1 = selfo._first_argument.replace(" ","").parse::<u32>().unwrap();
                selfo.solved = true;
                selfo._value = ! arg1;
                self.vec.remove(index);
                self.vec.push(selfo);
                return true;
            }
        }else if selfo._command.replace(" ", "") == "DIRECT" {
            if selfo._first_argument.replace(" ","").parse::<u32>().is_err(){
                for act in &mut self.vec  {
                    if act._target.replace(" ", "") == selfo._first_argument.replace(" ", "") {
                        if act.solved{
                            selfo.solved = true;
                            selfo._value = act._value;
                            self.vec.remove(index);
                            self.vec.push(selfo);
                            return true; 
                        }else{
                            return true;
                        }
                    }
                }
                println!("ACTION {} not found in array!", self.vec[index]._to_string());
                return true;
            }else{
                arg1 = selfo._first_argument.replace(" ","").parse::<u32>().unwrap();
                selfo.solved = true;
                selfo._value = arg1;
                self.vec.remove(index);
                self.vec.push(selfo);
                return true;
            }
        }
        else{
            let mut fbool;
            let mut sbool;
            if selfo._first_argument.replace(" ","").parse::<u32>().is_err(){
                fbool = true;
            }else{
                fbool = false;
                arg1 = selfo._first_argument.replace(" ","").parse::<u32>().unwrap();
            }
            if selfo._second_argument.replace(" ", "").parse::<u32>().is_err(){
                sbool = true;
            }else{
                sbool = false;
                arg2 = selfo._second_argument.replace(" ","").parse::<u32>().unwrap();
            }

            if fbool || sbool{
                for act  in &mut self.vec  {
                    if act._target.replace(" ", "") == selfo._first_argument.replace(" ",""){
                        if act.solved{
                            fbool = false;
                            arg1 = act._value;
                        }
                    }
                    if act._target.replace(" ", "") == selfo._second_argument.replace(" ", ""){
                        if act.solved{
                            sbool = false;
                            arg2 = act._value;
                        }
                    }
                }
            }

            if fbool || sbool{
                return true;
            }
            selfo.solved = true;
            match selfo._command.replace(" ", "").as_str() {
                "AND" => selfo._value = arg1 & arg2,
                "OR" => selfo._value = arg1 | arg2,
                "LSHIFT" => selfo._value = arg1 << arg2,
                "RSHIFT" => selfo._value = arg1 >> arg2,
                " " => (),
                "" => (),
                _ => println!("{} --> _command NOT FOUND IN STRING {}!",selfo._command, selfo._to_string())
            }
            self.vec.remove(index);
            self.vec.push(selfo);
            return true;
        }
    }
}

impl Action{

    pub fn new(t :String, f_a : String, s_a : String, com : String) -> Self{
        return Action{ _target : t,_first_argument: f_a, _second_argument : s_a, solved : false, _command: com, _value : 0}
    }
    
    pub fn new_direct(t: String, f_a : String) -> Self{
        return Action{ _target: t, _first_argument : f_a, _second_argument: String::from(""), solved: false, _command: String::from("DIRECT"), _value: 0};

    }



    pub fn _to_string(&self) -> String{
        if self._command == "NOT "{
            return String::from("NOT ") + self._first_argument.as_str() +" -> " + self._target.as_str();
        }else{
            return String::from( self._first_argument.to_owned() + " " + self._command.as_str()+" " + self._second_argument.as_str() +" -> "+ self._target.as_str());
        }
    }


}

pub fn seven(){
    let input = get_input();

    let mut vec = Vector{ vec: Vec::new()};
    for mut line in input.lines(){
        while line.starts_with(" ") {
            line = line.split_at(1).1;
        }
        if line.len() <3{
            continue;
        }
        if line.starts_with("NOT"){
            vec.vec.push( Action::new(String::from(line.split(" -> ").skip(1).next().unwrap()), String::from(line.split(" ").skip(1).next().unwrap()), String::from("NOT"), String::from("")));
        }
        else if line.contains("OR") || line.contains("AND") || line.contains("LSHIFT") || line.contains("RSHIFT"){
            vec.vec.push(  Action::new( String::from(line.split(" -> ").skip(1).next().unwrap()), String::from(line.split(" ").next().unwrap()), String::from(line.split(" ").skip(2).next().unwrap()), String::from(line.split(" ").skip(1).next().unwrap())));
        }else{
            if String::from(line.split(" ").next().unwrap()).parse::<u32>().is_err(){
                println!("ELSEELSE BUT WITH ERR");

                vec.vec.push(Action::new_direct(String::from(line.split(" -> ").skip(1).next().unwrap()), String::from(line.split(" ").next().unwrap())));
            }else{
                println!("ELSEELSE");

                vec.vec.push( Action::new_direct(String::from(line.split(" -> ").skip(1).next().unwrap()), String::from(line.split(" ").next().unwrap())));

            }
        }
    }
    let mut solved = 0;
    while solved < vec.vec.len() {
        for i in 0..vec.vec.len(){
            let act = vec.vec[i]._target.replace(" ", "");
            vec.solve_flat(act);
    
        }
        solved = 0;
        for i in 0..vec.vec.len(){
            if vec.vec[i].solved{
                solved +=1;
            }
        }
        println!("FOUND {} solved variables this run! needing {}", solved, vec.vec.len());

    }

    for i in 0..vec.vec.len(){
        if !vec.vec[i].solved {
            println!("NOT SOLVED: {} ", vec.vec[i]._to_string());
        }
       // println!("{}  --> {}", vec.vec[i]._value, vec.vec[i]._target);
    }
    let mut  index_a = 0;
    for i in 0 ..vec.vec.len(){
        if vec.vec[i]._target == "a"{
            println!(" a is : {}!", vec.vec[i]._value);
            index_a = i;
        }
    }

    for i in 0..vec.vec.len(){
        if vec.vec[i]._target == "b"{
            vec.vec[i]._value = vec.vec[index_a]._value;
        }
        else{
            vec.vec[i].solved = false;
        }
    }


    let mut solved = 0;
    while solved < vec.vec.len() {
        for i in 0..vec.vec.len(){
            let act = vec.vec[i]._target.replace(" ", "");
            vec.solve_flat(act);
    
        }
        solved = 0;
        for i in 0..vec.vec.len(){
            if vec.vec[i].solved{
                solved +=1;
            }
        }

    }

 
    for i in 0 ..vec.vec.len(){
        if vec.vec[i]._target == "a"{
            println!(" a is : {} after overriding b!", vec.vec[i]._value);
        }
    }
}


