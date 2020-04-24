#[cfg(test)]
mod time {
    use hana::time::*;
    #[test]
    fn create() {
        let duration = Duration::new(61, 29, 73);
        assert_eq!(Duration::from_msf_opt(61, 29, 73), Some(duration.clone()));
        assert_eq!(Duration::from_msf_opt(61, 29, 77), None);
        assert_eq!(Duration::from_msf_force(61, 28, 73 + 75), duration);
    }
    #[test]
    fn display() {
        let duration = Duration::new(61, 29, 73);
        assert_eq!(duration.to_string(), "61:29:73");
    }
    #[test]
    fn parse() {
        assert_eq!("61:29:73".parse::<Duration>().unwrap(), Duration::new(61, 29, 73));
        assert!("xd".parse::<Duration>().is_err());
        assert!("6:772:11".parse::<Duration>().is_err());
    }
}