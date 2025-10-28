fn stake_eth(eth_value: i32) -> bool {
    if eth_value < 32 {
        return false;
    }
    return true;
}
struct Validator {
    address: String,
    eth_to_stake: i32,
}
fn main() {
    let validator = Validator {
        address: "0x1234".to_string(),
        eth_to_stake: 21,
    };
    if stake_eth(validator.eth_to_stake) {
        println!("User can be a validator")
    } else {
        println!("You don't have much Eth to be a validator")
    }

    println!("Hello, world!");
}
