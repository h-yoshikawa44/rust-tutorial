fn main() {
    // 整数型
    let guess: u32 = "42".parse().expect("Not a number!");    // 数字ではありません！
    println!("guess: {}", guess);

    // 浮動小数点数型
    let x = 2.0; // f64
    println!("f64: {}", x);
    let y: f32 = 3.0; // f32
    println!("f32: {}", y);

    // 足し算
    let sum = 5 + 10;
    println!("sum: {}", sum);
    // 引き算
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    // 掛け算
    let product = 4 * 30;
    println!("product: {}", product);
    // 割り算
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    // 余り
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    // 論理値型
    let t = true;
    let f: bool = false; // 明示的型注釈付きで
    println!("bool: {}, {}", t, f);

    // 文字型
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫
    println!("char: {}, {}, {}", c, z, heart_eyed_cat);

    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (one, two, three) = tup;
    println!("tuple: {}, {}, {}", one, two, three);
    println!("tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    // 配列型
    let arr = [1, 2, 3, 4, 5];
    println!("array: {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);
}
