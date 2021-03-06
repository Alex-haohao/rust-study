use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess a number");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("请输入你的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you read is :{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
