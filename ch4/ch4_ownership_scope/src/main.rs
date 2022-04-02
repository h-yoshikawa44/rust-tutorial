fn main () {
    let s = String::from("hello"); // sはここから有効になる

    println!("The value of s is: {}", s);
    // sで作業をする
}   // このスコープはここでおしまい。sはもう有効ではない

// Rust においては、ひとたびメモリを所有している変数がスコープを抜けたらメモリは自動的に返還される
