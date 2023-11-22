/* This file is part of Combodate: <https://github.com/christopherphan/combodate_rust>
 *
 * Copyright 2023 Christopher Phan <cphan@chrisphan.com>
 *
 * Available under an MIT license. See LICENSE.TXT in repository root directory for more
 * information.
 */

use chrono::{DateTime, Datelike, Local, SecondsFormat, TimeZone, Timelike, Utc};

use std::fmt::Display;

pub fn run() {
    let t = Local::now();
    print!("{}", make_combodate_table(t));
}

fn make_combodate_table(t: DateTime<Local>) -> String {
    let tu = t.with_timezone(&Utc);
    let rows = [
        ("Unix", &unix_time(t)[..]),
        (
            "ISO-8601 Gregorian (Local)",
            &(t.to_rfc3339_opts(SecondsFormat::Secs, false)),
        ),
        (
            "ISO-8601 Gregorian (UTC)",
            &(tu.to_rfc3339_opts(SecondsFormat::Secs, false)),
        ),
        ("ISO-8601 Week-date (Local)", &isoweekday(t)[..]),
        ("ISO-8601 Week-date (UTC)", &isoweekday(tu)[..]),
        ("Proportion of day elapsed (Local)", &proportion_day(t)[..]),
        (
            "Proportion of week elapsed (Local)",
            &proportion_week(t)[..],
        ),
        (
            "Proportion of month elapsed (Local)",
            &proportion_month(t)[..],
        ),
        (
            "Proportion of year elapsed (Local)",
            &proportion_year(t)[..],
        ),
    ];
    make_table(&rows)
}

fn pad(x: &str, k: usize, left: bool, pad_char: char) -> String {
    if x.len() < k {
        let mut pad_char_str = String::from("");
        pad_char_str.push(pad_char);
        let pad_str = pad_char_str.repeat(k - x.len());
        if left {
            format!("{}{}", &pad_str[..], x)
        } else {
            format!("{}{}", x, &pad_str[..])
        }
    } else {
        String::from(x)
    }
}

fn make_table(rows: &[(&str, &str)]) -> String {
    let mut max_len: (usize, usize) = (0, 0);
    for k in rows {
        if k.0.len() > max_len.0 {
            max_len.0 = k.0.len();
        }
        if k.1.len() > max_len.1 {
            max_len.1 = k.1.len();
        }
    }
    let mut out_str = String::from("");
    for k in rows {
        out_str.push_str(&format!(
            "{} {}\n",
            &pad(k.0, max_len.0, false, ' ')[..],
            &pad(k.1, max_len.1, true, ' ')[..]
        ));
    }
    out_str
}

fn reverse_str(x: &str) -> String {
    let mut get_from = String::from(x);
    let mut out_str = String::from("");
    let mut n = get_from.pop();
    while n.is_some() {
        out_str.push(n.unwrap());
        n = get_from.pop();
    }
    out_str
}

fn separate(x: &str, places: u8, sep: char) -> String {
    let ell = x.len();
    let rem = ell % (places as usize);
    let parts = ell / (places as usize);
    let mut out_str = String::from(if rem > 0 { &x[0..rem] } else { "" });
    if rem > 0 {
        out_str.push(sep);
    }
    for k in 0..parts {
        out_str.push_str(&x[(rem + k * (places as usize))..(rem + (k + 1) * (places as usize))]);
        if k < parts - 1 {
            out_str.push(sep);
        }
    }
    out_str
}

fn separate_from_left(x: &str, places: u8, sep: char) -> String {
    reverse_str(&separate(&reverse_str(x)[..], places, sep)[..])
}

fn isoweekday<Tz: TimeZone>(x: DateTime<Tz>) -> String
where
    Tz::Offset: Display,
{
    x.format("%G-W%V-%uT%H:%M:%S%:z").to_string()
}

fn unix_time<Tz: TimeZone>(x: DateTime<Tz>) -> String {
    let s = format!("{}", x.timestamp());
    separate(&s, 3, ' ')
}

fn proportion_day<Tz: TimeZone>(x: DateTime<Tz>) -> String {
    let s = x.naive_local().num_seconds_from_midnight();
    let p = ((s as u128) * 100_000) / 86_400;
    format!("0.{}", separate_from_left(&format!("{:05}", p), 3, ' '))
}

fn proportion_week<Tz: TimeZone>(x: DateTime<Tz>) -> String {
    let s =
        x.naive_local().num_seconds_from_midnight() + x.weekday().num_days_from_monday() * 86_400;
    let p = ((s as u128) * 100_000) / (7 * 86_400);
    format!("0.{}", separate_from_left(&format!("{:05}", p), 3, ' '))
}

fn month_length(year: i32, month: u32) -> u32 {
    if month > 12 {
        panic!("Invalid month");
    }
    if (month % 2 == 1 && month <= 7) || (month % 2 == 0 && month > 7) {
        31
    } else if month == 2 {
        if is_leap_year(year) {
            29
        } else {
            28
        }
    } else {
        30
    }
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn proportion_month<Tz: TimeZone>(x: DateTime<Tz>) -> String {
    let s = x.naive_local().num_seconds_from_midnight() + x.day0() * 86_400;
    let p = ((s as u128) * 100_000) / ((month_length(x.year(), x.month()) as u128) * 86_400);
    format!("0.{}", separate_from_left(&format!("{:05}", p), 3, ' '))
}

fn year_length(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

fn proportion_year<Tz: TimeZone>(x: DateTime<Tz>) -> String {
    let s = x.naive_local().num_seconds_from_midnight() + x.ordinal0() * 86_400;
    let p = ((s as u128) * 100_000) / ((year_length(x.year()) as u128) * 86_400);
    format!("0.{}", separate_from_left(&format!("{:05}", p), 3, ' '))
}

/* TESTS */

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;

    #[test]
    fn sep_test_1() {
        assert_eq!(
            separate("1234567890", 3, ' '),
            String::from("1 234 567 890")
        );
    }

    #[test]
    fn sep_test_2() {
        let x = 123456789;
        assert_eq!(
            separate(&format!("{}", x), 3, ','),
            String::from("123,456,789")
        );
    }

    #[test]
    fn unix_test_1() {
        let x = Local.timestamp_opt(1679866623, 0).unwrap(); // is a valid Unix time
        assert_eq!(unix_time(x), "1 679 866 623");
    }

    #[test]
    fn unix_test_2() {
        let x = Utc.timestamp_opt(626651100, 0).unwrap(); // is a valid Unix time
        assert_eq!(unix_time(x), "626 651 100");
    }

    #[test]
    fn pad_test_left_1() {
        assert_eq!(pad("hhhh", 7, true, 'o'), String::from("ooohhhh"));
    }
    #[test]
    fn pad_test_left_2() {
        assert_eq!(pad("hhhh", 3, true, 'o'), String::from("hhhh"));
    }

    #[test]
    fn pad_test_right_1() {
        assert_eq!(pad("ooo", 7, false, 'h'), String::from("ooohhhh"));
    }
    #[test]
    fn pad_test_right_2() {
        assert_eq!(pad("hhhh", 3, false, 'o'), String::from("hhhh"));
    }

    #[test]
    fn table_test() {
        let w = [
            ("spaces!", "places"),
            ("vast", "space"),
            ("to", "be"),
            ("padded", "with"),
            ("spaces", "!"),
        ];
        assert_eq!(
            make_table(&w),
            String::from(
                "\
spaces! places
vast     space
to          be
padded    with
spaces       !
"
            )
        );
    }

    #[test]
    fn isoweekday_test_1() {
        let test_date = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(1989, 11, 9, 22, 45, 0)
            .unwrap();

        assert_eq!(
            isoweekday(test_date),
            String::from("1989-W45-4T22:45:00+01:00")
        );
    }

    #[test]
    fn isoweekday_test_2() {
        let test_date = Utc.with_ymd_and_hms(1989, 11, 9, 21, 45, 0).unwrap();

        assert_eq!(
            isoweekday(test_date),
            String::from("1989-W45-4T21:45:00+00:00")
        );
    }

    #[test]
    fn reverse_str_test_1() {
        assert_eq!(reverse_str("Koszulity"), String::from("ytiluzsoK"));
    }

    #[test]
    fn sep_from_left_test_1() {
        assert_eq!(
            separate_from_left("aaabbbcccdd", 3, 'x'),
            String::from("aaaxbbbxcccxdd")
        );
    }

    #[test]
    fn sep_from_left_test_2() {
        assert_eq!(
            separate_from_left("aaabbbcccddd", 3, 'x'),
            String::from("aaaxbbbxcccxddd")
        );
    }
    #[test]
    fn prop_day_test_1() {
        let test_date = FixedOffset::east_opt(-8 * 3600)
            .unwrap()
            .with_ymd_and_hms(1989, 11, 9, 12, 0, 0)
            .unwrap();

        assert_eq!(proportion_day(test_date), String::from("0.500 00"));
    }

    #[test]
    fn prop_week_test_1() {
        let test_date = FixedOffset::east_opt(-9 * 3600)
            .unwrap()
            .with_ymd_and_hms(1995, 7, 13, 12, 0, 0)
            .unwrap();

        assert_eq!(proportion_week(test_date), String::from("0.500 00"));
    }

    #[test]
    fn leap_year_test_1() {
        assert!(is_leap_year(2004));
    }

    #[test]
    fn leap_year_test_2() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn leap_year_test_3() {
        assert!(is_leap_year(1600));
    }

    #[test]
    fn leap_year_test_4() {
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn leap_year_test_5() {
        assert!(!is_leap_year(1997));
    }

    #[test]
    fn month_len_test_1() {
        assert_eq!(month_length(2004, 2), 29);
    }

    #[test]
    fn mon_len_test_2() {
        assert_eq!(month_length(2000, 2), 29);
    }

    #[test]
    fn mon_len_test_3() {
        assert_eq!(month_length(1600, 2), 29);
    }

    #[test]
    fn mon_len_test_4() {
        assert_eq!(month_length(2100, 2), 28);
    }

    #[test]
    fn mon_len_test_5() {
        assert_eq!(month_length(1997, 2), 28);
    }

    #[test]
    fn mon_len_test_6() {
        assert_eq!(month_length(2014, 3), 31);
    }

    #[test]
    fn mon_len_test_7() {
        assert_eq!(month_length(2000, 8), 31);
    }

    #[test]
    fn mon_len_test_8() {
        assert_eq!(month_length(2045, 12), 31);
    }

    #[test]
    fn mon_len_test_9() {
        assert_eq!(month_length(2014, 4), 30);
    }

    #[test]
    fn mon_len_test_10() {
        assert_eq!(month_length(2000, 6), 30);
    }

    #[test]
    fn mon_len_test_11() {
        assert_eq!(month_length(2045, 11), 30);
    }

    #[test]
    fn prop_month_test_1() {
        let test_date = FixedOffset::east_opt(-9 * 3600)
            .unwrap()
            .with_ymd_and_hms(1995, 2, 8, 0, 0, 0)
            .unwrap();

        assert_eq!(proportion_month(test_date), String::from("0.250 00"));
    }

    #[test]
    fn year_length_test_1() {
        assert_eq!(year_length(1996), 366);
    }

    #[test]
    fn year_length_test_2() {
        assert_eq!(year_length(2000), 366);
    }

    #[test]
    fn year_length_test_3() {
        assert_eq!(year_length(1600), 366);
    }

    #[test]
    fn year_length_test_4() {
        assert_eq!(year_length(2100), 365);
    }

    #[test]
    fn year_length_test_5() {
        assert_eq!(year_length(2013), 365);
    }

    #[test]
    fn prop_year_test_1() {
        let test_date = Utc.with_ymd_and_hms(1989, 4, 2, 6, 0, 0).unwrap();
        assert_eq!(proportion_year(test_date), "0.250 00");
    }

    #[test]
    fn prop_year_test_2() {
        let test_date = Utc.with_ymd_and_hms(2000, 4, 1, 12, 0, 0).unwrap();
        assert_eq!(proportion_year(test_date), "0.250 00");
    }
}
