struct Rectangle{
    height:u32,
    width:u32,
}
fn main(){
    let rect = Rectangle{
        height:30,
        width:40,
    };
    println!("The area of rectable is {}",area(&rect));
}

fn area(rectangle:&Rectangle) ->u32{
    rectangle.height * rectangle.width
}
