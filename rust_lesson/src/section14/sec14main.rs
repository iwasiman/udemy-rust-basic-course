pub fn lesson() {
    println!("");
    println!("##### section14だよ ファイル操作とシステム");

    commandline_args();
    standard_inputs(false);
    file_inputs();


}

use std::env;
fn commandline_args() {
    println!("--------------- コマンドライン引数");
    let args: Vec<String> = env::args().collect();
    println!("引数を表示してみよう {:?}", args); // cargo run --bin rust_lesson aaa bbb で ["target/debug/rust_lesson", "aaa", "bbb"]

}

use::std::io;
fn standard_inputs(do_input: bool) {
    if do_input == false {
        return;
    }
    println!("--------------- 標準入力");
    println!("文字列を入力してください。");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap(); // 結果はResult型。ここではunwrapする
    println!("入力された文字列を出すよ {:?}", input); // 入力された文字列を出すよ "abvcd\n" 結果はString型。
    let num: i32 = input.trim().parse().unwrap_or(-1);
    println!("入力された数値を出すよ {:?}", num); // 数字じゃなかったら−1
    let result = input.trim().parse::<i32>();
    match result {
        Ok(x) => println!("数値だよ {}", x),
        Err(e) => println!("数値じゃないよ {}", e),
    }

}

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fs;
fn file_inputs() {
    println!("--------------- ファイル入力");
    let mut f: File = File::open("src/sample1.txt").unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    println!("File::open して read_to_stringでファイルを読み込む {}", contents); //中身が出る

    let contents2 = fs::read_to_string("src/sample1.txt").unwrap();
    println!("1行で読み込む {}", contents2); //中身が出る

    // 1行づつ読み込む
    let mut buf_reader = BufReader::new(f);
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    println!("1行づつ先頭から読み込む {}", line); // Rust改行
    buf_reader.read_line(&mut line).unwrap();
    println!("1行づつ先頭から読み込む {}", line); // Rust改行Python改行

    let lines = buf_reader.lines();
    for l in lines {
        println!("イテレーターで読み込む {}", l.unwrap()); // 残りの行が読み込まれる
    }

    // バイト配列で読み込む方法
    // ここで変数fを使おうとすると、let mut buf_reader = BufReader::new(f); でもうborrowされてるのでだめ。
    let mut f_bytes: File = File::open("src/sample1.txt").unwrap();
    let mut bytes = Vec::new();
    f_bytes.read_to_end(&mut bytes).unwrap();
    println!("バイト配列で読み込むよ {:?}", bytes); // [82, 117, 115, 116, 10, 80, 121, 116, 104, 111, 110, 10, 74, 97, 118, 97, 10, 67, 10, 67, 43, 43]
    println!("String::from_utf8()で変換 {:?}", String::from_utf8(bytes).unwrap()); //"Rust\nPython\nJava\nC\nC++"


}