use std::{ io, string};
use rand::Rng;


const MILLISECONDS:i32 = 1000;
const MICROSECONDS:i32 = MILLISECONDS *1000;
const NANOSECONDS:i32 = MICROSECONDS *1000;
//const variables are not infered by the complier , you have to manually change them , has to be all CAPS
fn main() {
   //this is a string in rust
   let s: &str = "Hello, World";
   //all printing has to be in this for

   let mut name =String::from("Juan ");
   name.push_str("Martinez");
   println!("{name}");
   //in rust everything is immutable by default
   
   //data types
   let x:i8 = 0;
   let x_2:i16 = 0;
   let x_3:i32 =0 ;//default value if not specified
   let x_4:i64 =0;
   let x_5:i128=0; //stores large numbers
   // i8-128 store positive and negatives u8-128 store only positive starting with 0

   let f :f32 = 1667.21;
   let f_1 :f64 = 1667.21;
   //f32 and f64 store floats
   let bool1 = true;
  let bool2 = false; 

  //shadowing of a variable is used to change the variable at one place for example

  let num =323;

  let num =20;
  //can be done in rust as is called shadoeing and maylead to less errors because than ig let mut num =5; could be changed down the line 
// print_console();
let mut wins_count = 0;
while wins_count<2{
   let guessed_correctly =computer_guess();
if guessed_correctly{
   wins_count+=1;

}
println!("You have won {wins_count} times");
    
}
}

fn print_console(){
   println!("Guess a number 1-10");
   let mut user_input = String::new();

   io::stdin().read_line(&mut user_input).expect("Failed to read number");

   println!("You entered: {user_input}");

}


fn computer_guess()-> bool{
   println!("Guess a number 1-10");
   let mut user_input = String::new();

  io::stdin().read_line(&mut user_input).expect("Failed to read number");

  let user_guess:i8 = user_input.trim().parse().expect("Please Enter a valid number!");
   let rng = rand::random_range(1..=10);
   if rng ==user_guess{
      println!("You Guess Correctly!");
      return true;

   };
   println!("computer entered: {rng}");
   println!("You entered: {user_guess}");
   println!("Try again!");
   false

}
