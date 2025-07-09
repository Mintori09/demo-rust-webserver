use serde_json::{Value, json};
use validator::ValidationErrors;

pub fn validation_errors_to_json(errors: &ValidationErrors) -> Value {
    let mut map = serde_json::Map::new();

    for (field, field_errors) in errors.field_errors() {
        // Nếu có nhiều lỗi, bạn có thể map toàn bộ thành array
        let messages: Vec<String> = field_errors
            .iter()
            .filter_map(|err| err.message.as_ref())
            .map(|msg| msg.to_string())
            .collect();

        // Nếu chỉ lấy lỗi đầu tiên (phổ biến hơn)
        if let Some(first) = messages.first() {
            map.insert(field.to_string(), json!(first));
        }
    }

    json!({ "errors": map })
}
