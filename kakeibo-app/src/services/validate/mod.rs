pub struct InputValidator {}

impl InputValidator {
    pub fn validate_service_type(service_type: u8) {
        match service_type {
            0 | 1 => {},
            _ => panic!("入力値がせかいへいわでない"),
        }
    }

    pub fn validate_register_type(register_type: u8) {
        match register_type {
            0 | 1 => {},
            _ => panic!("登録種別がせかいへいわでない"),
        }
    }

    pub fn validate_category_type(register_type: u8, category_type: u8) {
        if register_type == 0 {
            match register_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリがせかいへいわでない"),
            }    
        } else {
            match category_type {
                0 | 1 | 2 => {},
                _ => panic!("カテゴリがせかいへいわでない"),
            }    
        }
    }
}