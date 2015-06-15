use super::{DateTime, Duration};

pub struct Start<'a> {
    date_time: &'a DateTime
}

impl<'a> Start<'a> {
    pub fn new(dt: &'a DateTime) -> Start {
        Start {
            date_time: dt
        }
    }

    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            2 => { if self.date_time.is_leap_year() { 29 } else { 28 } },
            4 | 6 | 9 | 11 => 31,
            _ => 30,
        }
    }
}

impl<'a> Duration for Start<'a> {
    fn month(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_mday = 1;

        match self.date_time.t.tm_mday % 7 {
            0 =>  copied_tm.tm_wday = 0,
            _ => { copied_tm.tm_wday = self.date_time.t.tm_wday - (self.date_time.t.tm_mday % 7) + 1; },
        }

        match self.date_time.t.tm_mon {
            1 => copied_tm.tm_yday = 0,
            _ => {
                let mut yday = 0;
                for m in (1..self.date_time.t.tm_mon) {
                    yday += self.days_in_month(m);
                }
                copied_tm.tm_yday = yday + 1;
            }
        }

        DateTime::create_from_tm(copied_tm).start_of().day()
    }

    fn day(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_hour = 0;
        DateTime::create_from_tm(copied_tm).start_of().hour()
    }

    fn hour(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_min = 0;
        DateTime::create_from_tm(copied_tm).start_of().minute()
    }

    fn minute(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_sec = 0;
        DateTime::create_from_tm(copied_tm).start_of().second()
    }

    fn second(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_nsec = 0;
        DateTime::create_from_tm(copied_tm)
    }
}
