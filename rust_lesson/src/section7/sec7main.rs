pub fn lesson() {
    println!("");
    println!("##### section7だよ");

    structs();
    enums();
    options();
    match_guards();
}

fn structs() {
    println!("--------------- 構造体");
    // 構造体 作ると「インスタンス」と呼ぶ。
    let rectangle: Rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    let height = 5;
    let rectangle2: Rectangle = Rectangle { width: 10, height };
    println!("構造体 {} {}", rectangle.width, rectangle.height);
    println!("構造体 {} {}", rectangle.width, rectangle.height);
    println!("構造体のメソッドareaを呼ぶ {}", rectangle2.area()); // 第1引数のselfは不要。

    let rec_new = Rectangle::new(10, 20); // コンストラクタから生成
    println!(
        "コンストラクタから作った構造体 {} {}",
        rec_new.width, rec_new.height
    );
}

// 先頭大文字のアッパーキャメルが通例。
struct Rectangle {
    width: u32, // フィールドはスネークケース。
    height: u32,
}

// 構造体の外側に書くのが他の言語のクラスと違う。
impl Rectangle {
    // 第1引数がselfなのが通常の関数と違う。型名は省略できる。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 型関連関数
    // 型自体に関連、構造体をインスタンス化しなくても使える。staticメソッドみたいなやつ。 コンストラクタによく使われる。
    // 第一引数にselfがなかったら型関連関数。
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn enums() {
    println!("--------------- 列挙型");
    // 列挙型
    let c = Shape::Circle; //Shape型として推論してくれる。
    let s = Shape::Square(1); //タプルを指定
    let t = Shape::Triangle {
        base: 10,
        height: 50,
    }; // 構造体で指定
    c.sample_method(); //どのバリアントからでもメソッドは使える
    s.sample_method();
    t.sample_method();
    println!(" フィールドの中身 {:?}", t);
}

// 列挙型の中のひとつひとつをバリアントと呼ぶ
#[derive(Debug)]
enum Shape {
    Circle,
    Square(u32),                         //タプル型バリアント
    Triangle { base: u32, height: u32 }, //構造体バリアント {}で囲うのに注意
}

impl Shape {
    fn sample_method(&self) {
        println!("Shape列挙型のサンプルメソッド");
        match *self {
            Shape::Circle => println!("まるだよ"),
            Shape::Square(value) => println!("四角だよ {}", value),
            Shape::Triangle { base, height } => println!("三角だよ {} {}", base, height),
        }
    }
}

fn options() {
    println!("--------------- Option型");

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let a = Some(1); // Option型だけはよく使うので、Option::Someと書かなくて良い。
    let b = Option::Some("str"); // いちおう書ける 中はジェネリック型で何でも入れられる
    let c: Option<i32> = None; // Noneの時だけはジェネリックが指定できないので明示的に書く。
    println!("{:?} {:?} {:?}", a, b, c);

    let v = vec![1, 2, 3];
    let val = v.get(2);
    match val {
        Some(x) => println!("vの2番めの値はあるよ {}", x), //とれるからこっちが実行
        None => println!("値がないよ"),
    }

    // 全分岐を書かなくても良い。Noneの分岐がなかったら単にスキップ。
    if let Some(x) = val {
        println!("vの値はあるよ {}", x);
    }
}

fn match_guards() {
    println!("--------------- マッチガード");
    let v = vec![1, 2, 3];

    for i in [0, 1, 2, 3, 4, 5] {
        let value = v.get(i);
        match value {
            Some(1) => println!("値は1だよ"), // i=0
            // 下がマッチガード
            Some(x) if *x == 2 => println!("マッチガードで判定、値は2だよ"), // i= 1
            Some(x) => println!("値はあるよ {}", x),                                  // i= 2
            None => println!("値がないよ"), // i = 3,4,5のとき
        }
    }

    // if let では使用できない。
}
