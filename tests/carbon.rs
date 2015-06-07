extern crate carbon;
extern crate time;

#[test]
fn test_start_of_month() {
    let january_second = time::Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 12,
        tm_mday: 2,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 5,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    let january_first = time::Tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 12,
        tm_mday: 1,
        tm_mon: 1,
        tm_year: 115,
        tm_wday: 4,
        tm_yday: 0,
        tm_isdst: 0,
        tm_utcoff: 0,
        tm_nsec: 0,
    };

    assert_eq!(january_first, carbon::DateTime::test_now(january_second).start_of_month());
}
