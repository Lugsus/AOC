use std::{fs, iter::Sum};
fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");
    //print!("{}",input);
    let mut total_sum = 0;

    let my_vec = creat_array_from_input(&input);

    let mut symbol_tag:bool =false;
    let mut buffer_string:String = String::new();
    for (linenr, lines) in my_vec.iter().enumerate(){

        for (charpos, chars) in lines.iter().enumerate(){

            if chars.is_numeric(){
                buffer_string.push(*chars);
                if valid_neigthbour(linenr, charpos, &my_vec) {
                    symbol_tag = true;
                    
                }
                
            }
            if !chars.is_numeric() || *chars == '.'{

                if symbol_tag == true{
                    println!("Number found: {}", buffer_string);
                    let addnumber = buffer_string.parse::<i32>().unwrap();
                    total_sum += addnumber;
                    symbol_tag = false;
                    buffer_string = String::new();

                }
            }    
        }
        println!("Zwischensumme der Zeile: {}",total_sum)
    }

    print!("Endergebnis= {}",total_sum);
}


fn valid_neigthbour(linenr: usize, charpos:usize, my_vec:&Vec::<Vec::<char>> ) -> bool{
    let mut neighbors_valid = Vec::new();
    let matchtuple = (linenr,charpos);
    println!("Linenr: {} Charpos: {} Charveclen: {}",linenr, charpos,my_vec[linenr].len());
    match matchtuple {
        (0,0) => { println!("1");
        neighbors_valid = vec![
                    my_vec[linenr + 1][charpos] != '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                    my_vec[linenr + 1][charpos + 1].is_ascii_punctuation() || !my_vec[linenr + 1][charpos + 1].is_numeric(),
                    my_vec[linenr][charpos + 1].is_ascii_punctuation() || !my_vec[linenr][charpos + 1].is_numeric(),
                    ]
                }
        (0,charpos) if charpos< my_vec[linenr].len() =>   {println!("2");
            neighbors_valid = vec![
                    my_vec[linenr + 1][charpos - 1]!= '.' || !my_vec[linenr + 1][charpos - 1].is_numeric(),
                    my_vec[linenr + 1][charpos]!= '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                    my_vec[linenr + 1][charpos + 1]!= '.' || !my_vec[linenr + 1][charpos + 1].is_numeric(),
                    my_vec[linenr][charpos - 1]!= '.' || !my_vec[linenr][charpos - 1].is_numeric(),
                    my_vec[linenr][charpos + 1]!= '.' || !my_vec[linenr][charpos + 1].is_numeric(),
                    ]
                }
        (linenr,0) if linenr < my_vec.len() => {println!("3");
            neighbors_valid = vec![
                my_vec[linenr - 1][charpos]!= '.' || !my_vec[linenr - 1][charpos].is_numeric(),
                my_vec[linenr - 1][charpos + 1]!= '.' || !my_vec[linenr - 1][charpos + 1].is_numeric(),
                my_vec[linenr + 1][charpos]!= '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                my_vec[linenr + 1][charpos + 1]!= '.' || !my_vec[linenr + 1][charpos + 1].is_numeric(),
                my_vec[linenr][charpos + 1]!= '.' || !my_vec[linenr][charpos + 1].is_numeric(),
                 ]}

                 (0,charpos) if charpos> my_vec[linenr].len() => {println!("4");
                    neighbors_valid = vec![
                    my_vec[linenr + 1][charpos - 1]!= '.' || !my_vec[linenr + 1][charpos - 1].is_numeric(),
                    my_vec[linenr + 1][charpos]!= '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                    my_vec[linenr][charpos - 1]!= '.' || !my_vec[linenr][charpos - 1].is_numeric(),
                    ]}
            (linenr,charpos) if 0 < linenr && linenr < my_vec.len() && charpos == my_vec[linenr].len()-1 => {println!("7");
                    neighbors_valid = vec![
                        my_vec[linenr - 1][charpos - 1]!= '.' || !my_vec[linenr - 1][charpos - 1].is_numeric(),
                        my_vec[linenr - 1][charpos]!= '.' || !my_vec[linenr - 1][charpos].is_numeric(),
                        my_vec[linenr + 1][charpos - 1]!= '.' || !my_vec[linenr + 1][charpos - 1].is_numeric(),
                        my_vec[linenr + 1][charpos]!= '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                        my_vec[linenr][charpos - 1]!= '.' || !my_vec[linenr][charpos - 1].is_numeric(),

                    ]
            }

        (linenr,0) if linenr > my_vec.len() => {println!("5");
            neighbors_valid = vec![
                my_vec[linenr - 1][charpos]!= '.' || !my_vec[linenr - 1][charpos].is_numeric(),
                my_vec[linenr - 1][charpos + 1]!= '.' || !my_vec[linenr - 1][charpos + 1].is_numeric(),
                my_vec[linenr][charpos + 1]!= '.' || !my_vec[linenr][charpos + 1].is_numeric(),
                 ]}

            (linenr,charnr) if linenr == my_vec.len()-1 && charnr>0 && charnr< my_vec[linenr].len() => {println!("5");
            neighbors_valid = vec![
                my_vec[linenr - 1][charpos - 1]!= '.' || !my_vec[linenr - 1][charpos - 1].is_numeric(),
                my_vec[linenr - 1][charpos]!= '.' || !my_vec[linenr - 1][charpos].is_numeric(),
                my_vec[linenr - 1][charpos + 1]!= '.' || !my_vec[linenr - 1][charpos + 1].is_numeric(),
                my_vec[linenr][charpos - 1]!= '.' || !my_vec[linenr][charpos - 1].is_numeric(),
            ]


                    }

        (_,_) =>{println!("6");
            neighbors_valid = vec![
                    my_vec[linenr - 1][charpos - 1]!= '.' || !my_vec[linenr - 1][charpos - 1].is_numeric(),
                    my_vec[linenr - 1][charpos]!= '.' || !my_vec[linenr - 1][charpos].is_numeric(),
                    my_vec[linenr - 1][charpos + 1]!= '.' || !my_vec[linenr - 1][charpos + 1].is_numeric(),
                    my_vec[linenr + 1][charpos - 1]!= '.' || !my_vec[linenr + 1][charpos - 1].is_numeric(),
                    my_vec[linenr + 1][charpos]!= '.' || !my_vec[linenr + 1][charpos].is_numeric(),
                    my_vec[linenr + 1][charpos + 1]!= '.' || !my_vec[linenr + 1][charpos + 1].is_numeric(),
                    my_vec[linenr][charpos - 1]!= '.' || !my_vec[linenr][charpos - 1].is_numeric(),
                    my_vec[linenr][charpos + 1]!= '.' || !my_vec[linenr][charpos + 1].is_numeric(),
                ]}

    }

    let any_valid = neighbors_valid.iter().any(|&valid|valid);
    println!("-----------------------------------------------");
    any_valid
}



fn creat_array_from_input(input: &String) -> Vec::<Vec::<char>> {
    let mut my_vec = Vec::<Vec::<char>>::new();
    for (zeilennummer,zeilen) in input.lines().enumerate(){
        my_vec.push(Vec::new());
        for (stelle, character) in zeilen.chars().enumerate(){
            my_vec[zeilennummer].push(character);
        }
    }
    my_vec
}
