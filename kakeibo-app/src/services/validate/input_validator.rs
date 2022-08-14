use crate::consts::REGISTER_TYPE_EXPENSE;
use crate::consts::REGISTER_TYPE_INCOME;
use crate::consts::SERVICE_TYPE_REGISTER;
use crate::consts::SERVICE_TYPE_SUMMARIZE;
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
        SERVICE_TYPE_REGISTER | SERVICE_TYPE_SUMMARIZE => {}
        _ => panic!("入力値がせかいへいわでない {}", service_type),
    }
}

pub fn validate_register_type(register_type: u8) {
    match register_type {
        REGISTER_TYPE_INCOME | REGISTER_TYPE_EXPENSE => {}
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
