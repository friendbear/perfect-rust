use chrono::prelude::*;
pub fn fixed_date_time_type() -> DateTime<Utc> {
    let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(2023, 02, 02, 0, 0, 0).unwrap();
    date_time
}
#[cfg(test)]
mod date_time_type {
    use std::time::SystemTime;

    use chrono::prelude::*;
    use chrono_tz::Tz;

    #[test]
    fn instantiate() {
        let now = Utc::now();
        dbg!(now);

        let now_local = Local::now();
        dbg!(now_local);
    }

    #[test]
    fn format() {
        let date_time = super::fixed_date_time_type();
        let formatted = date_time.format("%Y/%m/%d %H:%M:%S").to_string();
        assert!("2023/02/02 00:00:00" == formatted);
    }

    #[test]
    fn from_string() {
        let rfc2822_type = DateTime::parse_from_rfc2822("Fri, 14 Jan 2022 10:52:37 +0900");
        let date_time = rfc2822_type.ok().unwrap();
        let r = date_time.format("%Y/%m/%d %H:%M:%S").to_string();
        assert!("2022/01/14 10:52:37" == r);

        let rfc3339_type = DateTime::parse_from_rfc3339("2022-01-14T12:00:00-08:00");
        let date_time = rfc3339_type.ok().unwrap();
        let r = date_time.format("%Y/%m/%d %H:%M:%S").to_string();
        assert!("2022/01/14 12:00:00" == r);

        assert_eq!(
            NaiveTime::parse_from_str("23:56:04", "%H:%M:%S"),
            Ok(NaiveTime::from_hms_opt(23, 56, 4).unwrap())
        );
        assert_eq!(
            NaiveDate::parse_from_str("2023:08:04", "%Y:%m:%d"),
            Ok(NaiveDate::from_ymd_opt(2023, 8, 4).unwrap())
        );
    }

    #[test]
    fn get() {
        let date_time = super::fixed_date_time_type();
        assert_eq!(date_time.year(), 2023);
        assert_eq!(date_time.month(), 2);
        assert_eq!(date_time.day(), 2);
        assert_eq!(date_time.hour(), 0);
        assert_eq!(date_time.minute(), 0);
        assert_eq!(date_time.second(), 0);
        assert_eq!(date_time.nanosecond(), 0);
        let weekday = match &date_time.weekday() {
            Weekday::Mon => "月",
            Weekday::Tue => "火",
            Weekday::Wed => "水",
            Weekday::Thu => "木",
            Weekday::Fri => "金",
            Weekday::Sat => "土",
            Weekday::Sun => "日",
        };
        dbg!(weekday);
    }

    #[test]
    fn change() {
        let date_time = super::fixed_date_time_type();
        assert_eq!(date_time.with_year(2024).unwrap().year(), 2024);
        assert_eq!(date_time.with_month(8).unwrap().month(), 8);
        assert_eq!(date_time.with_day(10).unwrap().day(), 10);
        assert_eq!(date_time.with_hour(12).unwrap().hour(), 12);
    }

    #[test]
    fn time_zone() {
        let tokyo = Local::now().with_timezone(&chrono_tz::Asia::Tokyo);
        let chicago = Local::now().with_timezone(&chrono_tz::America::Chicago);

        let tokyo_native = tokyo.naive_local();
        let chicago_native = chicago.naive_local();

        let duration: chrono::Duration = tokyo_native - chicago_native;

        println!("時間数 = {}", duration.num_hours());
        println!("秒数 = {}", duration.num_seconds());
        println!("ns数 = {}", duration.num_nanoseconds().unwrap());
    }

    #[test]
    fn unix_epoch() {
        let x = Local::now().timestamp();
        let y = Utc::now().timestamp();
        let z = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        assert!(x == y);
        println!("SystemTime epoch = {}", z.unwrap().as_secs());
    }
}
