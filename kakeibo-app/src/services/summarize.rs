mod summarize_item;

/// サービスの集計処理を行います。
pub fn run(file_path: &str) {
    println!("かけいぼのしゅうけいをおこなうます");
    summarize_item::summarize(file_path);
}
