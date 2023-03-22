const STARTING_MISSILESI:i32=8;
const READY_AMOUNT:i32=2;
fn main() {
    let mut missiles=STARTING_MISSILESI;
    let ready=READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
