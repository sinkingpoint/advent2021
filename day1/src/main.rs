use libadvent::*;

fn main() {
    let input = must_parse_to_ints(must_read_input_to_lines());
    let count = (1..input.len()).filter(|&i| input[i] > input[i-1]).count();
    let count2 = (1..input.len()-2).filter(|&i| input[i] + input[i+1] + input[i+2] > input[i-1] + input[i+1] + input[i]).count();
    
    println!("{}", count);
    println!("{}", count2);
}
