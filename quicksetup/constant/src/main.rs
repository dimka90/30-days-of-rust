const PI: f64 = 3.142;
struct Circle {
    radius: f64,
}
impl Circle {
    fn area(&self) -> f64 {
        let result = (self.radius * self.radius) * PI;
        return result;
    }
}
fn main() {
    let circle1 = Circle { radius: 4.0 };
    println!("{:?}", circle1.area());

}
