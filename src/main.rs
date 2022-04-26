use std::io;
use std:: cmp::Ordering;
use rand::Rng;

fn main() {

  println! ("Welcome Rustaceans");
  
  println!("Guess the Number");

  let secret_number = rand::thread_rng().gen_range(1..100);
  
   loop {
  
    println!("Please input your guess ");

    let mut guess = String::new(); 
  
  
     io::stdin()
    .read_line(&mut guess)
    .expect("Fail to read line");

  let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
  };
  
  println!("You guessed:  {}", guess);

  match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too Small"),
      Ordering::Greater => println! ("Too Big"),
      Ordering::Equal => {
        println!("You Win");
        break;
      }
    
     }
  
   }

}
