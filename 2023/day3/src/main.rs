use std::fs;

fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");
    //print!("{}",input);
    let mut total_sum = 0;
                
    for (linenum, line) in input.lines().enumerate() {

        for (part,line) in line.split(".").enumerate(){

      
        }


        for (charnum, char) in line.chars().enumerate(){
            if !char.is_alphanumeric() && char != '.' {
                total_sum += find_left_number(line, charnum);
                find_right_number(line, charnum);

            }
        }
        let line_buffer = line;
    }
    println!("Total sum: {}", total_sum)
}

fn find_left_number(line:&str,charnum:usize) -> i32{
    let mut sum:i32 = 0;
    let mut leftiterator = 1;
    let mut left_number_as_string = String::new();
    while let Some (i) = line.chars().nth(charnum-leftiterator){
        if i.is_numeric(){
            left_number_as_string.push(i);
            leftiterator += 1;
            if leftiterator == charnum {
                break;
            }
        }
        else{
            break
        }
        //print!("Line {},Number {}",linenum, line.chars().nth(charnum-1).unwrap());
    }

    if !left_number_as_string.is_empty(){
    let left_number_as_string = left_number_as_string.chars().rev().collect::<String>();
    sum += left_number_as_string.parse::<i32>().unwrap();
    }
    sum

}

fn find_right_number(line:&str,charnum:usize){
    let mut sum:i32 = 0;
    let mut iterator = 1;
    let mut number_as_string = String::new();
    while let Some (i) = line.chars().nth(charnum+iterator){
        if i.is_numeric(){
            number_as_string.push(i);
            iterator += 1;
            if iterator == charnum {
                break;
            }
        }
        else{
            break
        }
        //print!("Line {},Number {}",linenum, line.chars().nth(charnum-1).unwrap());
    }

    if !number_as_string.is_empty(){
    println!("Number: {}", number_as_string);
    sum += number_as_string.parse::<i32>().unwrap();
    }

}

fn find_below_right_number(line:&str,charnum:usize){
    let mut sum:i32 = 0;
    let mut iterator = 1;
    let mut number_as_string = String::new();
    while let Some (i) = line.chars().nth(charnum+1+iterator){
        if i.is_numeric(){
            number_as_string.push(i);
            iterator += 1;
            if iterator == charnum {
                break;
            }
        }
        else{
            break
        }
        //print!("Line {},Number {}",linenum, line.chars().nth(charnum-1).unwrap());
    }

    if !number_as_string.is_empty(){
    println!("Number: {}", number_as_string);
    sum += number_as_string.parse::<i32>().unwrap();
    }

}