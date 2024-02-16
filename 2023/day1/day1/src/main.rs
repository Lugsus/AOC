use std::fs;

fn main() {


    let file_path:&str ="C:/Users/lukas/AOC/day1/day1/data/Input.txt";
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //println!("With text:\n{contents}");
    let mut total_number: i32 = 0;

    for zeile in contents.lines() {
        let convertierte_zeile = replacewords(zeile);
        println!("{},       {}",convertierte_zeile,zeile);
        let mut numbers = String::new();
        for char in convertierte_zeile.chars() {
            if char.is_numeric(){
                numbers.push(char)
            }
        }
        if numbers.chars().count()==1 {
            match numbers.parse::<i32>(){ 
                Ok(numbers) =>{total_number += numbers*11;},
                Err(_) => {println!("no number found")},
            }
        }
        else{
            let first_number: char = numbers.chars().next().unwrap();
            //let last_number: char = numbers.chars().nth(numbers.len() - 1).unwrap();
            let last_number: char = numbers.chars().last().unwrap();
            let new_int: i32 = format!("{}{}", first_number, last_number).parse::<i32>().unwrap();
            total_number += new_int;
        }
    }
    println!("Total number {:}", total_number)
}

fn replacewords(line:&str) ->String{
    let mut zeile = String::from(line);
    zeile = zeile.replace("one","1" );
    zeile = zeile.replace("two","2" );
    zeile = zeile.replace("three","3" );
    zeile = zeile.replace("four","4" );
    zeile = zeile.replace("five","5" );
    zeile = zeile.replace("six","6" );
    zeile = zeile.replace("seven","7" );
    zeile = zeile.replace("eight","8" );
    zeile = zeile.replace("nine","9" );
    zeile

}

fn replace_new(line: &str) -> String{
    let words_to_find = ["one","two","three","four","five","six","seven","eight","nine"];
    let mut zeile = String::from(line);

    for word in words_to_find {
        if let Some(index) = zeile.find(word) {
            if index < lowest_index {
                lowest_index = index;
                found_word = Some(word);
            }
            if index > highest_index {
                highest_index = index;
            }
        }
    }





}