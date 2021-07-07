# rate_limiter

A RateLimit library come from redis-cell
![CI](https://github.com/yangcancai/rate_limiter/actions/workflows/ci.yml/badge.svg)

# Required

* rust 

# How it works

```rust
    use rate_limiter::rate_limiter::Store;
    use rate_limiter::RateLimiter;
    use rate_limiter::rate_limiter::to_second;
    let mut store = Store::new();
    let mut rate_limiter = RateLimiter::new(&mut store);
    let rs = rate_limiter.rate_limit("foo".to_string(),10,1,1,1).unwrap();
    assert_eq!(rs.allowed, true);
    assert_eq!(rs.remaining, 10);
    assert_eq!(rs.limit, 11);
    assert_eq!(to_second(rs.retry_after), -1);
    assert_eq!(to_second(rs.reset_after), 1);
```
# Reference

[redis-cell](https://github.com/brandur/redis-cell)
[rate-limiting](https://brandur.org/rate-limiting)