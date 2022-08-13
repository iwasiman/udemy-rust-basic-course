// 下が、servicesモジュールの下にディレクトリを切ってこれだけサブモジュールがありますよ宣言
pub mod validate;
pub mod io;
pub mod register;
pub mod summarize;

// ディレクトリ内にmod.rsを置くのは古い。
// https://doc.rust-jp.rs/edition-guide/rust-2018/path-changes.html