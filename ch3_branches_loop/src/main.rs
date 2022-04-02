fn main() {
    // 無限ループ
    // loop {
    //     println!("again!");   // また
    // }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }

    // Range 型との組み合わせ
    for number2 in (1..4).rev() {
        println!("{}!", number2);
    }
    println!("LIFTOFF!!!");
}
