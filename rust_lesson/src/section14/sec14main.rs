pub fn lesson() {
    println!("");
    println!("##### section14だよ ファイル操作とシステム");

    commandline_args();
    standard_inputs(false);
    file_inputs();
    file_outputs();
    manipulate_jsons();
    manipulate_pathes();
    file_system_access_functions();


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

use std::fs::OpenOptions;

fn file_outputs() {
    println!("--------------- ファイル出力");
    let mut f1 = File::create("src/sample2.txt").unwrap();
    let bytes = b"write sample!/n";
    println!("バイトの文字列 {:?}", bytes); //[119, 114, 105, 116, 101, 32, 115, 97, 109, 112, 108, 101, 33, 47, 110]
    f1.write_all(bytes).unwrap(); //src/sample2.txtができる。

    let mut f2 = File::create("src/sample3.txt").unwrap();
    writeln!(f2, "Hello, {}", "Rust").unwrap(); // マクロを使うと直接文字列を出力できる。

    // ファイルオープンの補足
    // ファイルが存在したら上書きでなく末尾に追加にする
    let mut f_op1 = OpenOptions::new()
    .append(true)
    .open("src/sample1.txt").unwrap();
    writeln!(f_op1, "ついかだよ").unwrap();

    // ファイルが存在しない場合のみ作成
    // let mut f_op2 = OpenOptions::new()
    // .write(true)
    // .create_new(true)
    // .open("src/sampleNew.txt").unwrap();
    // writeln!(f_op2, "ついかだよ").unwrap();

    // Rustは変数がドロップされる時点で自動的にファイルクローズが行われるのでコード不要。クローズ忘れも起こらない。素晴らしい！!
}

use serde::{Serialize, Deserialize};
fn manipulate_jsons() {
    println!("--------------- JSONの操作");
    // serde サード serialize + deserialize の造語。

    // Cargo.toml のdependenciesに 追加。deriveはderiveアトリビュートで諸々使えるようにするため。
    // serde = { version = "1.0", features = ["derive"]}

    let p = Person {
        name: String::from("Smith"),
        age: 99,
        phones: vec![
            String::from("080-xxx"),
            String::from("080-xxx"),
        ],
    };
    let json_data = serde_json::to_string_pretty(&p).unwrap();
    println!("変換されたJSON文字列 {}", json_data);
    let mut f = File::create("src/sample.json").unwrap();
    writeln!(f, "{}", json_data).unwrap();

    let f = File::open("src/sample.json").unwrap();
    let buf_reader = BufReader::new(f);
    let data: Person = serde_json::from_reader(buf_reader).unwrap(); // 構造体に変換する時は型の指定が必要。
    println!("ファイルからJSONを読んで構造体に {:?}", data);



}

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8, // こうすると0−255 8bit符号なし整数型
    phones: Vec<String>,
}

use std::path::{Path, PathBuf};
fn manipulate_pathes() {
    println!("--------------- パスの操作");
    let path = Path::new("src");
    println!("パス src は存在するか {:?}", path.exists()); // true
    println!("パス src はディレクトリか {:?}", path.is_dir()); // true
    println!("パス src はファイルか {:?}", path.is_file()); // false
    println!("ファイル名かディレクトリ名を返す {:?}", path.file_name()); // Some("src")

    // パスバッファはパスを変更するメソッドも使える。
    let mut path_buf = PathBuf::from("src");
    path_buf.push("sample1.txt");
    println!("パスバッファにファイルを追加 {:?}", path_buf); //"src/sample1.txt"
    path_buf.set_file_name("path.txt");
    println!("追加したファイル名を変更 {:?}", path_buf); // "src/path.txt"
    path_buf.pop();
    println!("popで一番下の階層を取り除く {:?}", path_buf); // "src"

}

use std::os::unix::prelude::PermissionsExt;
fn file_system_access_functions() {
    println!("--------------- ファイルシステムアクセス関数");
    // ディレクトリを作って消す 存在するとこのままではpanic
    fs::create_dir("src/test1").unwrap();
    fs::create_dir_all("src/test2/test2-1/test2-1-1").unwrap();
    fs::remove_dir("src/test1").unwrap();
    fs::remove_dir_all("src/test2").unwrap();
    // ファイルの内容をコピー
    fs::copy("src/fs_access_sample1.txt", "src/fs_access_sample3.txt").unwrap();
    // renameで移動もできる
    fs::create_dir("src/test1").unwrap();
    fs::rename("src/fs_access_sample3.txt", "src/test1/moved.txt").unwrap();
    fs::remove_dir_all("src/test1").unwrap();

    // パーミション変更
    let mut permissions = fs::metadata("src/sample2.txt").unwrap().permissions(); // メタデータを取得
    permissions.set_mode(0o600); // 8進数なので0oをつける
    fs::set_permissions("src/sample2.txt", permissions).unwrap();
    /*
    $ ls -l src/sample2.txt
    -rw-r--r--  1 naoki  staff  15  8 12 14:18 src/sample2.txt  // 644
    $ cargo run --bin rust_lesson
    $ ls -l src/sample2.txt
    -rw-------  1 naoki  staff  15  8 12 14:23 src/sample2.txt  // 600に変わっている！

    */

}