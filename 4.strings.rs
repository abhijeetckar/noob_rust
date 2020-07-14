// fn main() {
//    let company:&str="Wassup";
//    let location:&str = "Hyderabad";
//    println!("company is : {} location :{}",company,location);
// }


// fn main() {
//    let company:&'static str = "Wassup";
//    let location:&'static str = "Hyderabad";
//    println!("company is : {} location :{}",company,location);
// }

// fn main(){
//    let empty_string = String::new();
//    println!("length is {}",empty_string.len());

//    let content_string = String::from("Wassup");
//    println!("length is {}",content_string.len());
// }

// fn main(){
//    let mut z = String::new();
//    z.push_str("hello");
//    println!("{}",z);
// }

// fn main(){
//    let name1 = "Hello Wassup , 
//    Hello!".to_string();
//    println!("{}",name1);
// }

// fn main(){
//    let name1 = "Hello Wassup , 
//    Hello!".to_string();         //String object
//    let name2 = name1.replace("Hello","Howdy");    //find and replace
//    println!("{}",name2);
// }

// fn main() {
//    let example_string = String::from("example_string");
//    print_literal(example_string.as_str());
// }

// fn print_literal(data:&str ){
//    println!("displaying string literal {}",data);
// }

// fn main(){
//    let mut company = "wassup".to_string();
//    company.push('s');
//    println!("{}",company);
// }

// fn main(){
//    let mut company = "wassups".to_string();
//    company.push_str(" Point");
//    println!("{}",company);
// }

// fn main() {
//    let fullname = " Wassup";
//    println!("length is {}",fullname.len());
// }

// fn main() {
//    let fullname = " Wassup \r\n";
//    println!("Before trim ");
//    println!("length is {}",fullname.len());
//    println!();
//    println!("After trim ");
//    println!("length is {}",fullname.trim().len());
// }

// fn main(){
//    let msg = "wassup has good".to_string();
//    let mut i = 1;
   
//    for token in msg.split_whitespace(){
//       println!("token {} {}",i,token);
//       i+=1;
//    }
// }

// fn main() {
//    let fullname = "Kannan,Sudhakaran,wassup";

//    for token in fullname.split(","){
//       println!("token is {}",token);
//    }

//    //store in a Vector
//    println!("\n");
//    let tokens:Vec<&str>= fullname.split(",").collect();
//    println!("firstName is {}",tokens[0]);
//    println!("lastname is {}",tokens[1]);
//    println!("company is {}",tokens[2]);
// }

// fn main(){
//    let n1 = "wassup ".to_string();

//    for n in n1.chars(){
//       println!("{}",n);
//    }
// }

// fn main(){
//    let n1 = "wassup ".to_string();
//    let n2 = "Point".to_string();

//    let n3 = n1 + &n2; // n2 reference is passed
//    println!("{}",n3);
// }