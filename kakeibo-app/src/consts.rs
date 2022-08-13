pub const FILE_PATH: &str = "store/data.json";
// TODO: 列挙型でうまくできないか？ consts::service_type::REGISTER みたいにサブモジュールするしかない？
pub const SERVICE_TYPE_REGISTER: u8 = 0;
pub const SERVICE_TYPE_SUMMARIZE: u8 = 1;
pub const REGISTER_TYPE_INCOME: u8 = 0;
pub const REGISTER_TYPE_EXPENSE: u8 = 1;

pub enum RegisterType {
    Income,
    Expense,
}
