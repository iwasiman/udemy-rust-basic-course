use crate::models::item::Item;
use crate::services;
use crate::services::validate::input_validator;
use chrono::NaiveDate;
use std::io;
use std::str::FromStr;

/// 標準入力からアイテム構造体を組み立てます。
pub fn build_item() -> Item {
    let register_type = input_register_type();
    let name = input_name();
    let category_type = input_category_type(register_type);
    let price = input_price();
    let date = input_date();
    let category = Item::get_category(register_type, category_type);

    Item::new(name, category, price, date)
}

// 以下、内部メソッド

fn input_register_type() -> u8 {
    println!("登録種別を入力するといいぜ (0:収入 1:支出)");
    let mut register_type = String::new();
    io::stdin()
        .read_line(&mut register_type)
        .expect("登録種別の入力に失敗しちまったぜ");
    let register_type = register_type.trim().parse().expect("0か1で頼むぜ");
    services::validate::input_validator::validate_register_type(register_type);
    register_type
}

fn input_name() -> String {
    println!("品目名をにゅうりょくするます");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("品目名にしっぱい");
    name.trim().to_string()
}

fn input_category_type(register_type: u8) -> u8 {
    println!("カテゴリーをいれるます");
    if register_type == 0 {
        println!("(0: きゅうりょう 1: ぼーなすぽいんと 2: そのた)");
    } else {
        println!("(0: たべもの 1: すきなこと 2: そのた)");
    }
    let mut categpry_type = String::new();
    io::stdin()
        .read_line(&mut categpry_type)
        .expect("カテゴリ種別の入力がしっぱい");
    let category_type: u8 = categpry_type
        .trim()
        .parse()
        .expect("カテゴリはすうじをいれるます");
    //services::validate::inputValidator::validate_category_type(register_type, category_type); // 講座の元のやりかた
    //services::validate::input_validator::validate_category_type(register_type, category_type); // フルパスで書いても同じ
    input_validator::validate_category_type(register_type, category_type); //use文でモジュール名まで指定だとモジュール名::関数名になる
    category_type
}

fn input_price() -> u32 {
    println!("おかねをいれるます");
    let mut price = String::new();
    io::stdin().read_line(&mut price).expect("おかねにしっぱい");
    price.trim().parse().expect("おかねはすうじをいれるます")
}

fn input_date() -> NaiveDate {
    println!("ひづけをいれるます (yyyy-mm-dd)");
    let mut date = String::new();
    io::stdin().read_line(&mut date).unwrap();
    NaiveDate::from_str(&date).expect("ひづけは yyyy-mm-dd の形式でにゅうりょくするます")
}
