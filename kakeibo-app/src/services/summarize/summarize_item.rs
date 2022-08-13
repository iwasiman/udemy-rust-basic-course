use std::{collections::{BTreeSet, BTreeMap}};
use chrono::{NaiveDate, Datelike};
use crate::services;
use crate::models::item::*; // このuse文で、Item構造体についてるメソッド new とかも使えるようになる。

/// 集計処理の実処理を実行します。
pub fn summarize(file_path: &str) {
    let data = services::io::read_all_data_or_panic(file_path);

    let target_dates: BTreeSet<NaiveDate> = get_target_dates(&data);
    let mut result_table: BTreeMap<NaiveDate, i32> = BTreeMap::new();
    for date in target_dates {
        let filtered_data = get_filtered_data(&data, date);
        let sum = summarize_data(&filtered_data);
        result_table.insert(date, sum);
    }
    print_table(result_table);
}

// 以下、内部メソッド

fn get_target_dates(data: &Vec<Item>) -> BTreeSet<NaiveDate> {
    let target_dates: BTreeSet<_> = data.iter().map(|item| {
        item.get_first_day()
    }).collect();
    target_dates
}

fn get_filtered_data(data: &Vec<Item>, filter_date: NaiveDate) -> Vec<&Item> {
    // i以下のようにフルパス的に書いてもエラーにならない。一部だけitem::Itemだとだめ。
    let filtered_data: Vec<&crate::models::item::Item> = data.iter().filter(|item| {
        item.get_year() == filter_date.year() && (item.get_month() == filter_date.month())
    }).collect();
    filtered_data
}
fn summarize_data(data: &Vec<&Item>) -> i32 {
    let mut sum = 0;
    for item in data {
        sum += item.get_price_for_summary();
    }
    sum
}

fn format_date(date: NaiveDate) -> String {
    format!("{}/{}", date.year(), date.month())
}

fn format_price(price: i32) -> String {
    if price > 0 {
        format!("+{}", price)
    } else {
        format!("{}", price)
    }
}

fn print_table(result_table: BTreeMap<NaiveDate, i32>) {
    for result in result_table {
        let date = format_date(result.0);
        let price = format_price(result.1);
        println!("{}のしゅうしは{}えんに なった", date, price);
    }
}

#[cfg(test)]
mod summarize_item_test {
    use crate::models::category::*;  //テスト中岳必要なので、use文はこの位置に必要。

    use super::*; // 親モジュールを全てインポート

    fn get_test_data() -> Vec<Item> {
        vec![
            Item::new (
                "旅行".to_string(),
                Category::Expense(ExpenseCategory::Food),
                5000,
                NaiveDate::from_ymd(2022, 8, 18),
            ),
            Item::new (
                "アマギフ券".to_string(),
                Category::Income(IncomeCategory::Bonus),
                4000,
                NaiveDate::from_ymd(2022, 8, 18),
            ),
        ]

    }

    #[test]
    fn test_get_target_dates() {
        let test_data = get_test_data();
        let mut expected: BTreeSet<NaiveDate> = BTreeSet::new();
        expected.insert(NaiveDate::from_ymd(2022, 8, 1));

        assert_eq!(get_target_dates(&test_data), expected);

    }
}