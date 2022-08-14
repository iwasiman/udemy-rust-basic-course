use kakeibo_app::services;
use std::io;
//mod services;
//mod services::validate; //できない
use kakeibo_app::models;
use services::validate::do_simple_validation;
//mod models;

use kakeibo_app::consts::FILE_PATH;

fn main() {
    let mut service_type = String::new();
    println!("実行したい内容を入力するます (0：登録 1：集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("0 か 1 をいれるます");
    // 入力値のバリデーション
    // services::validate までがモジュール階層。InputValidatorが構造体で、validate_service_typeが紐付いたメソッド。構造体のメソッドも::で出すのがわかりにくい。
    //services::validate::InputValidator::validate_service_type(service_type);
    services::validate::input_validator::validate_service_type(service_type);
    // 構造体のインスタンスメソッドは、一文では使えない。
    let val = services::validate::input_validator::InputValidator::new();
    val.instance_method_like(200); // 構造体のインスタンスを作った後は、実装したメソッドの中で引数に&selfがあるやつだけ使える

    // main.rsではlib.rsでモジュール指定していれば use kakeibo_app::models; か use kakeibo_app::models::models_belongs_func; がいる。
    // モジュール指定していなかったら mod models;
    models::models_belongs_func();

    // 先頭のmod で mod services::validate; のように階層指定は使えない。
    // use kakeibo_app::services; か mod services; したあとなら...
    services::validate::do_simple_validation(); // 構造体でなければ、モジュール名::モジュール名::関数名でいける。
                                                // use services::validate::do_simple_validation; した後なら...
    do_simple_validation(); // use文は階層がいくらでも書ける。

    if service_type == 0 {
        println!("登録サービス");
        services::register::run(FILE_PATH);
    } else if service_type == 1 {
        println!("エレガントォォォに集計サービスを実行する！");
        services::summarize::run(FILE_PATH);
    }
}

/*
A. ルート/lib.rs に書いた場合
  pub mod services;
  pub mod models;

ルート/main.rs
use kakeibo_app::services;
use kakeibo_app::models::models_belongs_func;
でも
mod services;
mod models;
も一応とおる。

ルート/services/io/mod.rs
use crate::models::item::Item

B. ルート/lib.rs に書かない場合はクレートと認識されない？
  //pub mod services;
  //pub mod models;

ルート/main.rs
mod services;
mod models; //models::models_belongs_funcは書けない。mod .. はあくまでモジュールを指定する構文だから。

ルート/services/io/mod.rs
use crate::models::item::Item //同じだった。

lib.rs に書いてライブラリ指定、
main.rs でmod xx指定はしないのが正解の模様。mod xx はモジュールがある宣言だから。

*/
