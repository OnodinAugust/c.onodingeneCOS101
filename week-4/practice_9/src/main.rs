fn main() {
   let a:i32 = 10;
   let b:i32 = 20;

   println!("Value of a: {} ",a);
   println!("Value of B: {} ",b);

   let mut res = a>b ; 
   println!("A greater than b: {} ",res);

   res = a <b ;
   println!("A lesser than b: {} ",res) ;

   res = a>=b ;
   println!("a greater than or equal to b: {} ",res);

   res = a <=b;
   println!("a lesser than or equal to b: {} ",res) ;

   res = a==b ;
   println!("a is equal to b: {} ",res) ;

   res = a!=b ;
   println!("a is not equal to b: {} ",res);
}
