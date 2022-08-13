// ルート/models/{ファイル名がモジュール}.js で、これだけサブモジュールがあるよというのを宣言
pub mod category;
pub mod item;
// モジュール名とファイル名が同一である必要がある。ローワースネークケース推奨。
// 構造体はアッパーキャメルなので、item.rsにItem構造体＋実装したメソッド、関連する処理を配置して use crate::models::item::Item; するのがよさそう。

// このファイルの中に関数や構造体などなどを書くと、それはmodelsモジュール内に存在と扱われる。
// しかし数が多くなってくるとわかりにくいからmod.rsに全部書くのは見通しが悪そう。
pub fn models_belongs_func() {
    println!("## モジュール models に属する関数の実行");
}

/*
ルート/models.rs
ルート/models/mod.rs
に同じ内容を書くと完全に等価。片方のみが必要で、両方存在するとambiguous (あいまい) だとコンパイルエラー。
しかし ルート/models/mod.rs に書くのはRust 2015 Editionの遺産だそうな。

*/
