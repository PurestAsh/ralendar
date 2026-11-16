use std::{fs, path::Path};

use chrono::NaiveDate;
use ralendar::{Backend, Cal};
use serde_json::Value;

fn main() {
    let db = Backend::new();
    let path = Path::new("holiday-cn");
    assert!(path.exists());
    import_batch(path, &db);
}

fn import_batch(path: &Path, db: &Backend) {
    let dir = fs::read_dir(path).unwrap();
    for ele in dir {
        let entry = ele.unwrap();

        let entry_path = entry.path();
        if entry_path.is_dir() {
            import_batch(&entry_path, db);
        } else {
            if entry_path
                .extension()
                .is_some_and(|ext| ext.eq("json"))
            {
                import_one_year(&entry_path, db);
            }
        }
    }
}

fn import_one_year(path: &Path, db: &Backend) {
    let content = fs::read_to_string(path).unwrap();
    let value: Value = serde_json::from_str(&content).unwrap();
    let Some(days) = value["days"].as_array() else {
        return;
    };
    for entry in days {
        let events_name =
            entry.get("name").unwrap().as_str().unwrap();
        let date = entry.get("date").unwrap().as_str().unwrap();
        let start_date =
            NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
        let off_day =
            entry.get("isOffDay").unwrap().as_bool().unwrap();
        let cal = if off_day { Cal::Rest } else { Cal::Work };
        db.add_event(
            1,
            "中国节假日",
            events_name,
            start_date,
            None,
            start_date,
            None,
            0xE88A80FF,
            true,
            cal,
            None,
        );
    }
}
