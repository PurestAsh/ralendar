use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use rusqlite::types::{
    FromSql, FromSqlError, FromSqlResult, ValueRef,
};

/// 日程分类（日历），例如“我的日程”“中国节假日”
#[derive(Debug, Clone)]
pub struct Category {
    pub id: i64,
    pub name: String,
    /// 十六进制颜色，例如 "#4F8EF7"
    pub color: u32,
    /// 是否启用（SQLite 中存为 INTEGER 0/1）
    pub enabled: bool,
    /// 排序权重，越小越靠前
    pub sort_order: i64,
}

/// 日程详情，归属某一个分类
#[derive(Debug, Clone)]
pub struct Event {
    pub id: i64,
    /// 所属分类 ID，对应 categories.id
    pub category_id: i64,
    pub category_name: String,
    pub title: String,
    //时间受全天事件影响
    //如果是全天事件，则没有时分秒
    pub start_date: NaiveDate,
    pub start_time: Option<NaiveTime>,

    pub end_date: NaiveDate,
    pub end_time: Option<NaiveTime>,

    pub color: u32,
    //是否为全天事件 true:是 false:否
    pub all_day: bool,
    //目前3个值, 休,班,假。
    pub cal: Cal,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Cal {
    //休
    Rest,
    //班
    Work,
    //假
    Leave,
    //空白
    None,
}

impl From<i64> for Cal {
    fn from(value: i64) -> Self {
        match value {
            0 => Cal::Rest,
            1 => Cal::Work,
            2 => Cal::Leave,
            _ => Cal::None,
        }
    }
}

impl FromSql for Cal {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        if let ValueRef::Integer(v) = value {
            Ok(v.into())
        } else {
            Err(FromSqlError::InvalidType)
        }
    }
}
