extern crate carbon;
extern crate time;

pub use carbon::*;

let january_starts   = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 0, tm_year: 115, tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let february_starts  = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115, tm_wday: 0, tm_yday: 31, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let march_starts     = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 2, tm_year: 115, tm_wday: 0, tm_yday: 59, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let april_starts     = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 3, tm_year: 115, tm_wday: 3, tm_yday: 90, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let may_starts       = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 4, tm_year: 115, tm_wday: 5, tm_yday: 120, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let june_starts      = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 5, tm_year: 115, tm_wday: 1, tm_yday: 151, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let july_starts      = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 6, tm_year: 115, tm_wday: 3, tm_yday: 181, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let august_starts    = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 7, tm_year: 115, tm_wday: 6, tm_yday: 212, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let september_starts = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 8, tm_year: 115, tm_wday: 2, tm_yday: 243, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let october_starts   = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 9, tm_year: 115, tm_wday: 4, tm_yday: 273, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let november_starts  = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 10, tm_year: 115, tm_wday: 0, tm_yday: 304, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
let december_starts  = Tm { tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 11, tm_year: 115, tm_wday: 2, tm_yday: 334, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };

let january_ends     = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 0, tm_year: 115, tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999, };
let february_ends    = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 28, tm_mon: 1, tm_year: 115, tm_wday: 6, tm_yday: 58, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let march_ends       = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 2, tm_year: 115, tm_wday: 2, tm_yday: 89, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let april_ends       = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 3, tm_year: 115, tm_wday: 4, tm_yday: 119, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let may_ends         = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 4, tm_year: 115, tm_wday: 0, tm_yday: 150, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let june_ends        = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 5, tm_year: 115, tm_wday: 2, tm_yday: 180, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let july_ends        = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 6, tm_year: 115, tm_wday: 5, tm_yday: 211, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let august_ends      = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 7, tm_year: 115, tm_wday: 1, tm_yday: 242, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let september_ends   = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 8, tm_year: 115, tm_wday: 3, tm_yday: 272, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let october_ends     = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 9, tm_year: 115, tm_wday: 6, tm_yday: 303, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let november_ends    = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 10, tm_year: 115, tm_wday: 1, tm_yday: 333, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };
let december_ends    = Tm { tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 11, tm_year: 115, tm_wday: 4, tm_yday: 364, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999 };

#[test]
fn test_start_of_month() {
    let january_ends = time::Tm {
        tm_sec: 59, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 31, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 100,
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
        tm_wday: 0, tm_yday: 32, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
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

#[test]
fn test_end_of_month() {
    let january_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 1, tm_year: 115,
        tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let january_ends = time::Tm {
        tm_sec: 60, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 1, tm_year: 115,
        tm_wday: 6, tm_yday: 30, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(january_ends),
        DateTime::create_from_tm(january_starts).end_of().month()
    );

    let june_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 6, tm_year: 115,
        tm_wday: 1, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let june_ends = time::Tm {
        tm_sec: 60, tm_min: 59, tm_hour: 23, tm_mday: 30, tm_mon: 6, tm_year: 115,
        tm_wday: 2, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(june_ends),
        DateTime::create_from_tm(june_starts).end_of().month()
    );

    let august_starts = time::Tm {
        tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 8, tm_year: 115,
        tm_wday: 6, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0,
    };

    let august_ends = time::Tm {
        tm_sec: 60, tm_min: 59, tm_hour: 23, tm_mday: 31, tm_mon: 8, tm_year: 115,
        tm_wday: 1, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 999999999,
    };

    assert_eq!(
        DateTime::create_from_tm(august_ends),
        DateTime::create_from_tm(august_starts).end_of().month()
    );
}
