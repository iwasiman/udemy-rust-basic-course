use std::{io};
use kakeibo_app::services;

const FILE_PATH: &str = "store/data.json";
fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力するます (0：登録 1：集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("1 か 2 をいれるます");
    // 入力値のバリデーション
    services::validate::InputValidator::validate_service_type(service_type);

    if service_type == 0 {
        println!("登録サービス");
        services::register::run(FILE_PATH);
    } else if service_type == 1 {
        println!("エレガントォォォに集計サービス");
        services::summarize::run(FILE_PATH);

    }
}
