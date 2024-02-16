use std::{collections::HashMap, fs};
fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");

    let _shape_score:HashMap<&str, i32>  = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let mut total_score = 0;
    
    for zeile in input.lines(){
        match zeile.split_at(1).1.trim(){

            "X" => {match zeile.split_at(1).0.trim() {
                "A" => total_score += 3,
                "B" =>total_score += 1,
                "C" =>total_score += 2,
                _ => println!("Fail bei X")
            }},
            "Y" => {match zeile.split_at(1).0.trim() {
                "A" => total_score += 3 + 1,
                "B" =>total_score += 3 + 2,
                "C" =>total_score += 3 + 3,
                _ => println!("Fail bei Y")
            }},
            "Z" => {match zeile.split_at(1).0.trim() {
                "A" => total_score += 6 + 2,
                "B" =>total_score += 6 + 3,
                "C" =>total_score += 6 + 1,
                _ => println!("Fail bei Z")
            }}
            _ => println!("scheisse")
          

        }
    }

    print!("Score: {}",total_score)
}


//A :rock, B:paper, c: scissors
//X:rock 1. y:paper 2, z:scissors  3
//win: 6, draw:3, lost:0
