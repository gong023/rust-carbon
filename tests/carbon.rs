extern crate carbon;
extern crate time;

pub use carbon::*;

#[test]
fn test_start_of_month() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(january_starts),
        DateTime::create_from_tm(january_ends).start_of().month()
    );

    let february_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 28, tm_mon: 2, tm_year: 115,
        tm_wday: 6, tm_yday: 59, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let february_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 2, tm_year: 115,
        tm_wday: 0, tm_yday: 31, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(february_starts),
        DateTime::create_from_tm(february_ends).start_of().month()
    );
}

#[test]
fn test_start_of_day() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(january_starts),
        DateTime::create_from_tm(january_ends).start_of().day()
    );
}

#[test]
fn test_start_of_hour() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(january_starts),
        DateTime::create_from_tm(january_ends).start_of().hour()
    );
}

#[test]
fn test_start_of_minute() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(january_starts),
        DateTime::create_from_tm(january_ends).start_of().minute()
    );
}

#[test]
fn test_start_of_second() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
    };

    let january_starts = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    assert_eq!(
        DateTime::create_from_tm(january_starts),
        DateTime::create_from_tm(january_ends).start_of().second()
    );
}

#[test]
fn test_is_leap_year() {
    let year_2012 = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1,
        tm_year: 112, tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };
    assert!(DateTime::create_from_tm(year_2012).is_leap_year());

    let year_2000 = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1,
        tm_year: 100, tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };
    assert!(DateTime::create_from_tm(year_2000).is_leap_year());

    let year_2100 = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1,
        tm_year: 200, tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };
    assert!(! DateTime::create_from_tm(year_2100).is_leap_year());
}

#[test]
fn test_end_of_second() {
    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let january_ends = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(january_ends),
        DateTime::create_from_tm(january_starts).end_of().second()
    );
}

#[test]
fn test_end_of_minute() {
    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let january_ends = time::Tm {
        tm_sec: 60, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(january_ends),
        DateTime::create_from_tm(january_starts).end_of().minute()
    );
}

#[test]
fn test_end_of_hour() {
    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let january_ends = time::Tm {
        tm_sec: 60, tm_min: 59, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(january_ends),
        DateTime::create_from_tm(january_starts).end_of().hour()
    );
}

#[test]
fn test_end_of_day() {
    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let january_ends = time::Tm {
        tm_sec: 60, tm_min: 59, tm_hour: 23, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(january_ends),
        DateTime::create_from_tm(january_starts).end_of().day()
    );
}
