# rust-carbon

rust-carbon is thin wrapper of [rust-time](https://github.com/rust-lang/time). Utility for handling date and time.

[![Build Status](https://travis-ci.org/gong023/rust-carbon.svg?branch=master)](https://travis-ci.org/gong023/rust-carbon)

# Usage

Add rust-carbon to your Cargo.toml.

```toml
[dependencies]
carbon = "0.1.*"
```

And put this in your crate root.

```rust
extern crate carbon;
```

# Overview

rust-carbon provides `start_of()` and `end_of()` to get datetime.
And you can modify datetime by `second`,`minute`,`hour`,`day`, and `month`.

```rust
// import carbon first
use carbon::*;

// now assume that it is 2015-01-15 01:30:30.500000000

DateTime::now().start_of().second();
// => return carbon::DateTime for 2015-01-15 01:30:30.000000000
DateTime::now().start_of().minute();
// => return carbon::DateTime for 2015-01-15 01:30:00.000000000
DateTime::now().start_of().hour();
// => return carbon::DateTime for 2015-01-15 01:00:00.000000000
DateTime::now().start_of().day();
// => return carbon::DateTime for 2015-01-15 00:00:00.000000000
DateTime::now().start_of().month();
// => return carbon::DateTime for 2015-01-01 00:00:00.000000000
DateTime::now().start_of().year();
// => return carbon::DateTime for 2015-01-15 00:00:00.000000000

DateTime::now().end_of().second();
// => return carbon::DateTime for 2015-01-15 01:30:30.999999999
DateTime::now().end_of().minute();
// => return carbon::DateTime for 2015-01-15 01:30:59.999999999
DateTime::now().end_of().hour();
// => return carbon::DateTime for 2015-01-15 01:59:59.999999999
DateTime::now().end_of().day();
// => return carbon::DateTime for 2015-01-15 23:59:59.999999999
DateTime::now().end_of().month();
// => return carbon::DateTime for 2015-01-31 23:59:59.999999999
DateTime::now().end_of().year();
// => return carbon::DateTime for 2015-12-31 11:59:59.999999999
```

# More sample codes

Chaining method calls

```rust
// now assume that it is 2015-01-15 01:30:30.500000000

DateTime::now().start_of().day().end_of().hour();
// => return carbon::DateTime for 2015-01-15 00:59:59.999999999
```

Directly specify time and set `now` the time. It is useful for testing.

```rust
// carbon::DateTime for 2015-01-01 00:00:00
let tm = time::Tm {
  tm_sec: 0, tm_min: 0, tm_hour: 0, tm_mday: 1, tm_mon: 0, tm_year: 115, tm_wday: 4, tm_yday: 0, tm_isdst: 0, tm_utcoff: 0, tm_nsec: 0 };
};

let test_now = DateTime::create_from_tm(tm);
DateTime::set_test_now(test_now);
DateTime::now();
// => return carbon::DateTime for 2015-01-01 00:00:00
```

rust-carbon is thin wrapper of [rust-time](https://github.com/rust-lang/time). You can access `time::Tm` easily.

```rust
// now assume that it is 2015-01-15 01:30:30.500000000

DateTime::now().tm.strftime("%Y-%m-%d %H:%M:%S").ok().unwrap().to_string();
// => return string "2015-01-15 01:30:30"

use std::ops::{Add, Sub};

let hour = time::Duration::hours(1);
DateTime::now().tm.add(hour)
// return time::Tm for 2015-01-15 02:30:30.500000000
```

If you want to know more about rust-carbon, read document.

 - http://gong023.github.io/rust-carbon/carbon/index.html

# Known Issue

- rust-carbon can only deal UTC.
- enable to modify `start_of().year()`

Contributions are welcome :sparkles:
