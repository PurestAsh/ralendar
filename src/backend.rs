mod model;
mod sqllite;

use chrono::NaiveDate;
use chrono::NaiveTime;
use gpui_kit::Global;
use gpui_kit::component::date_picker::DateTime;
pub use model::Cal;
pub use model::Category;
pub use model::Event;
pub use sqllite::CalendarDb;

pub struct Backend {
    db: CalendarDb,
}

impl Global for Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {
            db: CalendarDb::new(),
        }
    }

    ///获取所有日历
    pub fn get_cats_name(&self) -> Vec<String> {
        self.db.get_cats_name()
    }

    pub fn get_categories(&self) -> Vec<Category> {
        self.db.get_categories()
    }

    /// 根据日期和日程 ID 查询日程，转发到数据库层
    pub fn get_event_by_date(
        &self,
        id: i64,
        date: NaiveDate,
    ) -> Option<Event> {
        self.db.get_event_by_date(id, date)
    }

    pub fn get_event(&self, date: NaiveDate) -> Vec<Event> {
        self.db.get_event(date)
    }

    /// 查询与日期区间有交集的全部日程（start 包含，end 不包含）。
    pub fn get_events_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Vec<Event> {
        self.db.get_events_between(start, end)
    }

    pub fn save_event(
        &self,
        calendar_index: u32,
        calendar: &str,
        title: &str,
        all_day: bool,
        start_time: DateTime,
        end_time: DateTime,
        description: &str,
        color: u32,
    ) {
        if let DateTime::Single(Some(start_t)) = start_time {
            if let DateTime::Single(Some(end_t)) = end_time {
                let start_date = start_t.date();
                let end_date = end_t.date();
                let start_time;
                let end_time;
                if all_day {
                    start_time = None;
                    end_time = None;
                } else {
                    start_time = Some(start_t.time());
                    end_time = Some(end_t.time());
                }

                self.db
                    .add_event(
                        calendar_index,
                        calendar,
                        title,
                        start_date,
                        start_time,
                        end_date,
                        end_time,
                        color,
                        all_day,
                        Cal::None,
                        if description.is_empty() {
                            None
                        } else {
                            Some(&description)
                        },
                    )
                    .unwrap();
            }
        }
    }

    ///用来给数据导入工具使用的。
    pub fn add_event(
        &self,
        category_id: u32,
        category_name: &str,
        title: &str,
        start_date: NaiveDate,
        start_time: Option<NaiveTime>,
        end_date: NaiveDate,
        end_time: Option<NaiveTime>,
        color: u32,
        all_day: bool,
        cal: Cal,
        description: Option<&str>,
    ) {
        self.db
            .add_event(
                category_id,
                category_name,
                title,
                start_date,
                start_time,
                end_date,
                end_time,
                color,
                all_day,
                cal,
                description,
            )
            .unwrap();
    }
}
