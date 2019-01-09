#[derive(Debug)]
struct print_struct{
    name:String,
    email:String,
}
fn main() {
    let detail = print_struct{
        name:String::from("Pawan"),
        email:String::from("abc@"),
    };

    println!("The struct is {:?}",detail);

}
