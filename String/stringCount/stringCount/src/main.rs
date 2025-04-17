fn main() {

    let  name  = "Dimka";

    let array = name.chars().nth(name.len() - 1).unwrap();

    println!("{} ", array);
}
