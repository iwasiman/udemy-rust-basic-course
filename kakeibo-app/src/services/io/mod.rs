use crate::models;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


pub fn read_data_or_create_new_data(file_path: &str) -> Vec<models::Item> {
    let the_file = File::open(file_path);
    match the_file {
        Ok(f) => {
            let buf_reader = BufReader::new(f);
            serde_json::from_reader(buf_reader).expect("デシリアライズに任務失敗！")
        },
        Err(_) => {
            println!("新規ファイルを作成するます");
            Vec::new()
        }
    }

}

pub fn read_data_or_panic(file_path: &str) -> Vec<models::Item> {
    let the_file = File::open(file_path).expect("ファイルオープンに失敗！直ちに連絡を！");
    let buf_reader = BufReader::new(the_file);
    let data: Vec<_> = serde_json::from_reader(buf_reader).expect("デシリアライズ失敗！続行不可能！");

    if data.len() == 0 {
        panic!("データがないあるます");
    }
    data
}

pub fn write_to_json(data: &Vec<models::Item>, file_path: &str) {
    let json_data = serde_json::to_string_pretty(data).expect("JSONへのシリアライズ失敗！脱出を！");
    let mut file = File::create(file_path).expect("書き込みファイルのオープンに失敗！脱出を！");
    writeln!(file, "{}", json_data).expect("ファイルへの書き込みに失敗！脱出を！");
    println!("こうもくのとうろく みっしょんこんぷりーと！");
}