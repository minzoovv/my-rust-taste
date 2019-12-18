use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Please guess the number!");
   
    loop {

        let mut guess = String::new();
        println!("Please enter the guessing number.");

        io::stdin().read_line(&mut guess)
            .expect("cannot read input value.");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
  
        println!("input value: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("입력한 수가 큽니다."),
            Ordering::Less => println!("입력한 수가 작습니다."),
            Ordering::Equal => {
                println!("정답입니다!");
                break;
            }
        }
    }
}
