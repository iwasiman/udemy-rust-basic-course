use std::ops::RangeInclusive;
mod section6; // 実体はルート/section6.rs

fn main() {
    println!("Hello, world!");
    print!("Hello, ");
    print!("Rust!");
    println!("Hello, {}", "students");

    // コメント
    /* 複数行こめんと
     */

    let a: i32 = 1;
    println!("{}", a);
    //a = 2;
    //let mut b: i32 = 2;
    //b = 4; // 使ってないのd絵警告
    //let d: i32; // 途端に警告
    const B: i32 = 2; // 関数の内側でも宣言できる constは型名の宣言が必須
    println!("const Bは {}", B);

    let float1:f64 = 3.142;
    let float2:f64 = float1; // 型の指定は推論にまかせて書かなくても良いし、書いても良い。VSCodeが出してるのを消そうとして：が消えたりするのに注意。
    println!("float型の変数 {} {}", float1, float2);


    numeric();
    tupple();
    array();
    vector();
    string_type();
    functions();
    blocks();
    ifs();
    matchs();
    loops();

    section6::sec6main::lesson();
}

//const B: i32 = 2; // 関数の外側で宣言できる
//let c = 1; //できない

fn numeric() {
    // 数値型
    // iが符号付き、uが符号なし、fが浮動小数点数 +数字。　isizeというのも。

    let a: i32 = 1; //これらがデフォルト
    let b: f64 = 2.0;

    let c: u16 = 3;
    let d: f32 = 4.0f32;
    // 四則演算は同じ型同士のみ。

    let f: f64 = 1 as f64 + 2.0; //as でキャスト

    // 論理型
    let g: bool = false;
    let h: bool = 1 == 1;
    println!("{}{}{}{}{}{}{}", a, b, c, d, f, g, h);
}

fn tupple() {
    // タプル
    let t1: (i32, bool, f64) = (1, true, 2.0);
    let t2: (f64, i32, bool) = (2.0, 1, true);

    // タプルを表示するにはデバッグフォーマットがいる。
    println!("{:?}", t1); // (1, true, 2.0)

    let i: f64 = t2.0; // 小数点でなく、t2の1番めの要素
    println!("{}", i); // 2．0
    let (x, y, _) = t2;
    println!("{}{}", x, y); // 21

    //let u: () = (); // ユニット型。何も返さない関数の戻り値などに。
    //println!("u: {}", u);
}

fn array() {
    // 配列
    let l1: [i32; 3] = [1, 2, 3]; // 型宣言のあとはセミコロンなので注意。3は要素の数
    let l2: [i32; 10] = [0; 10]; // 10こに全部0を入れる
    println!("配列{:?}", l1);
    println!("配列{:?}", l2);

    let i: i32 = l1[0];
    let [x, y, z] = l1;
    println!("配列の中身を取り出す {} {}　{}　{}", i, x, y, z); // 1 1 2 3
    let l3: &[i32] = &l1[0..2];
    println!("スライス {:?}", l3); //[1, 2]
    let l4: &[i32] = &l1[0..=2];
    println!("スライス {:?}", l4); //[1, 2, 3]
    let l5: &[i32] = &l1[..];
    println!("スライス全部 {:?}", l5); //[1, 2, 3]
}

fn vector() {
    // ベクタ型
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = vec![0; 10];
    let mut v3: Vec<i32> = Vec::new();
    v3.push(1);
    v3.push(100);
    println!("ベクタ {:?}", v1);
    println!("ベクタ {:?}", v2);
    println!("ベクタ {:?}", v3);

    let x: Option<i32> = v3.pop();
    println!("最後の値を取得 {:?}", x); // Some(100) Rustにはnullがない！
    println!("取られたあとのベクタ {:?}", v3); //[1]

    let y: i32 = v3[0]; // ここで[1]番目を取ると存在しないのでpanic
    let z: Option<&i32> = v3.get(100); //100番目をとる ここのindex: はVSCodeが表示しているだけで実際は存在しない
    println!("番号指定で取得 {:?}", y); // 1
    println!("存在しない100番目を取得 {:?}", z); // None
}

// ここでstringType という関数名にするとコンパイラが警告を出してくれる。
fn string_type() {
    // 文字型
    let c1: char = 'a';
    let c2: char = '@';
    let c3: char = '😇';
    println!("char型 {}{}{}", c1, c2, c3);

    // 文字列型
    let s1: &str = "Rust"; //文字列スライスは変更不可
                           //s1 = s1 + "plus"; エラーたくさん
    println!("文字列スライス型 {}", s1);

    // String型
    let s2: String = String::from("Python");
    let s3: String = "Java".to_string();
    let mut s4: String = String::from("Hello!");
    s4.push_str(", Rust world"); // この行のstring: はVSCodeが出してるだけで存在足内
    println!("文字列型 {}{}{}", s2, s3, s4);
    println!("文字列型 {}", s4 + " and friend is Golang"); // 左辺String、右辺文字列スライスなら結合できる

    // 下のようにformatでも結合できる。ちなみにここでs4を使おうとすると所有権が移動したあとで借用したとエラーになる。
    let s5: String = format!("{}{}", s3, " and otherlanguages");
    println!("{}", s5); // Python and other languages
}

fn functions() {
    // 関数 ローワースネークケース。

    println!("関数コール {}", add(100, 200));
    let c = add(200, 500); // 書かなくても c: i32になる。背景色が変わるのでVSCodeが表示している。
    println!("関数コール {}", c);
    let say_hello_result:() = say_hello(); // 型推論でユニット型が表示。 変数名もスネークケースでないと警告してくれる。
    println!("関数コール {:?}", say_hello_result); // ()
}

fn add(a: i32, b: i32) -> i32 {
    a + b // 関数の最後でセミコロンなしで書くとreturn文扱い。こちらが一般的。
}
fn say_hello() {
    println!("Hi!")
}

/*
式と文が別。Rustは式ベース。
式はセミコロンがない。
*/

fn blocks() {
    {
        println!("ブロックの中だよ");
        let x = 1;
        println!("xは {}", x);
    }
    //println!("{}", x); //ブロックの外では使えない。

    // シャドーイング
    let y = 10;
    println!("yを宣言し10 {}", y);
    {
        let y = 5;
        println!("yをブロック内で再度宣言し5を代入 {}", y);
    }
    println!("ブロックの外に戻るとyは... {}", y); // 10に戻る
    let xxx = {
        100;
    };
    println!("xxxは {:?}", xxx); // ()になる

}

fn ifs() {
    // IFは文でなく式。
    let x = 5;
    // ifの条件部分は常に論理型必須。0とか空文字とかは入らない。
    if x> 0 {
        println!("xは0より大きい");
    }
    // 式なのでこうも書ける
    let y = if x > 0 {
        x
    } else {
        //"0" // コンパイルエラー
        0
    };
    println!("y= {:?}", y); //5

}

fn matchs() {
    //マッチ式
    let x = 0;
    match x {
        0 => println!("ゼロだよ"), // ひとつだったらカンマ
        1 => {
            println!("いちだよ");
            println!("もういっちょ"); // 複数あったらブロックで囲ってセミコロン
        },
        // ここでmatch式を終わらせるとi32の範囲を網羅していないのでエラー。すごい。
        //non-exhaustive patterns: `i32::MIN..=-1_i32` and `2_i32..=i32::MAX` not covered the matched value is of type `i32`
        _ => println!("そのた"),
    }
}

fn loops() {
    // loop
    let mut cnt = 0;
    loop {
        println!("無限ハロー！ Ctr+C必須!と見せかけてbreak");
        if cnt == 5 {
            break;
        }
        cnt += 1;
    }

    // while ならより簡潔
    let mut cnt2 = 0;
    while cnt2 <= 5 {
        println!("今度はwhile");
        cnt2 += 1;
    }

    for i in [1,2,3] {
        println!("今度はfor {}", i); // 3回出力
    }
    let r: RangeInclusive<i32> = 1..=10;
    for x in r { // ここでx：i32と書くとコンパイルエラー
        println!("for in で自乗 {}", x * x); // 10回出力
    }

}
