struct method{
    length:u32,
    bredth:u32,
}
impl method{
    fn area(&self)->u32{
        self.length * self.bredth
    }

}
fn main() {
    let detail = method{length:40,bredth:50};
    println!("The details of the struct using structs methods is {}",detail.area());
}
