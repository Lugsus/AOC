use std::fs;


fn main() {
    
    let input = fs::read_to_string("./Input.txt").expect("Couldn't read file");
    let mut overlaps = 0;
    let mut overlapsp2 = 0;
    for lines in input.lines(){

        let pair = lines.split_once(',').unwrap();
        let assignment1 = pair.0.split_once("-").unwrap();
        let assignment2= pair.1.split_once("-").unwrap();
        let assignment1_as_int = (assignment1.0.parse::<i32>().unwrap(),assignment1.1.parse::<i32>().unwrap());
        let assignment2_as_int = (assignment2.0.parse::<i32>().unwrap(),assignment2.1.parse::<i32>().unwrap());

        let assign1:Vec<i32> = (assignment1_as_int.0..assignment1_as_int.1+1).collect();
        let assign2:Vec<i32> = (assignment2_as_int.0..assignment2_as_int.1+1).collect();
        //println!("{:?},{:?}",assign1,assign2);
        let mut isoverlap:bool = false;
        for numbers in assign1.iter(){
            if assign2.contains(numbers){
                isoverlap = true;
            }
        }
        if isoverlap{
            overlapsp2 += 1;
        }

        

        if (assignment1_as_int.0 >= assignment2_as_int.0 && assignment1_as_int.1 <= assignment2_as_int.1 ) || (assignment2_as_int.0 >= assignment1_as_int.0 && assignment2_as_int.1 <= assignment1_as_int.1){
            //println!("{:?}-{:?}",assignment1_as_int,assignment2_as_int );
            overlaps += 1;
        }
    }
    println!("Overlaps:{}, Total-overlaps{}",overlaps,overlapsp2);
}

    


