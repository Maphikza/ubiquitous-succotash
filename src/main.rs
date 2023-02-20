use std::io;
fn main() {
    
    let mut run = true;
    let mut sum: i32 = 0;

    while run {
        let int = get_int();
        if int < 0 {
            eprintln!("!!!You entered a negative number, This will not work with numbers less than 0. Try Again\n");
        } else {
            for i in 0..int + 1 {
                if (i % 2) == 0 {
                    sum = sum + i;
                }
                run = false;
            }
            println!("The sum of even numbers between 0 and {} is: {}", int, sum);
        }
    }
}

fn get_int() -> i32 {
    println!("We will give you a sum of even numbers between 0 and the number you enter.\nEnter a positive number:");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Could not readline.");

    let num: i32 = num
        .trim()
        .parse()
        .expect("This needs to be a number above zero.");

    num
}
