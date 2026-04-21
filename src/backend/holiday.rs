use std::collections::HashMap;

use serde_json::Value;
use time::{Date, PrimitiveDateTime, format_description, macros::format_description};

use crate::holiday_cn::ALL_YEAR;

#[derive(Debug)]
pub struct Holiday {
    pub name: String,
    pub date: Date,
    pub is_off_day: bool,
}

pub struct Holidays {
    holiday: HashMap<Date, Holiday>,
}

impl Holidays {
    pub fn new() -> Self {
        let mut holidays = Holidays {
            holiday: HashMap::new(),
        };

        for &year in ALL_YEAR {
            let json: Value = serde_json::from_str(year).unwrap();
            let json = &json["days"];
            for day in json.as_array().unwrap() {
                let day_json = day.as_object().unwrap();

                let name = day_json.get("name").unwrap().as_str().unwrap().to_owned();

                let date = day_json.get("date").unwrap().as_str().unwrap();

                let format = format_description::parse("[year]-[month]-[day]").unwrap();

                let date = Date::parse(date, &format).unwrap();

                let is_off_day = day_json.get("isOffDay").unwrap().as_bool().unwrap();

                let holiday = Holiday {
                    name: name,
                    date: date,
                    is_off_day: is_off_day,
                };

                holidays.holiday.insert(date, holiday);
            }
        }

        holidays
    }

    pub fn get_holiday(&self, date: Date) -> Option<&Holiday> {
        self.holiday.get(&date)
    }
}

#[cfg(test)]
mod test {
    use crate::backend::holiday::Holidays;

    #[test]
    fn test() {
        let holidays = Holidays::new();
        for (date, holiday) in holidays.holiday {
            println!("date:{},holiday:{:?}", date, holiday);
        }
    }
}
