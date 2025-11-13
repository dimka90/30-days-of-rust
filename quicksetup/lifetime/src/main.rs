
fn first_element(list: &[i32]) -> &i32 {
    &list[0]
}

fn pick_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

struct Message<'a> {
    content: &'a str,
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    first_element(&numbers);
    let name = "Dimka";
    let surname = "Tom";
    println!("{:?}", pick_longer(&name, &surname));

    let text = String::from("Hello");
    let msg = Message { content: &text };
    println!("{}", msg.content);
    let name = "dimka";
    println!("{name}");
}
