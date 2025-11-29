use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main (){
    println!("Guss the Number"); 
	
	let rand_number = rand::thread_rng().gen_range(1..=100);
	
	println!("the random numb is: {rand_number}"); 	
	
	
	
    println!("What number you are gussing ? from 1 to 100 ");
	
    let mut guess = String::new(); 
    io::stdin()
    .read_line(&mut guess)
    .expect("Bhai Bakchodi mat kar ");
	
	let guess: u32 = guess.trim().parse().expect("Bhai number lik chup chap");
	println!("Bhai tune: {guess} guess kiya");
	
	match guess.cmp(&rand_number) {
		Ordering::Less => println!("bhai tera number chota hai"),
		Ordering::Greater => println!("bhai tera number bada hai"),
		Ordering::Equal => println!("Tu hero hai bhai jeet gaya "),
		
		
	}



}