#![feature(iter_array_chunks)]
use std::fs;


fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");
    let mut total_sum = 0;

    let result:u16 = input
    .lines()
    .array_chunks::<3>()
    .map(|[a,b,c]|{
        let mut value = 0;    
            a.chars()
            .for_each(|char|{
            if b.find(char).is_some() && c.find(char).is_some(){
                let mut  dst= [0;2];
                let encoded = &char.encode_utf16(&mut dst);
                let byte_value = encoded[0];
                if char.is_ascii_uppercase() {
                    value = byte_value-64+26;
                    
                }
                else{
                    value = byte_value-70-26;     
                }
            }
            
            });
            println!("{}",value);
            value
        })
        .sum();
    println!("Badges: {}",result);



    'nextline: for line in input.lines(){

        let first = line.split_at(line.len()/2).0;
        let second = line.split_at(line.len()/2).1;

        for chars1 in first.chars(){
            for chars2 in second.chars(){
                if chars1 == chars2{
                    let mut  dst= [0;2];
                    let encoded = &chars1.encode_utf16(&mut dst);
                    let byte_value = encoded[0];
                    if chars1.is_uppercase(){
                    
                    total_sum += byte_value-64+26;
                }
                    else{
                        total_sum += byte_value-70-26;
                    }
                    continue 'nextline;
                   
                }
            }
        } 
        }
        println!("{}",total_sum)
    }

