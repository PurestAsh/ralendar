use std::path::{Path, PathBuf};

use chrono::{NaiveDate, NaiveTime};
use rusqlite::{Connection, OptionalExtension, Result, params};

use crate::backend::{
    Cal,
    model::{Category, Event},
};

const DEV_DB_PATH: &str = "./ralendar.db";
const PROD_DB_PATH: &str = "/usr/share/ralendar/ralendar.db";
const ENV: &str = "RALENDAR_ENV";
pub struct CalendarDb {
    conn: Connection,
}

impl CalendarDb {
    /// 打开数据库；首次运行时自动创建目录和数据库文件
    pub fn new() -> Self {
        let db_path = if std::env::var(ENV).is_ok() {
            DEV_DB_PATH
        } else {
            PROD_DB_PATH
        };

        let conn = Connection::open(db_path).unwrap();
        // 打开外键约束（SQLite 默认关闭）
        conn.pragma_update(None, "foreign_keys", "ON").unwrap();

        let db = Self { conn };
        db.init().unwrap();
        db
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

    /// 查询单个分类的名字
    pub fn get_cats_name(&self) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT 
                        name
                 FROM categories
                 order by sort_order",
            )
            .unwrap();

        let mut rows = stmt.query(params![]).unwrap();

        let mut res = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            res.push(row.get(0).unwrap())
        }
        res
    }

    pub fn get_categories(&self) -> Vec<Category> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT 
                        id,
                        name,
                        color,
                        enabled,
                        sort_order
                 FROM categories
                 order by sort_order",
            )
            .unwrap();

        let mut rows = stmt.query(params![]).unwrap();

        let mut res = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            res.push(map_category(&row).unwrap());
        }
        res
    }

    // ==================== 日程 ====================

    /// 新增日程，返回新日程的 ID
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
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO events(
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
                    description)
             VALUES (?1, ?2, ?3, ?4, ?5,?6,?7,?8,?9,?10,?11)",
            params![
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
                description
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// 查询与某个日期区间有交集的全部日程。
    /// `start` 包含，`end` 不包含。
    pub fn get_events_between(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Vec<Event> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT
                        id,
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
                        description
                    FROM events
                    WHERE start_date < ?2
                    AND end_date >= ?1
                    ORDER BY start_date",
            )
            .unwrap();

        let mut rows = stmt.query(params![start, end]).unwrap();

        let mut res = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            res.push(map_event(row).unwrap())
        }
        res
    }

    //根据日期获取经日的事件
    pub fn get_event(&self, date: NaiveDate) -> Vec<Event> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT 
                        id,
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
                        description
                    FROM events
                    WHERE  start_date <= ?1
                    AND end_date >= ?1",
            )
            .unwrap();

        let mut rows = stmt.query(params![date]).unwrap();

        let mut res = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            res.push(map_event(row).unwrap())
        }
        res
    }

    /// 根据日期和日程 ID 查询单个日程：
    /// 只要该日期落在 [start_date, end_date] 区间内即算命中；
    pub fn get_event_by_date(
        &self,
        id: i64,
        date: NaiveDate,
    ) -> Option<Event> {
        self.conn
            .query_row(
                "SELECT 
                        id,
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
                        description
                    FROM events
                    WHERE category_id = ?1
                    AND start_date <= ?2
                    AND end_date >= ?2",
                params![id, date],
                map_event,
            )
            .optional()
            .unwrap()
    }
}

/// 把一行查询结果映射成 Event
fn map_event(row: &rusqlite::Row<'_>) -> Result<Event> {
    Ok(Event {
        id: row.get_unwrap(0),
        category_id: row.get_unwrap(1),
        category_name: row.get_unwrap(2),
        title: row.get_unwrap(3),
        start_date: row.get_unwrap(4),
        start_time: row.get(5).ok(), //可为空
        end_date: row.get_unwrap(6),
        end_time: row.get(7).ok(), //可为空
        color: row.get_unwrap(8),
        all_day: row.get_unwrap(9),
        cal: row.get_unwrap(10),
        description: row.get_unwrap(11),
    })
}

fn map_category(row: &rusqlite::Row<'_>) -> Result<Category> {
    Ok(Category {
        id: row.get_unwrap(0),
        name: row.get_unwrap(1),
        color: row.get_unwrap(2),
        enabled: row.get_unwrap(3),
        sort_order: row.get_unwrap(4),
    })
}
