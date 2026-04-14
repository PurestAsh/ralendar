use std::fs;
use std::path::Path;

fn main() {
    let data_dir = Path::new("holiday-cn");
    let out_file = Path::new("src/holiday_cn.rs");

    let mut code = String::new();
    let mut names = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(data_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();

    entries.sort();

    for path in entries {
        if let Some(extension) = path.extension() {
            if extension == "json" {
                if let Some(name) = path.file_stem() {
                    if name == "" || name == "renovate" || name == "schema" {
                        continue;
                    }

                    let name = name.to_str().unwrap();
                    names.push(format!("YEAR_{}", name));
                    // 生成 A1, A2 ...
                    code.push_str(&format!(
                        "pub const YEAR_{}: &str = include_str!(\"../{}\");\n",
                        name,
                        path.display()
                    ));
                }
            }
        }
    }

    // 生成数组
    code.push_str("\npub const ALL_YEAR: &[&str] = &[\n");
    for name in &names {
        code.push_str(&format!("    {},\n", name));
    }
    code.push_str("];\n");

    fs::write(out_file, code).unwrap();
}
