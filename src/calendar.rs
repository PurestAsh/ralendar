use std::fmt;

use chrono::{DateTime, Datelike, Days, Local, Months};
use tyme4rs::tyme::Culture;
use tyme4rs::tyme::solar::SolarDay;

use crate::util::ChineseDayConverter;
pub struct Calendar {
    today: DateTime<Local>,
}

/// 表示一天的结构体，包含日期和当天的事件
/// 还包括农历
pub struct Day {
    date: DateTime<Local>,
    event: Vec<String>,
    lunar: String,
}
impl Day {
    pub fn day(&self) -> u32 {
        self.date.day()
    }
    pub fn event(&self) -> &Vec<String>{
        &self.event
    }

    pub fn lunar(&self) -> &String {
        &self.lunar
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.date,
            self.event.iter().fold(String::new(), |mut a, x| {
                a.insert_str(a.len(), x);
                a
            }),
            self.lunar
        )
    }
}

impl Calendar {
    pub fn new() -> Self {
        Self {
            today: Local::now(),
        }
    }

    pub fn get_month(&mut self, month_offset: i32) -> Vec<Day> {
        let today = self.today;
        let mut time = today.with_day(1).unwrap();
        if month_offset >= 0 {
            time = time
                .checked_add_months(Months::new(month_offset as u32))
                .unwrap()
        } else {
            time = time
                .checked_sub_months(Months::new((-month_offset) as u32))
                .unwrap()
        }

        let week = time.weekday().number_from_monday();
        let weeek_offset = week - 1;
        time=time.checked_sub_days(Days::new(weeek_offset.into()))
            .unwrap();
        self.page_remain_days(time)
    }

    fn page_remain_days(&self, first_day: DateTime<Local>) -> Vec<Day> {
        let mut res = Vec::new();
        for index in 0..42 {
            let time = first_day.checked_add_days(Days::new(index)).unwrap();
            let solar = SolarDay::from_ymd(
                time.year() as isize,
                time.month() as usize,
                time.day() as usize,
            );
            let mut event = Vec::new();
            //法定节假日
            solar.get_legal_holiday().map(|holiday|{
                event.push(holiday.get_name());
            });

            //公历现代节日
            solar.get_festival().map(|solar_festival| {
                event.push(solar_festival.get_name());
            });

            let lunar = solar.get_lunar_day();
            //农历节日
            lunar.get_festival().map(|lunar_festival| {
                event.push(lunar_festival.get_name());
            });
            let lunar_str = lunar.get_day().to_string();
            let lunar_str = lunar_str.to_chinese_day();
            //去重
            event.dedup();
            res.push(Day {
                date: time,
                event: event,
                lunar: lunar_str,
            });
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::calendar::Calendar;

    #[test]
    fn test_current_month() {
        let mut calendar = Calendar::new();
        let v1 = calendar.get_month(0);
        for ele in v1 {
            println!("{ele}")
        }
    }

    #[test]
    fn test_next_month() {
        let mut calendar = Calendar::new();
        let v1 = calendar.get_month(1);
        for ele in v1 {
            println!("{ele}")
        }
    }

    #[test]
    fn test_last_month() {
        let mut calendar = Calendar::new();
        let v1 = calendar.get_month(-1);
        for ele in v1 {
            println!("{ele}")
        }
    }
}
