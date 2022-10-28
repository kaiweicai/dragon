use chrono::{Datelike, Local, DateTime};

/// gen today date string
/// 10-21-2022
pub fn today_date_str(now:&DateTime<Local>) -> String {
    format!("{:02}-{:02}-{}", now.month(), now.day(), now.year())
}

/// 2022-10-21
pub fn today_dragon_str(now:&DateTime<Local>) -> String {
    format!("{}-{:02}-{:02}", now.year(), now.month(), now.day())
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use crate::utils::date_utils::{today_date_str, today_dragon_str};

    #[test]
    fn test_today_date_str() {
        let today = Local::now();
        let today_date_str = today_date_str(&today);
        assert_eq!(today_date_str, "10-22-2022");
    }

    #[test]
    fn test_today_dragon_str() {
        let today = Local::now();
        let today_dragon_str = today_dragon_str(&today);
        println!("today_dragon_str is:{:?}",today_dragon_str);
        // assert_eq!(today_date_str, "10-22-2022");
    }
}
