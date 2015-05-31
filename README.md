# rust-carbon

```rust
let date = Date::now('UTC');

date.toString();
```

modify date

```rust
let format = DateFormat::new()
let date = Date::create(format);

date.toString();

date.unixtime();
date.startOfDay().toString();
date.startOfYear().startOfMonth().startOfDay().startOfHour().startOfMinute().toString();

date.addDay(1i).toString();
date.addYear(1i).addMonth(1i).addDay(1i).addHour(1i).addMinute(1i).addSecond(1i).toString();

date.subDay(1i).toString();
date.subYear(1i).subMonth(1i).subDay(1i).subHour(1i).subMinute(1i).subSecond(1i).subString();
```

compare date

```rust
let future_date = Date::now('UTC').addSecond(10i);
let now = Date::now('UTC');

now.greaterThan(future_date); // false
now.greaterThanOrEqual(future_date); // false

now.lessThan(future_date); // true
now.lessThanOrEqual(future_date); // true
```
