
fn main() {
    println!("The value after match using Option is {:?}",check_match(5));
}
fn check_match(num:i32)->Option<i32>{
    match check(num){
        Some(i)=>Some(i+1),
        None=>None,
    }
}
fn check(n:i32)->Option<i32>{
    Some(n)
}