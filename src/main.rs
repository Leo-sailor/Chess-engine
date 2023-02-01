use std::io;
fn main() {
    use std::time::Instant;
    println!("Please eneter your number.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Your number: {guess}");
        let num: u64 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 1,
        };
    let now = Instant::now();
    // Code block to measure.
    let res = lowest_multiple(num);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("number was: {}", res)

    }


fn lowest_multiple(inp: u64) -> u64{
    let mut current:u64 = 0;
    const NUMBERS: [char; 2] = ['0','1'];
    let mut curr_string = String::new();
    let mut passing: bool = true;
    let result = loop {
        current = current + inp;
        curr_string = current.to_string();
        passing = true;
        for cha in curr_string.chars() {
            if !NUMBERS.contains(&cha) {
                passing = false;
            }
        }
        if passing {
            break current
        }
    };
    result
}
