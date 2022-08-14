// register の下にbuild_item, store_itemモジュールがあることの宣言
pub mod build_item;
pub mod store_item;

/// サービスの登録処理を行います。
pub fn run(file_path: &str) {
    println!("しゅうしのとうろくをするます");
    let item = build_item::build_item();
    println!("あいてむ {:?}", &item);

    store_item::store_item(item, file_path);
}
