use std::fmt::Debug;
use std::fmt::Display;

pub fn lesson() {
    println!("");
    println!("##### section9だよ トレイトとジェネリクス");

    /*
    derive属性による継承

    構造体の中身は単なるprintln! では出力されない。 deriveをつけるとよい。
    */
    let rect = Rectangle {
        height: 10.0,
        width: 20.0,
    };
    println!("中身が出るかな {:?}", rect); // Rectangle { width: 20.0, height: 10.0 }
    let rect2 = Rectangle {
        height: 10.0,
        width: 20.0,
    };
    println!("PartialEqを実装した構造体同士を比較 {:?}", rect == rect2); // true

    generics();
    generics_struct_enums();
}

pub trait Shape {
    fn calc_area(&self) -> f64;
    fn calc_perimeter(&self) -> f64;
    fn do_something();
    // デフォルト実装
    fn default_something(&self) -> &str {
        "これはトレイト自体にあるデフォルトメソッドだよ"
    }
}

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// 実装中もこの関数が足りないとかエラー表示してくれる。すごい。
impl Shape for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        self.width * 2.0 + self.height * 2.0
    }

    fn do_something() {
        println!("これはRectangleのなにかする関数だよ");
    }
    fn default_something(&self) -> &str {
        "これはトレイト自体にあるデフォルトメソッドをRectangleで上書きしたよ"
    }
}

pub struct Circle {
    pub radius: f64,
}
impl Shape for Circle {
    fn calc_area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }

    fn calc_perimeter(&self) -> f64 {
        self.radius * 2.0 * std::f64::consts::PI
    }

    fn do_something() {
        println!("これはCircleのなにかする関数だよ");
    }
}

// 引数shapeは、トレイトShapeを実装しているという制限がつけられる。OOPのポリモーフィズム
pub fn double_area(shape: &impl Shape) -> f64 {
    shape.calc_area() * 2.0
}

fn generics() {
    println!("ジェネリクスを使ったmax関数 {}", max(100, 999));
}

// ジェネリック境界、トレイト境界 と呼ぶ。任意の型 T は大小比較できるトレイトを実装していることを前提にする。
// 境界がシンプル：<T> の後に書いちゃう
// 境界が複数ある、複雑：whereのあとにかく　という使い分けが多い。
// fn max<T: PartialOrd>(a: T, b: T) -> T {
// }
fn max<T>(a: T, b: T) -> T
where
    T: PartialOrd
{
    if a >= b {
        a
    } else {
        b
    }
}

fn generics_struct_enums() {
    // ジェネリックな列挙型や構造体
    let p1 = Point {x: 2, y: 200};
    let p2 = Point {x: 2.5, y: 200.5};
    let p3 = Point {x: "2", y: "200"};
    println!("ジェネリックな列挙型や構造体 メソッドmax {}", p1.max());
    println!("ジェネリックな列挙型や構造体 メソッドmax {}", p2.max());
    println!("ジェネリックな列挙型や構造体 メソッドmax {}", p3.max());
    println!("ジェネリックな列挙型や構造体 メソッドprint_arg");
    p3.print_arg(true);
    println!("ジェネリックな列挙型や構造体 メソッドi32_hello {}", p1.i32_hello());


}

struct Point<T> {
    x: T,
    y: T,
}

// ジェネリック境界はimplのほうに書く
impl<T: PartialOrd + Debug> Point<T> {
    fn max(&self) -> &T {
        if self.x >= self.y {
            &self.x
        } else {
            &self.y
        }
    }

    fn print_arg<U: Display>(&self, val: U) {
        println!("別のジェネリクスを使ったprint_arg self.x: {:?}", self.x);
        println!("別のジェネリクスを使ったprint_arg val: {}", val);

    }
}

impl Point<i32> {
    fn i32_hello(&self) -> String {
        " i32を引数に取るPoint構造体専用 ".to_string()
    }
}