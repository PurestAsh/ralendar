/// Trait for converting a string representation of a day number to Chinese numerals.
pub trait ChineseDayConverter {
    fn to_chinese_day(&self) -> String;
}

/// Converts a number (1-31) to Chinese numerals, e.g., 1 -> "初一", 12 -> "一二", 20 -> "二十".
fn num_to_chinese(n: u8) -> String {
    let digits = [
        "零", "一", "二", "三", "四", "五", "六", "七", "八", "九", "十",
    ];

    if n == 0 {
        return "零".to_string();
    }

    if n <= 10 {
        return format!("初{}", digits[n as usize]);
    }

    let tens = n / 10;
    let ones = n % 10;

    if ones == 0 {
        format!("{}十", digits[tens as usize])
    } else {
        format!("{}{}", digits[tens as usize], digits[ones as usize])
    }
}

impl ChineseDayConverter for String {
    fn to_chinese_day(&self) -> String {
        if let Ok(_n) = self.parse::<u8>() {
            num_to_chinese(self.parse::<u8>().unwrap())
        } else {
            self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_chinese_day() {
        assert_eq!("1".to_string().to_chinese_day(), "初一");
        assert_eq!("5".to_string().to_chinese_day(), "初五");
        assert_eq!("10".to_string().to_chinese_day(), "初十");
        assert_eq!("12".to_string().to_chinese_day(), "一二");
        assert_eq!("20".to_string().to_chinese_day(), "二十");
        assert_eq!("29".to_string().to_chinese_day(), "二九");
        assert_eq!("30".to_string().to_chinese_day(), "三十");
        assert_eq!("31".to_string().to_chinese_day(), "三一");
    }

    #[test]
    fn test_to_chinese_day_edge_cases() {
        assert_eq!("0".to_string().to_chinese_day(), "零");
        assert_eq!("11".to_string().to_chinese_day(), "一一");
        assert_eq!("21".to_string().to_chinese_day(), "二一");
        assert_eq!("40".to_string().to_chinese_day(), "四十");
    }

    #[test]
    fn test_to_chinese_day_invalid() {
        assert_eq!("invalid".to_string().to_chinese_day(), "invalid");
    }
}
