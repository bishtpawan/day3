use std::io;
use std::fs::File;
use std::io::Read;
fn main() {
    let a =get_file();
    println!("{:?}",a);
}
fn get_file()->Result<String,io::Error>{

    let mut f = File::open("abc.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)


}
