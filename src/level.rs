use log::Level;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum GelfLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    Error = 3,
    Warning = 4,
    Notice = 5,
    Informational = 6,
    Debugging = 7,
}

impl Default for GelfLevel {
    fn default() -> GelfLevel {
        GelfLevel::Alert
    }
}


impl From<Level> for GelfLevel {
    fn from(level: Level) -> GelfLevel {
        match level {
            Level::Trace => GelfLevel::Debugging,
            Level::Debug => GelfLevel::Debugging,
            Level::Info => GelfLevel::Informational,
            Level::Warn => GelfLevel::Warning,
            Level::Error => GelfLevel::Error
        }
    }
}

impl From<&GelfLevel> for Level {
    fn from(level: &GelfLevel) -> Level {
        match level {
            GelfLevel::Debugging => Level::Debug,
            GelfLevel::Informational => Level::Info,
            GelfLevel::Warning => Level::Warn,
            _ => Level::Error,
        }
    }
}


impl From<GelfLevel> for String {
    fn from(level: GelfLevel) -> String {
        match level {
            GelfLevel::Emergency => "Emergency".to_string(),
            GelfLevel::Alert => "Alert".to_string(),
            GelfLevel::Critical => "Critical".to_string(),
            GelfLevel::Error => "Error".to_string(),
            GelfLevel::Warning => "Warning".to_string(),
            GelfLevel::Notice => "Notice".to_string(),
            GelfLevel::Informational => "Informational".to_string(),
            GelfLevel::Debugging => "Debugging".to_string(),
        }
    }
}
