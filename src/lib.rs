extern crate time;

pub use time::*;

pub struct DateTime {
    t: Tm,
}

impl DateTime {
    pub fn now() -> DateTime {
        DateTime {
            t: time::now(),
        }
    }

    pub fn test_now(tm: Tm) -> DateTime {
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

    pub fn start_of_month(&self) -> time::Tm {
        let mut tt = self.t;
        tt.tm_mday = 1;

        tt.tm_wday = self.t.tm_wday - (self.t.tm_mday % 7) + 1;
        // todo:start_of_day
        return tt;
    }

    // create Tm from unixtime
//    pub fn at(clock: Timespec) -> DateTime {
//    }
}
