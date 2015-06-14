extern crate time;

pub use time::*;

pub struct DateTime {
    pub t: Tm,
}

impl DateTime {
    pub fn now() -> DateTime {
        DateTime {
            t: time::now(),
        }
    }

    pub fn create_from_tm(tm: Tm) -> DateTime {
        DateTime {
            t: tm,
        }
    }

    pub fn to_string(&self) -> time::TmFmt {
        self.t.ctime()
    }

    pub fn unixtime(&self) -> time::Timespec {
        self.t.to_timespec()
    }

    pub fn start_of_month(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_mday = 1;

        match self.t.tm_mday % 7 {
            0 =>  copied_tm.tm_wday = 0,
            _ => { copied_tm.tm_wday = self.t.tm_wday - (self.t.tm_mday % 7) + 1; },
        }

        match self.t.tm_mon {
            1 => copied_tm.tm_yday = 0,
            _ => {
                let mut yday = 0;
                for m in (1..self.t.tm_mon) {
                    yday += self.days_in_month(m);
                }
                copied_tm.tm_yday = yday + 1;
            }
        }

        DateTime::create_from_tm(copied_tm).start_of_day()
    }

    pub fn start_of_day(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_hour = 0;
        DateTime::create_from_tm(copied_tm).start_of_hour()
    }

    pub fn start_of_hour(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_min = 0;
        DateTime::create_from_tm(copied_tm).start_of_minute()
    }

    pub fn start_of_minute(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_sec = 0;
        DateTime::create_from_tm(copied_tm).start_of_second()
    }

    pub fn start_of_second(&self) -> DateTime {
        let mut copied_tm = self.t;
        copied_tm.tm_nsec = 0;
        DateTime::create_from_tm(copied_tm)
    }

    pub fn is_leap_year(&self) -> bool {
        let y = 1900 + self.t.tm_year;
        (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
    }

    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            2 => { if self.is_leap_year() { 29 } else { 28 } },
            4 | 6 | 9 | 11 => 31,
            _ => 30,
        }
    }
}


