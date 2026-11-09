mod cache;
mod model;
mod sqllite;
use cache::Cache;
use chrono::NaiveDate;
use gpui_kit::Global;
use gpui_kit::component::date_picker::DateTime;
pub use model::Cal;
pub use model::Category;
pub use model::Event;
pub use sqllite::CalendarDb;

pub struct Backend {
    cache: Cache,
    db: CalendarDb,
}

impl Global for Backend {}

impl Backend {
    pub fn new() -> Self {
        Self {
            cache: Cache::new(),
            db: CalendarDb::new(),
        }
    }

    /// 查询分类：优先读缓存，未命中则从数据库读取并写入缓存
    pub fn category(&mut self, id: i64) -> &Category {
        // 先确保缓存里有：没有就查库并写入。
        // 这一步的可变借用必须在这里结束，后面才能再借出引用。
        if self.cache.get_category(id).is_none() {
            let cat = self.db.get_category(id);
            self.cache.add_category(cat);
        }

        // 之后不再修改缓存，因此可以安全地借出指向缓存内部的引用。
        self.cache.get_category(id).unwrap()
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
        calendar: String,
        title: String,
        all_day: bool,
        start_time: DateTime,
        end_time: DateTime,
        description: String,
    ) {
        println!("可开始");
        if let DateTime::Single(Some(start_t)) = start_time {
            println!("进入");
            if let DateTime::Single(Some(end_t)) = end_time {
                println!("净土2");
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
                        &title,
                        start_date,
                        start_time,
                        end_date,
                        end_time,
                        0x42accf,
                        1,
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
}
