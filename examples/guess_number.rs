/* #![feature(old_io)] */
/* extern crate rand; */
/* use std::old_io as io; */
/* use rand::Rng; */
/* use std::from_str::FromStr; */
/*  */
fn main() {
/*     println!("Guess the number!"); */
/*  */
/*     let mut rng = rand::thread_rng(); */
/*     let secret_number = rng.gen::<u32>(); */
/*     /* println!("Secret number is {}", secret_number); */ */
/*  */
/*     let max_number_of_tries = 5; */
/*     let mut guesses: u32 = 0; */
/*     let mut reader = io::stdin(); */
/*  */
/*     loop { */
/*         if guesses == max_number_of_tries { */
/*           println!("You failed to guess within the limit of {} guesses!", guesses); */
/*           break; */
/*         } */
/*         println!("Please input guess number {}", guesses + 1); */
/*  */
/*         let input =  reader.lock().lines().next().unwrap().ok(); */
/*         let input_num: u32 = from_str::<u32>(input).unwrap().trim();  */
/*  */
/*         let num = match input  { */
/*             Ok(num) => num, */
/*             Err(num)      => { */
/*                 println!("Please input a number."); */
/*                 continue; */
/*             } */
/*         }; */
/*  */
/*         println!("You guessed: {}", num); */
/*         guesses += 1; */
/*  */
/*         if num < secret_number { */
/*             println!("Too small!") */
/*         } else if num > secret_number { */
/*             println!("Too big!") */
/*         }else { */
/*                 println!("You win!"); */
/*                 println!("You took {} guesses!", guesses); */
/*                 break; */
/*         } */
/*          */
/*     } */
}
