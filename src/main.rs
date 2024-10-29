use scraper::{ElementRef, Html};
use std::fs::{self, File};
use std::io::{self, Read};
use toml::{map::Map, Value};

// HTML要素を再帰的にTOML形式へ変換する関数
fn element_to_toml(element: ElementRef) -> Option<Value> {
    let mut toml_map = Map::new();

    // 各属性をTOMLに追加
    for (attr_name, attr_value) in element.value().attrs() {
        toml_map.insert(attr_name.to_string(), Value::String(attr_value.to_string()));
    }

    // 子要素が存在するかを確認
    let mut has_child_elements = false;
    let mut text_content = String::new();

    for child in element.children() {
        if let Some(child_element) = ElementRef::wrap(child) {
            // 子要素があるのでテキストのみの追加はしない
            has_child_elements = true;
            let tag_name = child_element.value().name().to_string();

            if let Some(child_toml) = element_to_toml(child_element) {
                match toml_map.entry(tag_name) {
                    toml::map::Entry::Vacant(entry) => {
                        entry.insert(child_toml);
                    }
                    toml::map::Entry::Occupied(mut entry) => {
                        let existing = entry.get_mut();
                        if let Value::Array(arr) = existing {
                            arr.push(child_toml);
                        } else {
                            *existing = Value::Array(vec![existing.clone(), child_toml]);
                        }
                    }
                }
            }
        } else if let Some(text) = child.value().as_text() {
            text_content.push_str(text);
            text_content.push(' '); // 単語区切りのスペース
        }
    }

    // 子要素がない場合にのみテキストを追加
    if !has_child_elements && !text_content.trim().is_empty() {
        toml_map.insert(
            "text".to_string(),
            Value::String(text_content.trim().to_string()),
        );
    }

    if toml_map.is_empty() {
        None
    } else {
        Some(Value::Table(toml_map))
    }
}

// HTMLファイルを読み込み、TOML形式に変換する関数
fn html_to_toml(html_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    File::open(html_path)?.read_to_string(&mut contents)?;

    let document = Html::parse_document(&contents);
    let mut toml_map = Map::new();

    for element in document.tree.root().children().filter_map(ElementRef::wrap) {
        let tag_name = element.value().name().to_string();
        if let Some(toml_value) = element_to_toml(element) {
            toml_map.insert(tag_name, toml_value);
        }
    }

    let toml_value = Value::Table(toml_map);
    Ok(toml::to_string(&toml_value)?)
}

fn main() -> io::Result<()> {
    let html_path = "./res/index.html"; // 変換するHTMLファイルのパス

    match html_to_toml(html_path) {
        Ok(toml_output) => {
            println!("{}", toml_output); // 標準出力に表示
            fs::write("out.toml", toml_output)?; // TOMLデータをファイルとして保存
        }
        Err(e) => eprintln!("エラーが発生しました: {}", e),
    }

    Ok(())
}
