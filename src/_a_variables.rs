const STARTING_MISSILES:i32 = 8; //explicit annotation
const READY_AMOUNT:i32 = 2;
READY_AMOUNT = 1;

fn main() {
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    let random_var = 0;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles = missiles - ready;
    println!("{} missiles left", missiles-ready);
}
