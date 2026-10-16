mod cache;
mod model;
mod sqllite;
use cache::Cache;
use chrono::NaiveDate;
pub use model::Cal;
pub use model::Category;
pub use model::Event;
pub use sqllite::CalendarDb;

pub struct Backend {
    cache: Cache,
    db: CalendarDb,
}

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
}
