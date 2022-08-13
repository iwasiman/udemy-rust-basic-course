use crate::services::io;
use crate::models::item::Item;

/// アイテムをJSON化して保存します。
pub fn store_item(item: Item, file_path: &str) {
    let mut data = io::read_data_or_create_new_data(file_path);
    data.push(item);
    io::write_to_json(&data, file_path);
}