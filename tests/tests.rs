type Result = std::result::Result<(), cuna::error::Error>;

const CUE: &str = include_str!(r"EGOIST - Departures ～あなたにおくるアイの歌～.cue");

#[cfg(test)]
mod time {
    use super::*;
    use cuna::time::*;
    use std::time::Duration;
    #[test]
    fn create() {
        let timestamp = TimeStamp::new(61, 29, 73);
        assert_eq!(TimeStamp::from_msf_opt(61, 29, 73), Some(timestamp.clone()));
        assert_eq!(TimeStamp::from_msf_opt(61, 29, 77), None);
        assert_eq!(TimeStamp::from_msf(61, 28, 73 + 75), timestamp);
    }
    #[test]
    fn display() {
        let timestamp = TimeStamp::new(61, 29, 73);
        assert_eq!(timestamp.to_string(), "61:29:73");
    }
    #[test]
    fn parse() -> Result {
        assert_eq!("61:29:73".parse::<TimeStamp>()?, TimeStamp::new(61, 29, 73));
        assert!("xd".parse::<TimeStamp>().is_err());
        assert!("6:772:11".parse::<TimeStamp>().is_err());
        assert!("6:72:111".parse::<TimeStamp>().is_err());
        Ok(())
    }
    #[test]
    fn modify() {
        let mut timestamp = TimeStamp::new(21, 29, 73);
        timestamp.set_frames(21);
        assert_eq!(timestamp, TimeStamp::new(21, 29, 21));
        timestamp.set_seconds(33);
        assert_eq!(timestamp, TimeStamp::new(21, 33, 21));
        timestamp.set_minutes(28);
        assert_eq!(timestamp, TimeStamp::new(28, 33, 21));
    }
    #[test]
    #[should_panic]
    fn modify_panic() {
        let mut timestamp = TimeStamp::new(61, 29, 73);
        timestamp.set_frames(88);
    }
    #[test]
    fn convert() {
        let timestamp = TimeStamp::new(0, 0, 30);
        assert_eq!(Duration::from(timestamp), Duration::from_millis(400));
        let duration = Duration::from_millis(400);
        assert_eq!(TimeStamp::from(duration), TimeStamp::new(0, 0, 30));
    }
    #[test]
    fn getter() {
        let timestamp = TimeStamp::new(21, 29, 73);
        assert_eq!(timestamp.minutes(), 21);
        assert_eq!(timestamp.seconds(), 29);
        assert_eq!(timestamp.frames(), 73);
        assert_eq!(timestamp.as_seconds(), 1289);
        assert_eq!(timestamp.as_frames(), 96748);
    }
}
#[cfg(test)]
mod command {
    use super::*;
    use cuna::parser::Command;

    #[test]
    fn new() -> Result {
        let cmd = r#"PERFORMER "Supercell""#;
        Command::new(cmd)?;
        Ok(())
    }
    #[test]
    fn display() -> Result {
        let cmds = r#"REM COMMENT ExactAudioCopy v0.99pb5
        PERFORMER "Supercell"
        TITLE "My Dearest"
        FILE "Supercell - My Dearest.flac" WAVE"#;
        for (cmd, ori) in cmds.lines().map(Command::new).zip(cmds.lines()) {
            assert_eq!(cmd?.to_string(), ori.trim().to_string())
        }
        Ok(())
    }
}
#[cfg(test)]
mod cue_sheet {
    use super::*;
    use cuna::CueSheet;

    #[test]
    fn new() -> Result {
        let sheet = CueSheet::from_utf8_with_bom(CUE)?;
        assert_eq!(sheet.comments.0[0], "GENRE Pop");
        assert_eq!(sheet.header.title, Some(vec!["Departures ～あなたにおくるアイの歌～".to_owned()]));
        assert_eq!(sheet.files.len(), 1);
        assert_eq!(sheet.last_file().unwrap().name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
        assert_eq!(sheet.last_track().unwrap().performer(), Some(&vec!["EGOIST".to_owned()]));
        Ok(())
    }
    #[test]
    fn from_buf_read() -> Result {
        let sheet = CueSheet::from_buf_read(&mut CUE.to_string().as_ref())?;
        assert_eq!(sheet.comments.0[0], "GENRE Pop");
        assert_eq!(sheet.header.title, Some(vec!["Departures ～あなたにおくるアイの歌～".to_owned()]));
        assert_eq!(sheet.files.len(), 1);
        assert_eq!(sheet.last_file().unwrap().name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
        assert_eq!(sheet.last_track().unwrap().performer(), Some(&vec!["EGOIST".to_owned()]));
        Ok(())
    }
}
#[cfg(test)]
mod parser {
    use super::*;
    use cuna::parser::Parser;
    use cuna::CueSheet;

    #[test]
    fn current_line() {
        let mut parser = Parser::new(cuna::trim_utf8_header(CUE));
        let mut sheet = CueSheet::default();
        assert_eq!(parser.current_line(), Some("REM GENRE Pop"));
        let _ = parser.parse_next_n_lines(5, &mut sheet);
        assert_eq!(parser.current_line(), Some(r#"TITLE "Departures ～あなたにおくるアイの歌～""#));
    }
    #[test]
    fn parse_next_n_lines() -> Result {
        let mut parser = Parser::new(cuna::trim_utf8_header(CUE));
        let mut sheet = CueSheet::default();
        parser.parse_next_n_lines(8, &mut sheet)?;
        assert_eq!(parser.current_line(), Some("  TRACK 01 AUDIO"));
        assert_eq!(sheet.header.title, Some(vec!["Departures ～あなたにおくるアイの歌～".to_owned()]));
        assert_eq!(&sheet.files[0].name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
        assert!(sheet.files[0].tracks.is_empty());
        Ok(())
    }
}