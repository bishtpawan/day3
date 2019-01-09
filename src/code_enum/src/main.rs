enum Coin{
    penny,
    nickel,
    dime,
    quarter,
}
fn check_coin(coin :Coin) ->u32{
   let num_coin =  match coin{
        Coin::penny=>1,
        Coin::nickel=>5,
        Coin::dime=>10,
        Coin::quarter=>15,

    };
    num_coin
}
fn main() {
    let pen= Coin::penny;
    let nic= Coin::nickel;
    let dic= Coin::dime;
    let qua= Coin::quarter;
    println!("The value is {}",check_coin(pen));

}