fn main() {
    println!("Hello, world!");

    another_function();
    another_function2(5);
    another_function3(5, 6);

    let a = 5;
    // ブロックは式であるため結果値が評価される
    let b = {
        let a = 3;
        a + 1
    };
    println!("The value of b is: {}", b);

    let f = five();
    println!("The value of f is: {}", f);

    let p = plus_one(6);
    println!("The value of p is: {}", p);
}

// 関数の定義箇所は問わない
fn another_function() {
    println!("Another function.");  // 別の関数
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);   // xの値は{}です
}

fn another_function3(y: i32, z: i32) {
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

// 返り値がある関数
fn five() -> i32 {
    5 // 式（セミコロンをつけると文になるのでつけない）
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
