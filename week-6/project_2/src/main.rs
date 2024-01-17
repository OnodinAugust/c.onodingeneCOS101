// Rust program to develop a Researchers Publication Incentive System.

use std::io;

   fn main() {

   for x in 0..500{
   println!("Researchers Publication Incentive System");
   
   let mut name = String::new();
   let mut number_papers_published = String::new();
 
   // input the name of the researcher 
   println!("\nEnter name of the Researcher: ");
   io::stdin().read_line(&mut name ).expect("Failed to read input");
   let name:String = name.trim().parse().expect("Invalid Number Papers Published");

   
   //input the number papers published
   println!("\nEnter Number Papers Published");
   io::stdin().read_line(&mut number_papers_published).expect("Not a valid string");
   let number_of_papers_published:i32 = number_papers_published.trim().parse().expect("Invalid number of papers published");
   
   if number_of_papers_published >=3 && number_of_papers_published <=5
    {
        println!("Incentive obtained is N500,000.");
    }

   if number_of_papers_published >5 && number_of_papers_published <10
    {
        println!("Incentive obtained is N800,000.");
    }
   
   if number_of_papers_published >=10
   {
        println!("Incentive obtained is N1,000,000.");
   }

   if number_of_papers_published <3
   {
        println!("Incentive obtained is N100,000.", );
   }
        println!(" count {}",x );
}
    
}

