fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);     // xの値は{}です
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;    // シャドーイング
    let y = y * 2;
    println!("The value of x is: {}", y);
}
