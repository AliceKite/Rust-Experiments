fn main() {
   let x = 5;
   println!("the value of x is {x}");
   let x = x + 1;
   println!("the value of x is {x}");

   {
    let x = x * 5;
    println!("x within this scope is {x}");
   }

   println!("but x out here is {x}");

   let spaces = "     ";
   let spaces = spaces.len();
   println!("space is {spaces}");

   const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
   println!("three hours is {THREE_HOURS_IN_SECONDS} seconds.");
}