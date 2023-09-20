extern crate time;
extern crate time_macros;

mod start;
mod end;

pub use time::OffsetDateTime;
pub use time_macros::datetime;
pub use start::Start;
pub use end::End;

type Tm = time::OffsetDateTime;

static mut TEST_NOW: Option<DateTime> = None;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct DateTime {
    tm: Tm,
}

pub trait CarbonDuration<'d> {
    fn year(&self) -> DateTime;

    fn month(&self) -> DateTime;

    fn day(&self) -> DateTime;

    fn hour(&self) -> DateTime;

    fn minute(&self) -> DateTime;

    fn second(&self) -> DateTime;
}

impl DateTime {
    pub fn now() -> DateTime {
        match DateTime::get_test_now() {
            Some(date_time) => date_time,
            None => DateTime { tm: time::OffsetDateTime::now_utc() }
        }
    }

    pub fn create_from_tm(tm: Tm) -> DateTime {
        DateTime {
            tm: tm,
        }
    }

    pub fn set_test_now(test_now: Option<DateTime>) {
        unsafe {
            TEST_NOW = test_now
        }
    }

    pub fn get_test_now() -> Option<DateTime> {
        unsafe {
            TEST_NOW.clone()
        }
    }

    pub fn start_of(&self) -> Start {
        Start::new(&self)
    }

    pub fn end_of(&self) -> End {
        End::new(&self)
    }

    pub fn is_leap_year(&self) -> bool {
        let year = 1900 + self.tm.year();
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
    }

    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            2 => { if self.is_leap_year() { 29 } else { 28 } },
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
}

// https://en.wikipedia.org/wiki/Zeller's_congruence#Implementation_in_software
// fn zeller_congruence(y: f32, m: f32, d: f32) -> i32 {
//     let mut yy = 1900.0 + y;
//     let mut mm = 1.0 + m;
//     if mm == 1.0 || mm == 2.0 {
//         yy -= 1.0;
//         mm += 12.0;
//     }

//     let x = ((d + (26.0 * (mm + 1.0) / 10.0).floor() + yy + (yy / 4.0).floor() + (6.0 * (yy / 100.0).floor()) + (yy / 400.0).floor()) % 7.0) as i32;
//     match x - 1 {
//         -1 => 6,
//         _  => x - 1,
//     }
// }

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    extern crate time;
    use super::*;
    use time::Duration;
    use time_macros::offset;
    use std::ops::{Add, Sub};

    struct ExampleTms {
        january: Tm,
        february: Tm,
        march: Tm,
        april: Tm,
        may: Tm,
        june: Tm,
        july: Tm,
        august: Tm,
        september: Tm,
        october: Tm,
        november: Tm,
        december: Tm,
    }

    fn start_tms() -> ExampleTms {
        ExampleTms {
            january: datetime!(2015-01-01 0:00).assume_offset(offset!(UTC)),
            february: datetime!(2015-02-01 0:00).assume_offset(offset!(UTC)),
            march: datetime!(2015-03-01 0:00).assume_offset(offset!(UTC)),
            april: datetime!(2015-04-01 0:00).assume_offset(offset!(UTC)),
            may: datetime!(2015-05-01 0:00).assume_offset(offset!(UTC)),
            june: datetime!(2015-06-01 0:00).assume_offset(offset!(UTC)),
            july: datetime!(2015-07-01 0:00).assume_offset(offset!(UTC)),
            august: datetime!(2015-08-01 0:00).assume_offset(offset!(UTC)),
            september: datetime!(2015-09-01 0:00).assume_offset(offset!(UTC)),
            october: datetime!(2015-10-01 0:00).assume_offset(offset!(UTC)),
            november: datetime!(2015-11-01 0:00).assume_offset(offset!(UTC)),
            december: datetime!(2015-12-01 0:00).assume_offset(offset!(UTC)),
            // january:   Tm { date: Date::from_iso_week_date(115, 1, 31), time: Time{ }, offset: Offset{} },
            // february:  Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115, tm_wday: 0, tm_yday: 31, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // march:     Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 2, tm_year: 115, tm_wday: 0, tm_yday: 59, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // april:     Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 3, tm_year: 115, tm_wday: 3, tm_yday: 90, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // may:       Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 4, tm_year: 115, tm_wday: 5, tm_yday: 120, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // june:      Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 5, tm_year: 115, tm_wday: 1, tm_yday: 151, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // july:      Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 6, tm_year: 115, tm_wday: 3, tm_yday: 181, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // august:    Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 7, tm_year: 115, tm_wday: 6, tm_yday: 212, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // september: Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 8, tm_year: 115, tm_wday: 2, tm_yday: 243, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // october:   Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 9, tm_year: 115, tm_wday: 4, tm_yday: 273, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // november:  Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 10, tm_year: 115, tm_wday: 0, tm_yday: 304, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
            // december:  Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 11, tm_year: 115, tm_wday: 2, tm_yday: 334, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 },
        }
    }

    fn end_tms() -> ExampleTms {
        ExampleTms {
            january: datetime!(2015-01-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            february: datetime!(2015-02-28 23:59:59.999999999).assume_offset(offset!(UTC)),
            march: datetime!(2015-03-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            april: datetime!(2015-04-30 23:59:59.999999999).assume_offset(offset!(UTC)),
            may: datetime!(2015-05-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            june: datetime!(2015-06-30 23:59:59.999999999).assume_offset(offset!(UTC)),
            july: datetime!(2015-07-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            august: datetime!(2015-08-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            september: datetime!(2015-09-30 23:59:59.999999999).assume_offset(offset!(UTC)),
            october: datetime!(2015-10-31 23:59:59.999999999).assume_offset(offset!(UTC)),
            november: datetime!(2015-11-30 23:59:59.999999999).assume_offset(offset!(UTC)),
            december: datetime!(2015-12-31 23:59:59.999999999).assume_offset(offset!(UTC)),

            // january:     Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115, tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999, },
            // february:    Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 28, tm_mon: 1, tm_year: 115, tm_wday: 6, tm_yday: 58, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // march:       Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 2, tm_year: 115, tm_wday: 2, tm_yday: 89, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // april:       Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 3, tm_year: 115, tm_wday: 4, tm_yday: 119, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // may:         Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 4, tm_year: 115, tm_wday: 0, tm_yday: 150, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // june:        Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 5, tm_year: 115, tm_wday: 2, tm_yday: 180, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // july:        Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 6, tm_year: 115, tm_wday: 5, tm_yday: 211, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // august:      Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 7, tm_year: 115, tm_wday: 1, tm_yday: 242, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // september:   Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 8, tm_year: 115, tm_wday: 3, tm_yday: 272, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // october:     Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 9, tm_year: 115, tm_wday: 6, tm_yday: 303, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // november:    Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 10, tm_year: 115, tm_wday: 1, tm_yday: 333, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
            // december:    Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 11, tm_year: 115, tm_wday: 4, tm_yday: 364, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 },
        }
    }

    #[test]
    fn test_start_of_month() {
        assert_eq!(DateTime::create_from_tm(start_tms().january), DateTime::create_from_tm(end_tms().january).start_of().year());
        assert_eq!(DateTime::create_from_tm(start_tms().january), DateTime::create_from_tm(end_tms().january).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().february), DateTime::create_from_tm(end_tms().february).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().march), DateTime::create_from_tm(end_tms().march).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().april), DateTime::create_from_tm(end_tms().april).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().may), DateTime::create_from_tm(end_tms().may).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().june), DateTime::create_from_tm(end_tms().june).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().july), DateTime::create_from_tm(end_tms().july).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().august), DateTime::create_from_tm(end_tms().august).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().september), DateTime::create_from_tm(end_tms().september).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().october), DateTime::create_from_tm(end_tms().october).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().november), DateTime::create_from_tm(end_tms().november).start_of().month());
        assert_eq!(DateTime::create_from_tm(start_tms().december), DateTime::create_from_tm(end_tms().december).start_of().month());

        // let january_mid = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 15, tm_mon: 0, tm_year: 115, tm_wday: 4, tm_yday: 14, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
        let january_mid = datetime!(2015-01-15 0:00 +000);
        assert_eq!(DateTime::create_from_tm(start_tms().january), DateTime::create_from_tm(january_mid).start_of().month());
    }

    #[test]
    fn test_start_of_day() {
        let january_31th_starts = datetime!(2015-01-31 0:00 +000);
        // let january_31th_starts = Tm {
        //     tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };
        assert_eq!(
            DateTime::create_from_tm(january_31th_starts),
            DateTime::create_from_tm(end_tms().january).start_of().day()
        );
    }

    #[test]
    fn test_start_of_hour() {
        let january_31th_23h_starts = datetime!(2015-01-31 23:00:00 +000);
        // let january_31th_23h_starts = Tm {
        //     tm_sec: 0, tm_min: 0, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(january_31th_23h_starts),
            DateTime::create_from_tm(end_tms().january).start_of().hour()
        );
    }

    #[test]
    fn test_start_of_minute() {
        let january_31th_23h_59m_starts = datetime!(2015-01-31 23:59 +000);
        // let january_31th_23h_59m_starts = Tm {
        //     tm_sec: 0, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(january_31th_23h_59m_starts),
            DateTime::create_from_tm(end_tms().january).start_of().minute()
        );
    }

    #[test]
    fn test_start_of_second() {
        let january_31th_23h_59m_59s_starts = datetime!(2015-01-31 23:59:59 +000);
        // let january_31th_23h_59m_59s_starts = Tm {
        //     tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(january_31th_23h_59m_59s_starts),
            DateTime::create_from_tm(end_tms().january).start_of().second()
        );
    }

    #[test]
    fn test_end_of_month() {
        assert_eq!(DateTime::create_from_tm(end_tms().january), DateTime::create_from_tm(end_tms().january).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().february), DateTime::create_from_tm(end_tms().february).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().march), DateTime::create_from_tm(end_tms().march).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().april), DateTime::create_from_tm(end_tms().april).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().may), DateTime::create_from_tm(end_tms().may).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().june), DateTime::create_from_tm(end_tms().june).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().july), DateTime::create_from_tm(end_tms().july).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().august), DateTime::create_from_tm(end_tms().august).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().september), DateTime::create_from_tm(end_tms().september).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().october), DateTime::create_from_tm(end_tms().october).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().november), DateTime::create_from_tm(end_tms().november).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().december), DateTime::create_from_tm(end_tms().december).end_of().month());
        assert_eq!(DateTime::create_from_tm(end_tms().december), DateTime::create_from_tm(end_tms().december).end_of().year());

        let january_mid = datetime!(2015-01-15 0:00 +000);
        // let january_mid = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 15, tm_mon: 0, tm_year: 115, tm_wday: 4, tm_yday: 14, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
        assert_eq!(DateTime::create_from_tm(end_tms().january), DateTime::create_from_tm(january_mid).end_of().month());
    }

    #[test]
    fn test_end_of_day() {
        let january_31th_starts = datetime!(2015-01-31 0:00 +000);
        // let january_31th_starts = Tm {
        //     tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 32, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(end_tms().january),
            DateTime::create_from_tm(january_31th_starts).end_of().day()
        );
    }

    #[test]
    fn test_end_of_hour() {
        let january_31th_23h_starts = datetime!(2015-01-31 23:59:59.999999999 +000);
        // let january_31th_23h_starts = Tm {
        //     tm_sec: 0, tm_min: 0, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(end_tms().january),
            DateTime::create_from_tm(january_31th_23h_starts).end_of().hour()
        );
    }

    #[test]
    fn test_end_of_minute() {
        let january_31th_23h_59m_starts = datetime!(2015-01-31 23:59:00.00 +000);
        // let january_31th_23h_59m_starts = Tm {
        //     tm_sec: 0, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(end_tms().january),
            DateTime::create_from_tm(january_31th_23h_59m_starts).end_of().minute()
        );
    }

    #[test]
    fn test_end_of_second() {
        let january_31th_23h_59m_59s_starts = datetime!(2015-01-31 23:59:59.000 +000);
        // let january_31th_23h_59m_59s_starts = Tm {
        //     tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115,
        //     tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
        // };

        assert_eq!(
            DateTime::create_from_tm(end_tms().january),
            DateTime::create_from_tm(january_31th_23h_59m_59s_starts).end_of().second()
        );
    }

    #[test]
    fn test_test_now() {
        DateTime::set_test_now(Some(DateTime::create_from_tm(start_tms().january)));
        assert_eq!(DateTime::create_from_tm(start_tms().january), DateTime::now());
        DateTime::set_test_now(None);
    }

    // #[test]
    // fn it_can_get_wday_from_time_library() {
    //     let arbitrary = datetime!(2023-01-01 12:34:56 +000);
    //     // arbitrary.date().weekday();
    //     assert_eq!(
    //         6,
    //         arbitrary.date().weekday()
    //     );

    // }

    // #[test]
    // fn test_integration_for_tm() {
    //     let tm_string = DateTime::create_from_tm(start_tms().january).tm.fmt("%Y-%m-%d %H:%M:%S").ok().unwrap().to_string();
    //     assert_eq!("2015-01-01 00:00:00", tm_string);

    //     let h = Duration::hours(1);
    //     let added = DateTime::create_from_tm(start_tms().january).tm.add(h).fmt("%Y-%m-%d %H:%M:%S").ok().unwrap().to_string();
    //     assert_eq!("2015-01-01 01:00:00", added);
    // }
}
