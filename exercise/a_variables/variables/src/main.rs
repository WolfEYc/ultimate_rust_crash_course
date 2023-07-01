
const STARTING_MISSILES: i32 = 8;
const READY_AMT: i32 = 2;

fn main() {
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMT);

    
    println!("Firing {ready} of my {missiles} missiles");

    let missiles_left = missiles - ready;

    println!("{missiles_left} missiles left");

}
