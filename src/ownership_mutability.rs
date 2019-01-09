fn main(){
    let mut s = String::from("Mutability test with ownership");
    check(&mut s);
}
fn check(change_str:&mut String){
    change_str.push_str(" with reference variable");
    println!("The value after appending the mutability test is --> {} --> Here either we use one reference variable
    or more than on immutable variables", change_str);
}