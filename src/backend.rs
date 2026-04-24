use std::{collections::HashMap, fmt};

use time::{Date, Duration, Month, OffsetDateTime, UtcOffset};
use tyme4rs::tyme::Culture;
use tyme4rs::tyme::solar::SolarDay;

use crate::backend::holiday::Holidays;
use crate::util::ChineseDayConverter;
mod holiday;
pub struct Calendar {
    date_status: OffsetDateTime,
    holiday: Holidays,
    cache: HashMap<CacheKey, Vec<Day>>,
}

pub enum MonthOp {
    Pre,
    Current,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CacheKey {
    year: i32,
    month: Month,
}

impl CacheKey {
    fn new(year: i32, month: Month) -> Self {
        CacheKey { year, month }
    }
}

/// 表示一天的结构体，包含日期和当天的事件
/// 还包括农历
pub struct Day {
    date: Date,
    event: Vec<String>,
    lunar: String,
}
impl Day {
    pub fn day(&self) -> u8 {
        self.date.day()
    }
    pub fn event(&self) -> String {
        self.event.iter().fold(String::new(), |mut acc, x| {
            acc.insert_str(0, x);
            acc
        })
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
            date_status: OffsetDateTime::now_utc()
                .replace_offset(UtcOffset::from_hms(8, 0, 0).unwrap()),
            holiday: Holidays::new(),
            cache: HashMap::new(),
        }
    }

    pub fn get_month(&mut self, op: MonthOp) -> &Vec<Day> {
        match op {
            MonthOp::Current => self.get_or_insert_cache(self.date_status),
            MonthOp::Next => {
                let mut time = self.date_status;
                let mut month = time.month();
                let mut year = time.year();
                if month == Month::December {
                    year += 1;
                }
                month = month.next();

                time = time
                    .replace_day(1)
                    .unwrap()
                    .replace_year(year)
                    .unwrap()
                    .replace_month(Month::from(month))
                    .unwrap();
                self.date_status = time;

                self.get_or_insert_cache(time)
            }
            MonthOp::Pre => {
                let mut time = self.date_status;
                let mut month = time.month();
                let mut year = time.year();

                if month == Month::January {
                    year -= 1;
                }
                month = month.previous();

                time = time
                    .replace_day(1)
                    .unwrap()
                    .replace_year(year)
                    .unwrap()
                    .replace_month(month)
                    .unwrap();
                self.date_status = time;

                self.get_or_insert_cache(time)
            }
        }
    }

    fn month_first_week_day(time: OffsetDateTime) -> OffsetDateTime {
        let time = time.replace_day(1).unwrap();
        let week = time.weekday().number_from_monday() as u8;
        let weeek_offset = week - 1;
        time.checked_sub(Duration::days(weeek_offset as i64))
            .unwrap()
    }

    fn one_month_res(&self, first_day: OffsetDateTime) -> Vec<Day> {
        let mut res = Vec::new();
        for index in 0..42 {
            let time = first_day.checked_add(Duration::days(index)).unwrap();
            let date = time.date();
            let solar = SolarDay::from_ymd(
                date.year() as isize,
                date.month() as usize,
                date.day() as usize,
            );
            let mut event = Vec::new();
            solar.get_festival().map(|solar_festival| {
                event.push(solar_festival.get_name());
            });

            let lunar = solar.get_lunar_day();

            lunar.get_festival().map(|lunar_festival| {
                event.push(lunar_festival.get_name());
            });
            let lunar_str = lunar.get_day().to_string();
            let lunar_str = lunar_str.to_chinese_day();

            event.dedup();
            res.push(Day {
                date: date,
                event: event,
                lunar: lunar_str,
            });
        }
        res
    }

    fn get_holiday(&self, date: Date) -> Vec<String> {
        match self.holiday.get_holiday(date) {
            None => Vec::new(),
            Some(holiday) => holiday.name.split("、").map(|s| s.to_owned()).collect(),
        }
    }

    fn get_or_insert_cache(&mut self, mut time: OffsetDateTime) -> &Vec<Day> {
        let year = time.year();
        let month = time.month();
        let cache_kay = CacheKey::new(year, month);

        if !self.cache.contains_key(&cache_kay) {
            time = Self::month_first_week_day(time);
            let res = self.one_month_res(time);
            self.cache.insert(cache_kay, res);
        }

        self.cache.get(&cache_kay).unwrap()
    }

    pub fn get_cache(&self) -> Option<&Vec<Day>> {
        let year = self.date_status.year();
        let month = self.date_status.month();
        self.cache.get(&CacheKey::new(year, month))
    }
}

#[cfg(test)]
mod test {
    use crate::backend::{Calendar, MonthOp};

    #[test]
    fn test() {
        let mut calendar = Calendar::new();
        let v1 = calendar.get_month(MonthOp::Pre);
        for ele in v1 {
            println!("{ele}")
        }
        let v2 = calendar.get_month(MonthOp::Pre);
        println!("------");
        for ele in v2 {
            println!("{ele}")
        }
    }
}
