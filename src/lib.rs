pub mod rate_limiter;
pub use crate::rate_limiter::RateLimiter;
pub use crate::rate_limiter::Store;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
