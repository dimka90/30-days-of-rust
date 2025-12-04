fn main() {
    println!("Hello, world!");

    let a = 20;
    let b = 4;

    println!("Add: {}", add(a, b));
    println!("Sub: {}", sub(a, b));

    #[cfg(feature = "mul")]
    {
        println!("Mul: {}", mul(a, b));
    }

    #[cfg(feature = "expo")]
    {
        match div(a, b) {
            Ok(result) => println!("Div: {}", result),
            Err(e) => println!("Div Error: {}", e),
        }
    }

}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn sub(a: u32, b: u32) -> u32 {
    a - b
}
#[cfg(feature = "mul")]
fn mul(first_number: u32, second_number: u32) -> u32{

    return  first_number * second_number;
}

#[cfg(feature = "div")]
fn div(first_number: u32, second_number: u32) ->Result<u32, String>{
    if second_number == 0{
        return  Err("Error: Zero division Error".to_string());
    }
    Ok(first_number / second_number)
}