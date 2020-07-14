// let variable_name = value;           
// let variable_name:dataType = value;   


// fn main() {
//    let fees = 25_000;
//    let salary:f64 = 35_000.00;
//    println!("fees is {} and salary is {}",fees,salary);
// }


// fn main() {
//    let fees = 25_000;
//    println!("fees is {} ",fees);
//    fees = 35_000;
//    println!("fees changed is {}",fees);
// }

// let mut variable_name = value;
// let mut variable_name:dataType = value;
// Let us understand this with an example

// fn main() {
//    let mut fees:i32 = 25_000;
//    println!("fees is {} ",fees);
//    fees = 35_000;
//    println!("fees changed is {}",fees);
// }

// const VARIABLE_NAME:dataType = value;

// fn main() {
//    const USER_LIMIT:i32 = 100;    
//    const PI:f32 = 3.14;           

//    println!("user limit is {}",USER_LIMIT);  
//    println!("pi value is {}",PI);            
// }


// fn main() {
//    let salary = 100.00;
//    let salary = 1.50 ; 
//    // reads first salary
//    println!("The value of salary is :{}",salary);
// }

// fn main() {
//    let uname = "Mohtashim";
//    let uname = uname.len();
//    println!("name changed to integer : {}",uname);
// }

fn main() {
   const NAME:&str = "Mohtashim";
   const NAME:usize = NAME.len(); 
   println!("name changed to integer : {}",NAME);
   //Error : `NAME` already defined
}