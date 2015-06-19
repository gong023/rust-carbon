use super::{DateTime, Duration};

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

impl<'a> Duration for End<'a> {
    fn month(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_mday = self.date_time.days_in_month(self.date_time.t.tm_mon);

        let difference = (copied_tm.tm_mday - 1) % 7;
        if self.date_time.t.tm_wday + difference > 6 {
            copied_tm.tm_wday = difference - 1;
        } else {
            copied_tm.tm_wday = self.date_time.t.tm_wday + difference;
        }

        let mut yday = 0;
        for month in (1..self.date_time.t.tm_mon) {
            yday += self.date_time.days_in_month(month);
        }
        copied_tm.tm_yday = yday - 1;

        DateTime::create_from_tm(copied_tm).end_of().day()
    }

    fn day(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_hour = 23;
        DateTime::create_from_tm(copied_tm).end_of().hour()
    }

    fn hour(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_min = 59;
        DateTime::create_from_tm(copied_tm).end_of().minute()
    }

    fn minute(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_sec = 60;
        DateTime::create_from_tm(copied_tm).end_of().second()
    }

    fn second(&self) -> DateTime {
        let mut copied_tm = self.date_time.t;
        copied_tm.tm_nsec = 999999999;
        DateTime::create_from_tm(copied_tm)
    }
}
