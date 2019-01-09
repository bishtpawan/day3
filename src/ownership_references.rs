fn main(){
    let s1 = String::from("ownership references");
    reference_ownership(&s1);
    println!("The value of s1 after assigning it {}",s1);
}

fn reference_ownership(s:&String){
    let length = s.len();
    println!("The length of reference variable {}",length);
}