use std::path::Path;

use chrono::NaiveDate;
use rusqlite::{Connection, OptionalExtension, Result, params};
use xdg::BaseDirectories;

use crate::backend::model::{Category, Event};

pub struct CalendarDb {
    conn: Connection,
}

impl CalendarDb {
    /// 打开数据库；首次运行时自动创建目录和数据库文件
    pub fn new() -> Self {
        let xdg_dirs = BaseDirectories::with_prefix("ralendar");
        let data_home =
            xdg_dirs.get_data_home().expect("无法获取用户数据目录");

        if !data_home.exists() {
            std::fs::create_dir_all(&data_home)
                .expect("无法创建用户数据目录");
        }

        Self::open(data_home.join("calendar.db")).unwrap()
    }

    /// 打开指定路径的数据库，并初始化表结构
    fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        // 打开外键约束（SQLite 默认关闭）
        conn.pragma_update(None, "foreign_keys", "ON")?;

        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    /// 初始化表结构，不会重复创建已有的表
    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            -- 日程分类表：保存“我有哪些日程分类”
            CREATE TABLE IF NOT EXISTS categories (
                id         INTEGER PRIMARY KEY,
                name       TEXT    NOT NULL UNIQUE,
                color      INTEGER NOT NULL,
                enabled    INTEGER NOT NULL DEFAULT 1,
                sort_order INTEGER NOT NULL DEFAULT 0,
                created_at TEXT    NOT NULL DEFAULT (datetime('now', 'localtime'))
            );

            -- 日程详情表：保存每个分类下的具体日程
            CREATE TABLE IF NOT EXISTS events (
                id            INTEGER PRIMARY KEY,
                category_id   INTEGER NOT NULL,
                category_name TEXT    NOT NULL,
                title         TEXT    NOT NULL,
                start_time    TEXT    NOT NULL,
                end_time      TEXT    NOT NULL,
                color         INTEGER NOT NULL,
                description   TEXT,
                created_at    TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
                updated_at    TEXT    NOT NULL DEFAULT (datetime('now', 'localtime')),
                FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS idx_events_start_time
            ON events(start_time);

            CREATE INDEX IF NOT EXISTS idx_events_category_id
            ON events(category_id);
            ",
        )
    }

    // ==================== 分类 ====================

    /// 新增分类，返回新分类的 ID
    pub fn add_category(
        &self,
        name: &str,
        color: u32,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO categories (name, color, sort_order)
             VALUES (?1, ?2,
                     COALESCE((SELECT MAX(sort_order) + 1 FROM categories), 0))",
            params![name, color],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 查询所有分类，按 sort_order 排序
    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, color, enabled, sort_order
             FROM categories
             ORDER BY sort_order, id",
        )?;

        let rows = stmt.query_map([], map_category)?;

        rows.collect()
    }

    /// 查询单个分类
    pub fn get_category(&self, id: i64) -> Category {
        self.conn
            .query_row(
                "SELECT id, name, color, enabled, sort_order
                 FROM categories WHERE id = ?1",
                params![id],
                map_category,
            )
            .unwrap()
    }

    /// 修改分类的名称、颜色与启用状态
    pub fn update_category(
        &self,
        id: i64,
        name: &str,
        color: u32,
        enabled: bool,
    ) -> Result<bool> {
        let changed = self.conn.execute(
            "UPDATE categories
             SET name = ?1, color = ?2, enabled = ?3
             WHERE id = ?4",
            params![name, color, enabled, id],
        )?;

        Ok(changed > 0)
    }

    /// 只切换分类的启用状态
    pub fn set_category_enabled(
        &self,
        id: i64,
        enabled: bool,
    ) -> Result<bool> {
        let changed = self.conn.execute(
            "UPDATE categories SET enabled = ?1 WHERE id = ?2",
            params![enabled, id],
        )?;

        Ok(changed > 0)
    }

    /// 删除分类，同时级联删除该分类下的所有日程
    pub fn delete_category(&self, id: i64) -> Result<bool> {
        let deleted = self.conn.execute(
            "DELETE FROM categories WHERE id = ?1",
            params![id],
        )?;

        Ok(deleted > 0)
    }

    // ==================== 日程 ====================

    /// 新增日程，返回新日程的 ID
    pub fn add_event(
        &self,
        category_id: i64,
        title: &str,
        start_time: NaiveDate,
        end_time: Option<NaiveDate>,
        description: Option<&str>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO events
                (category_id, title, start_time, end_time, description)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![category_id, title, start_time, end_time, description],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 查询一个月的所有日程
    pub fn get_month_events(
        &self,
        month_start: NaiveDate,
        next_month_start: NaiveDate,
    ) -> Result<Vec<Event>> {
        self.query_events(
            "SELECT id, category_id, title, start_time, end_time, description
             FROM events
             WHERE start_time >= ?1
               AND start_time < ?2
             ORDER BY start_time",
            params![month_start, next_month_start],
        )
    }

    /// 查询某个月、某个分类下的日程
    pub fn get_month_events_by_category(
        &self,
        category_id: i64,
        month_start: NaiveDate,
        next_month_start: NaiveDate,
    ) -> Result<Vec<Event>> {
        self.query_events(
            "SELECT id, category_id, title, start_time, end_time, description
             FROM events
             WHERE category_id = ?1
               AND start_time >= ?2
               AND start_time < ?3
             ORDER BY start_time",
            params![category_id, month_start, next_month_start],
        )
    }

    /// 查询某个分类下的全部日程
    pub fn get_events_by_category(
        &self,
        category_id: i64,
    ) -> Result<Vec<Event>> {
        self.query_events(
            "SELECT id, category_id, title, start_time, end_time, description
             FROM events
             WHERE category_id = ?1
             ORDER BY start_time",
            params![category_id],
        )
    }

    //根据日期获取经日的事件
    pub fn get_event(&self, date: NaiveDate) -> Vec<Event> {
        let mut stmt= self.conn.prepare("SELECT id, category_id, title, start_time, end_time, color, description
                 FROM events
                 WHERE  start_time <= ?1
                   AND end_time >= ?1").unwrap();

        let mut rows = stmt.query(params![date]).unwrap();

        let mut res = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            res.push(Event {
                id: row.get(0).unwrap(),
                category_id: row.get(1).unwrap(),
                category_name: row.get(2).unwrap(),
                title: row.get(3).unwrap(),
                start_time: row.get(4).unwrap(),
                end_time: row.get(5).unwrap(),
                color: row.get(6).unwrap(),
                cal: row.get(7).unwrap(),
                description: row.get(8).unwrap(),
            })
        }
        res
    }

    /// 根据日期和日程 ID 查询单个日程：
    /// 只要该日期落在 [start_time, end_time] 区间内即算命中；
    /// end_time 为 NULL 时视为只持续 start_time 当天。
    pub fn get_event_by_date(
        &self,
        id: i64,
        date: NaiveDate,
    ) -> Option<Event> {
        self.conn
            .query_row(
                "SELECT id, category_id, title, start_time, end_time, color, description
                 FROM events
                 WHERE id = ?1
                   AND start_time <= ?2
                   AND end_time >= ?2",
                params![id, date],
                map_event,
            )
            .optional().unwrap()
    }

    /// 修改日程
    pub fn update_event(
        &self,
        id: i64,
        category_id: i64,
        title: &str,
        start_time: NaiveDate,
        end_time: Option<NaiveDate>,
        description: Option<&str>,
    ) -> Result<bool> {
        let changed = self.conn.execute(
            "UPDATE events
             SET category_id = ?1,
                 title       = ?2,
                 start_time  = ?3,
                 end_time    = ?4,
                 description = ?5,
                 updated_at  = datetime('now', 'localtime')
             WHERE id = ?6",
            params![
                category_id,
                title,
                start_time,
                end_time,
                description,
                id
            ],
        )?;

        Ok(changed > 0)
    }

    /// 删除日程
    pub fn delete_event(&self, id: i64) -> Result<bool> {
        let deleted = self.conn.execute(
            "DELETE FROM events WHERE id = ?1",
            params![id],
        )?;

        Ok(deleted > 0)
    }

    /// 执行查询并把结果行映射成 Event 列表
    fn query_events<P: rusqlite::Params>(
        &self,
        sql: &str,
        params: P,
    ) -> Result<Vec<Event>> {
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(params, map_event)?;
        rows.collect()
    }
}

/// 把一行查询结果映射成 Category
fn map_category(row: &rusqlite::Row<'_>) -> Result<Category> {
    Ok(Category {
        id: row.get(0)?,
        name: row.get(1)?,
        color: row.get(2)?,
        enabled: row.get(3)?,
        sort_order: row.get(4)?,
    })
}

/// 把一行查询结果映射成 Event
fn map_event(row: &rusqlite::Row<'_>) -> Result<Event> {
    Ok(Event {
        id: row.get(0).unwrap(),
        category_id: row.get(1).unwrap(),
        category_name: row.get(2).unwrap(),
        title: row.get(3).unwrap(),
        start_time: row.get(4).unwrap(),
        end_time: row.get(5).unwrap(),
        color: row.get(6).unwrap(),
        cal: row.get(7).unwrap(),
        description: row.get(8).unwrap(),
    })
}
