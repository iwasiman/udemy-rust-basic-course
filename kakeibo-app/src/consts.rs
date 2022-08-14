pub mod register_type;
pub mod service_type;

pub const FILE_PATH: &str = "store/data.json";

// TODO: 列挙型でうまくできないか？ consts::service_type::REGISTER みたいにサブモジュールにするしかなかった。
// 列挙型にしてみたが、match式でうまく使えない。
pub enum RegisterType {
    Income,
    Expense,
}

impl RegisterType {
    pub fn get_value(&self) -> u8 {
        match *self {
            RegisterType::Expense => 0,
            RegisterType::Income => 1,
        }
    }
}
