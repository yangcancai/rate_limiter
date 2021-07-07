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

use std::thread;

use rate_limiter::RateLimiter;
use rate_limiter::Store;
use rate_limiter::rate_limiter::RateError;
use rate_limiter::rate_limiter::to_second;
#[test]
fn invalid_params(){
       let mut store = Store::new();
       let mut limiter = RateLimiter::new(&mut store); 
	let rs = limiter.rate_limit("test".into(), 10, 1, 0, 1).unwrap_err();
	assert_eq!(rs, RateError::ZeroRatesNoSupported)
}
#[test]
fn rate_limit(){
	let mut store = Store::new();
	let mut limiter = RateLimiter::new(&mut store); 
	let rs = limiter.rate_limit("test".into(), 10, 1, 1, 1).unwrap();
	assert_eq!(rs.allowed, true);
	let rs = limiter.rate_limit("test".into(), 10, 1, 1, 5).unwrap();
	assert_eq!(rs.allowed, true);
	let rs = limiter.rate_limit("test".into(), 10, 1, 1, 5).unwrap();
	assert_eq!(rs.allowed, true);
	let rs = limiter.rate_limit("test".into(), 10, 1, 1, 1).unwrap();
	assert_eq!(rs.allowed, false);
	assert_eq!(rs.remaining, 0);
	assert_eq!(rs.limit, 11);
	assert_eq!(to_second(rs.retry_after), 1);
	assert_eq!(to_second(rs.reset_after), 11);
	thread::sleep(std::time::Duration::from_secs(2) + 
	std::time::Duration::from_millis(rs.retry_after.num_milliseconds() as u64));
	let rs = limiter.rate_limit("test".into(), 10, 1, 1, 3).unwrap();
	assert_eq!(rs.allowed, true);
	assert_eq!(rs.remaining, 0);
	assert_eq!(rs.limit, 11);
	assert_eq!(to_second(rs.retry_after), -1);
	assert_eq!(to_second(rs.reset_after), 11);
}
#[test]
fn to_second_work(){
	assert_eq!(to_second(time::Duration::milliseconds(1)), 1);
	assert_eq!(to_second(time::Duration::milliseconds(0)), 0);
	assert_eq!(to_second(time::Duration::milliseconds(999)), 1);
	assert_eq!(to_second(time::Duration::milliseconds(1001)), 2);
	assert_eq!(to_second(time::Duration::milliseconds(1999)), 2);
}
