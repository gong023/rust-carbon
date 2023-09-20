use super::{DateTime, CarbonDuration};

pub struct Start<'a> {
    date_time: &'a DateTime
}

impl<'a> Start<'a> {
    pub fn new(dt: &'a DateTime) -> Start {
        Start {
            date_time: dt
        }
    }
}

impl<'a, 'd> CarbonDuration<'d> for Start<'a> {
    fn year(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_date(
            self.date_time.tm.date().replace_month(time::Month::January).expect("Could not replace month")
        )).start_of().month()
    }

    fn month(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_date(
            self.date_time.tm.date().replace_day(1).expect("Could not replace day")
        )).start_of().day()
    }

    fn day(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_hour(0).expect("Could not replace minute")
        )).start_of().hour()
    }

    fn hour(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_minute(0).expect("Could not replace minute")
        )).start_of().minute()
    }

    fn minute(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_second(0).expect("Could not replace second")
        )).start_of().second()
    }

    fn second(&self) -> DateTime {
        DateTime::create_from_tm(self.date_time.tm.replace_time(
            self.date_time.tm.time().replace_nanosecond(0).expect("Could not replace second")
        ))
    }
}
