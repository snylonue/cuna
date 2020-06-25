#[cfg(test)]
mod time {
    use cuna::time::*;
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
        assert!("6:72:111".parse::<Duration>().is_err());
    }
    #[test]
    fn modify() {
        let mut duration = Duration::new(21, 29, 73);
        duration.set_frames(21);
        assert_eq!(duration, Duration::new(21, 29, 21));
        duration.set_seconds(33);
        assert_eq!(duration, Duration::new(21, 33, 21));
        duration.set_minutes(28);
        assert_eq!(duration, Duration::new(28, 33, 21));
    }
    #[test]
    #[should_panic]
    fn modify_panic() {
        let mut duration = Duration::new(61, 29, 73);
        duration.set_frames(88);
    }
}
#[cfg(test)]
mod command {
    use cuna::parser::Command;

    #[test]
    fn display() {
        let cmds = r#"REM COMMENT ExactAudioCopy v0.99pb5
        PERFORMER "Supercell"
        TITLE "My Dearest"
        FILE "Supercell - My Dearest.flac" WAVE"#;
        for (cmd, ori) in cmds.lines().map(Command::new).zip(cmds.lines()) {
            assert_eq!(format!("{}", cmd.unwrap()), ori.trim().to_string())
        }
    }
}