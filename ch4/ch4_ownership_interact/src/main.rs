fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // s1はs2にムーブされたためエラーになる
    println!("{}, world!", s2);

    let m1 = String::from("hello");
    let m2 = m1.clone();

    // クローンするとヒープデータのコピーも行われるため問題なく動作する
    println!("m1 = {}, m2 = {}", m1, m2);
}
