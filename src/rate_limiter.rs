//-------------------------------------------------------------------
// @author yangcancai

// Copyright (c) 2021 by yangcancai(yangcancai0112@gmail.com), All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//       https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// @doc
//
// @end
// Created : 2021-07-02T06:58:13+00:00
//-------------------------------------------------------------------

extern crate time;
use std::collections::HashMap;
use time::Duration;
#[derive(Debug ,PartialEq)]
pub enum RateError {
   ZeroRatesNoSupported 
}
#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub limit: i64,
    pub remaining: i64,
    pub reset_after: time::Duration,
    pub retry_after: time::Duration,
}
pub struct Store {
    data: HashMap<String, (i64, time::Tm)>,
}
pub trait StoreI {
    fn get(&self, key: &str) -> i64;
    fn set(&mut self, key: String, val: i64, ttl: time::Duration);
}
impl Store{
    pub fn new() -> Self{
	    Self{
		    data: HashMap::new()
	    }
    }
}
impl StoreI for &mut Store{
    fn get(&self, key: &str) -> i64 {
        match self.data.get(key) {
            None => -1,
            Some(v) => {
                if v.1 <= time::now_utc() {
                    -1
                } else {
                    v.0
                }
            }
        }
    }
    fn set(&mut self, key: String, val: i64, ttl: Duration) {
        let expired = time::now_utc() + ttl;
        self.data.insert(key, (val, expired));
    }
}
pub struct RateLimiter<T> {
    store: T,
    delay_variation_tolerance: time::Duration,
    emission_interval: time::Duration,
    limit: i64,
}
fn per_period(n: i64, seconds: i64) -> time::Duration {
    let period = time::Duration::seconds(seconds);
    let ns: i64 = period.num_nanoseconds().unwrap();
    if n == 0 || ns == 0 {
        return time::Duration::nanoseconds(0);
    }
    time::Duration::nanoseconds(((ns as f64) / (n as f64)) as i64)
}
pub fn to_second(dur: time::Duration) -> i64{
    let milliseconds = dur.num_milliseconds();
	if milliseconds % 1000 == 0{
		milliseconds / 1000
	}else{
		milliseconds / 1000 + 1
	}
}
fn from_nanoseconds(x: i64) -> time::Tm {
    let ns = 10_i64.pow(9);
    time::at(time::Timespec {
        sec: x / ns,
        nsec: (x % ns) as i32,
    })
}
fn nanoseconds(x: time::Tm) -> i64 {
    let ts = x.to_timespec();
    ts.sec * 10_i64.pow(9) + i64::from(ts.nsec)
}
impl<T: StoreI> RateLimiter<T> {
    pub fn new(store: T) -> Self {
        RateLimiter {
            store: store,
            delay_variation_tolerance: time::Duration::seconds(0),
            emission_interval: time::Duration::seconds(0),
            limit: 0,
        }
    }
    fn reflesh(&mut self, burst: i64, count: i64, seconds: i64) {
        let per_period = per_period(count, seconds);
        self.delay_variation_tolerance =
            time::Duration::nanoseconds(per_period.num_nanoseconds().unwrap() * (burst + 1));
        self.emission_interval = per_period;
        self.limit = burst + 1;
    }
    /// Rate limit
    ///
    /// # Examples 
    ///
    /// ```
    /// use rate_limiter::rate_limiter::Store;
    /// use rate_limiter::RateLimiter;
    /// use rate_limiter::rate_limiter::to_second;
    /// let mut store = Store::new();
    /// let mut rate_limiter = RateLimiter::new(&mut store);
    /// let rs = rate_limiter.rate_limit("foo".to_string(),10,1,1,1).unwrap();
    /// assert_eq!(rs.allowed, true);
	/// assert_eq!(rs.remaining, 10);
	/// assert_eq!(rs.limit, 11);
	/// assert_eq!(to_second(rs.retry_after), -1);
	/// assert_eq!(to_second(rs.reset_after), 1);
    /// ```
 
    pub fn rate_limit(
        &mut self,
        key: String,
        burst: i64,
        count: i64,
        seconds: i64,
        quantity: i64,
    ) -> Result<RateLimitResult, RateError> {
        self.reflesh(burst, count, seconds);
        if self.emission_interval == time::Duration::seconds(0){
            return Err(RateError::ZeroRatesNoSupported)
        }
        let tat = self.store.get(&key);
        let now = time::now_utc();
        let increment = time::Duration::nanoseconds(
            quantity * self.emission_interval.num_nanoseconds().unwrap(),
        );
        let tat = match tat {
            -1 => now,
            _ => from_nanoseconds(tat),
        };
        let new_tat = if now > tat {
            now + increment
        } else {
            tat + increment
        };
        let allow_at = new_tat - self.delay_variation_tolerance;
        let diff = now - allow_at;
        let mut limited = false;
        let ttl;
        let mut remaining = 0;
        let reset_after;
        let mut retry_after = time::Duration::seconds(-1);
        if diff < time::Duration::zero() {
            if increment <= self.delay_variation_tolerance {
                retry_after = -diff;
            }
            limited = true;
            ttl = tat - now;
        } else {
            let new_tat_ns = nanoseconds(new_tat);
            ttl = new_tat - now;
            self.store.set(key, new_tat_ns, ttl);
        }

        let next = self.delay_variation_tolerance - ttl;
        if next > -self.emission_interval {
            remaining = (next.num_microseconds().unwrap() as f64
                / self.emission_interval.num_microseconds().unwrap() as f64)
                as i64;
        }
        reset_after = ttl;
        Ok(RateLimitResult {
            allowed: !limited,
            limit: self.limit,
            remaining: remaining,
            reset_after: reset_after,
            retry_after: retry_after,
        })
    }
}
