extern crate time;

mod start;

pub use time::Tm;
pub use start::Start;

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

    pub fn is_leap_year(&self) -> bool {
        let y = 1900 + self.t.tm_year;
        (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
    }

    pub fn to_string(&self) -> time::TmFmt {
        self.t.ctime()
    }

    pub fn unixtime(&self) -> time::Timespec {
        self.t.to_timespec()
    }
}
