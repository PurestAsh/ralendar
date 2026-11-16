CREATE TABLE categories
(
  id          INTEGER                                        PRIMARY KEY,
  name        TEXT                                           NOT NULL UNIQUE,
  color       INTEGER                                        NOT NULL,           --颜色，采用16进制保存
  enabled     INTEGER                                        NOT NULL DEFAULT 1, --是否启用 1:启用，0不启用
  sort_order  INTEGER                                        NOT NULL DEFAULT 0,
  created_at  TEXT                                           NOT NULL DEFAULT (datetime('now', 'localtime'))
)



CREATE TABLE events
(
  id             INTEGER                                           PRIMARY KEY,
  category_id    INTEGER                                           NOT NULL,
  category_name  TEXT                                              NOT NULL,
  title          TEXT                                              NOT NULL,
  start_date     TEXT                                              NOT NULL,--例如: 1970-01-02
  start_time     TEXT                                              NULL,    --例如: 18:00:00
  end_date       TEXT                                              NOT NULL,
  end_time       TEXT                                              NULL,
  color          INTEGER                                           NOT NULL,--颜色，采用16进制保存
  all_day        INTEGER(1)                                        NOT NULL,--是否为全天事件,如果为全天事件，xxx_time为空
  cal            INTEGER                                           NOT NULL,--事件分类:0休，1工作日，2假，其他
  description    TEXT                                              NULL,    --详细描述
  created_at     TEXT                                              NOT NULL DEFAULT (datetime('now', 'localtime')),
  updated_at     TEXT                                              NOT NULL DEFAULT (datetime('now', 'localtime')),

  FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
)