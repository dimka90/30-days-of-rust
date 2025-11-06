fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = numbers.iter().map(|num| num * num).collect();
    println!("{:?}", squares);

    let ages = vec![12, 45, 18, 7, 33, 16, 21];

    // TODO: Use iterators to get only ages 18+
    // Expected: [45, 18, 33, 21]
    let adults: Vec<_>= ages.iter().filter(|&&age|  age >= 18).collect();
    println!("{:?}", adults);
}
