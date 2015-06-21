use super::{DateTime, CarbonDuration, zeller_congurence};

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

impl<'a> CarbonDuration for Start<'a> {
    fn month(&self) -> DateTime {
        let mut copied_tm = self.date_time.tm;
        copied_tm.tm_mday = 1;

        copied_tm.tm_wday = zeller_congurence(
            self.date_time.tm.tm_year as f32,
            self.date_time.tm.tm_mon as f32,
            1.0
        );

        match self.date_time.tm.tm_mon {
            0 => copied_tm.tm_yday = 0,
            _ => {
                let mut yday = 0;
                for m in (0..self.date_time.tm.tm_mon) {
                    yday += self.date_time.days_in_month(m);
                }
                copied_tm.tm_yday = yday;
            }
        }

        DateTime::create_from_tm(copied_tm).start_of().day()
    }

    fn day(&self) -> DateTime {
        let mut copied_tm = self.date_time.tm;
        copied_tm.tm_hour = 0;
        DateTime::create_from_tm(copied_tm).start_of().hour()
    }

    fn hour(&self) -> DateTime {
        let mut copied_tm = self.date_time.tm;
        copied_tm.tm_min = 0;
        DateTime::create_from_tm(copied_tm).start_of().minute()
    }

    fn minute(&self) -> DateTime {
        let mut copied_tm = self.date_time.tm;
        copied_tm.tm_sec = 0;
        DateTime::create_from_tm(copied_tm).start_of().second()
    }

    fn second(&self) -> DateTime {
        let mut copied_tm = self.date_time.tm;
        copied_tm.tm_nsec = 0;
        DateTime::create_from_tm(copied_tm)
    }
}
