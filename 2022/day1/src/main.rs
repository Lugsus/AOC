use std::fs;
fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");
    //print!("{}",input);
    let mut Elves_list:Vec<Elve> = Vec::new();

    //println!("{}",input);

    for line in input.lines(){

        if line.is_empty(){
           Elves_list.push(Elve::new(0));
        }

        else{

            'block:{ match Elves_list.last_mut(){
                    Some(i)=> i.add(line.parse::<i32>().unwrap()),
                    None => {Elves_list.push(Elve::new(0)); break 'block}

                }
            }
        }
    }
    let mut top3:[i32;3]= [0;3];

    for (_pos, elves) in Elves_list.iter().enumerate(){
        if elves.get_calories() > top3[0]{
            top3[2] = top3[1];
            top3[1] = top3[0];
            top3[0]= elves.get_calories();
        }
        else if elves.get_calories()> top3[1] {
            top3[2] = top3[1];
            top3[1] = elves.get_calories();          
        }
        else if elves.get_calories()> top3[2] {
            top3[2] = elves.get_calories();
            
        }
    }


print!("Most cal: {:?}, Sum: {}",top3, top3[0]+top3[1]+top3[2] )


    
}

struct Elve {

    calories:i32
}

impl Elve{
    fn new(calorie:i32) -> Self{
        Self{
            calories : calorie
        }
    }

    fn add(&mut self, calorie:i32, ){
        self.calories += calorie;

    }

    fn get_calories(&self) -> i32{
        self.calories

    }
   
}
