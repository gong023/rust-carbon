extern crate time;

mod start;
mod end;

pub use time::Tm;
pub use start::Start;
pub use end::End;

#[derive(PartialEq, Eq, Debug)]
pub struct DateTime {
    t: Tm,
}

pub trait Duration {
    fn month(&self) -> DateTime;

    fn day(&self) -> DateTime;

    fn hour(&self) -> DateTime;

    fn minute(&self) -> DateTime;

    fn second(&self) -> DateTime;
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

    pub fn start_of(&self) -> Start {
        Start::new(&self)
    }

    pub fn end_of(&self) -> End {
        End::new(&self)
    }

    pub fn is_leap_year(&self) -> bool {
        let year = 1900 + self.t.tm_year;
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
    }

    pub fn to_string(&self) -> time::TmFmt {
        self.t.ctime()
    }

    pub fn unixtime(&self) -> time::Timespec {
        self.t.to_timespec()
    }

    fn days_in_month(&self, month: i32) -> i32 {
        match month {
            2 => { if self.is_leap_year() { 29 } else { 28 } },
            4 | 6 | 9 | 11 => 31,
            _ => 30,
        }
    }
}
