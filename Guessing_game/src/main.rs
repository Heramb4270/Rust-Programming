use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the Number Program");

   

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("Secret Number is {}",secret_num);
    loop{

        
        println!("Enter the Number You Want : ");
        let mut guess = String::new();

        io::stdin()
                .read_line(&mut guess)
                .expect("Failure !!");
        let guess : u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_)=>continue,
        };
        println!("You Guessed {guess}");

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal =>{
                println!("Equal");
                break;    }
        }
    }   
}
