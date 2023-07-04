use std::convert::TryInto;

use super::{DateTime, CarbonDuration};

static NANOSECOND_MAX: u32 = 999999999;

#[derive(Clone, Copy)]
pub struct End<'a> {
    date_time: &'a DateTime
}

impl<'a> End<'a> {
    pub fn new(dt: &'a DateTime) -> End {
        End {
            date_time: dt
        }
    }
}

impl<'a, 'd> CarbonDuration<'d> for End<'a> {
    fn year(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_date(
            self.date_time.tm.date().replace_month(time::Month::December).expect("Could not replace month")
        )).end_of().month()
    }

    fn month(&self) -> DateTime {
        let month_number :i32 = match self.date_time.tm.date().month() {
            time::Month::January => 1,
            time::Month::February => 2,
            time::Month::March => 3,
            time::Month::April => 4,
            time::Month::May => 5,
            time::Month::June => 6,
            time::Month::July => 7,
            time::Month::August => 8,
            time::Month::September => 9,
            time::Month::October => 10,
            time::Month::November => 11,
            time::Month::December => 12,
        };
        DateTime::create_from_tm(self.date_time.tm.replace_day(
            self.date_time.days_in_month(month_number).try_into().unwrap()
        ).expect("Cannot replace day")).end_of().day()

    }

    fn day(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_hour(23).unwrap())
        ).end_of().hour()
    }

    fn hour(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_minute(59).unwrap())
        ).end_of().minute()
    }

    fn minute(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_second(59).unwrap())
        ).end_of().second()
    }

    fn second(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_nanosecond(NANOSECOND_MAX).expect("Unable to replace nanosecond")
        ))
    }
}
