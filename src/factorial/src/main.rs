use std::io;
fn main() {
    let mut num=String::new();
    let mut result=1;
    println!("Enter any number");
    io::stdin().read_line(&mut num).expect("Not able to read input");
    let mut n: u32 = match num.trim().parse().expect("not a string") {
        "one" => 1,
        "two" => 2,
    };

    while n!=0{
        result=result*n;
        n=n-1;
    }
    println!("The factorial of the number is {} ,{}",num,result);

}
