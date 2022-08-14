use crate::consts::register_type;
//use crate::consts::RegisterType;
use crate::consts::service_type;
pub struct InputValidator {}

impl InputValidator {
    pub fn new() -> Self {
        InputValidator {}
    }
    pub fn instance_method_like(&self, i: u32) {
        println!("## インスタンスメソッド的なやつ {}", i);
    }
}

// 以下、InputValidator構造体に紐付いていたが外だし
pub fn validate_service_type(service_type: u8) {
    match service_type {
        service_type::REGISTER | service_type::SUMMARIZE => {}
        _ => panic!("入力値がせかいへいわでない {}", service_type),
    }
}

pub fn validate_register_type(register_type: u8) {
    // let expense = RegisterType::Expense;
    // let income = RegisterType::Income;
    match register_type {
        register_type::INCOME | register_type::EXPENSE => {}
        _ => panic!("登録種別がせかいへいわでない {}", register_type),
    }
}

pub fn validate_category_type(register_type: u8, category_type: u8) {
    if register_type == 0 {
        match register_type {
            0 | 1 | 2 => {}
            _ => panic!("カテゴリがせかいへいわでない"),
        }
    } else {
        match category_type {
            0 | 1 | 2 => {}
            _ => panic!("カテゴリがせかいへいわでない"),
        }
    }
}
