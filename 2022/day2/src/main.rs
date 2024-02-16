use std::{collections::HashMap, fs};
fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");

    let shape_score:HashMap<&str, i32>  = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let mut total_score = 0;
    
    for zeile in input.lines(){
        match zeile{

            "A Y" => {total_score += 6+2},
            "B Z" => total_score += 6+3,
            "C X" => total_score += 6+1,

            "A X" => total_score += 3+1,
            "B Y" => total_score += 3+2,
            "C Z" => total_score += 3+3,
            
            _ => total_score += shape_score.get(zeile.split_at(1).1.trim()).expect("HUH")

        }
    }

    print!("Score: {}",total_score)
}


//A :rock, B:paper, c: scissors
//X:rock 1. y:paper 2, z:scissors  3
//win: 6, draw:3, lost:0
