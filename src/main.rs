use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
   println!("Guess the number");
   println!("Enter the number");

   let secret_number=rand::thread_rng().gen_range(1..=100);
   // println!("secret number:{secret_number}");
   loop{
      let mut guess=String::new();

      io::stdin()
      .read_line(&mut guess)
      .expect("failed to load lines");
      
      println!("you guessed :{guess}");
      let guess:u32= match guess.trim().parse(){
         Ok(num)=>num,
         Err(_)=>continue,

      };
      match guess.cmp(&secret_number){
          Ordering::Equal=>{
            println!("YOu won!!!");
            break;
         },
          Ordering::Greater=>println!("Too Big"),
          Ordering::Less=>println!("Too small"),
      }
   }
  
}
