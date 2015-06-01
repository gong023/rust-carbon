extern crate time;

pub use time::*;

pub struct CarbonDate {
    t: Tm,
}

impl CarbonDate {
    fn now() -> CarbonDate {
        CarbonDate {
            t: time::now(),
        }
    }

    fn to_string(&self) -> time::TmFmt {
        self.t.ctime()
    }

    fn unixtime(&self) -> time::Timespec {
        self.t.to_timespec()
    }
}

fn main() {
    println!("{}", time::now().ctime());
    println!("{}", CarbonDate::now().to_string());
    println!("{:?}", CarbonDate::now().unixtime().sec);
}
