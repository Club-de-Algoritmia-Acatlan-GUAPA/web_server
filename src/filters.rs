pub mod filters {
    use askama_markdown_cmark::filters::markdown_cmark;
    use chrono::{MappedLocalTime, TimeZone, Utc};
    // This filter does not have extra arguments
    pub fn millis_to_utc<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        match Utc.timestamp_millis_opt(s.to_string().parse::<i64>().unwrap()) {
            MappedLocalTime::Single(dt) => Ok(dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            _ => Ok("Incorrect timestamp_millis".to_string()),
        }
    }

    pub fn int_to_minutes<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let time = s.to_string().parse::<u32>().unwrap();
        let hours = time / 60;
        let minutes = time % 60;
        Ok(format!("{:02}:{:02}", hours, minutes))
    }
    pub fn markdown<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
        let old = s.to_string();
        match markdown_cmark(&s) {
            Ok(marked) => Ok(marked),
            Err(_) => Ok(old.to_string()),
        }
    }
}
