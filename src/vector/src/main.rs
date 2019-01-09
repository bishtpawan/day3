fn main() {
    let v=vec![10,20,30,40,50];
    let num:Option<&i32>=v.get(3);
    get_value(num);
}
fn get_value(n:Option<&i32>){
    match n{
        Some(n)=>println!("The forth value is {:?}",n),
        None=> println!("None"),
    }
}
