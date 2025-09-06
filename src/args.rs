use clap::Args;
use std::str::FromStr;
use thiserror::Error;

/// Shared arguments available to all commands
#[derive(Args, Debug)]
pub struct GlobalArgs {
    /// Path to configuration file (supports .json, .yaml, .yml)
    #[arg(short = 'C', long, global = true, default_value = "deckard.yaml")]
    pub config: String,

    /// Increase logging verbosity (can be used multiple times: -vvv or -v -v -v)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Set log level (syslog-style: emergency, alert, critical, error, warning, notice, info, debug)
    #[arg(short = 'L', long, global = true, value_parser = parse_log_level)]
    pub log_level: Option<LogLevel>,
}

/// Syslog-style log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LogLevel {
    Emergency = 0,
    Alert = 1,
    Critical = 2,
    #[default]
    Error = 3,
    Warning = 4,
    Notice = 5,
    Info = 6,
    Debug = 7,
}

impl LogLevel {
    /// Convert to tracing filter string
    pub fn as_filter(&self) -> &'static str {
        match self {
            LogLevel::Emergency | LogLevel::Alert | LogLevel::Critical | LogLevel::Error => "error",
            LogLevel::Warning => "warn",
            LogLevel::Notice | LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }

    /// Increment log level by n steps (capped at Debug)
    pub fn increment(self, n: u8) -> Self {
        let current = self as u8;
        let new_level = current.saturating_add(n).min(LogLevel::Debug as u8);
        Self::from_numeric(new_level).unwrap_or(LogLevel::Debug)
    }

    /// Create from numeric value (0-7)
    pub fn from_numeric(n: u8) -> Option<Self> {
        match n {
            0 => Some(LogLevel::Emergency),
            1 => Some(LogLevel::Alert),
            2 => Some(LogLevel::Critical),
            3 => Some(LogLevel::Error),
            4 => Some(LogLevel::Warning),
            5 => Some(LogLevel::Notice),
            6 => Some(LogLevel::Info),
            7 => Some(LogLevel::Debug),
            _ => None,
        }
    }
}

impl FromStr for LogLevel {
    type Err = LogLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing as number first
        if let Ok(n) = s.parse::<u8>() {
            return LogLevel::from_numeric(n).ok_or(LogLevelParseError::InvalidNumeric(n));
        }

        // Parse as string (case-insensitive)
        match s.to_lowercase().as_str() {
            "emergency" | "emerg" => Ok(LogLevel::Emergency),
            "alert" => Ok(LogLevel::Alert),
            "critical" | "crit" => Ok(LogLevel::Critical),
            "error" | "err" => Ok(LogLevel::Error),
            "warning" | "warn" => Ok(LogLevel::Warning),
            "notice" => Ok(LogLevel::Notice),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            _ => Err(LogLevelParseError::InvalidName(s.to_string())),
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            LogLevel::Emergency => "emergency",
            LogLevel::Alert => "alert",
            LogLevel::Critical => "critical",
            LogLevel::Error => "error",
            LogLevel::Warning => "warning",
            LogLevel::Notice => "notice",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        };
        write!(f, "{}", s)
    }
}

#[derive(Error, Debug)]
pub enum LogLevelParseError {
    #[error("Invalid log level name: {0}")]
    InvalidName(String),
    #[error("Invalid log level number: {0} (must be 0-7)")]
    InvalidNumeric(u8),
}

/// Parse log level from string (for clap)
fn parse_log_level(s: &str) -> Result<LogLevel, String> {
    LogLevel::from_str(s).map_err(|e| e.to_string())
}

/// Determine the effective log level from arguments
pub fn effective_log_level(args: &GlobalArgs) -> LogLevel {
    // Start with explicit log level or default
    let base_level = args.log_level.unwrap_or_default();

    // Increment by verbose count
    if args.verbose > 0 {
        base_level.increment(args.verbose)
    } else {
        base_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_numeric() {
        assert_eq!(LogLevel::from_numeric(0), Some(LogLevel::Emergency));
        assert_eq!(LogLevel::from_numeric(1), Some(LogLevel::Alert));
        assert_eq!(LogLevel::from_numeric(2), Some(LogLevel::Critical));
        assert_eq!(LogLevel::from_numeric(3), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_numeric(4), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_numeric(5), Some(LogLevel::Notice));
        assert_eq!(LogLevel::from_numeric(6), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_numeric(7), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_numeric(8), None);
        assert_eq!(LogLevel::from_numeric(255), None);
    }

    #[test]
    fn test_log_level_from_str() {
        // Test string names (various cases)
        assert_eq!(
            LogLevel::from_str("emergency").unwrap(),
            LogLevel::Emergency
        );
        assert_eq!(
            LogLevel::from_str("EMERGENCY").unwrap(),
            LogLevel::Emergency
        );
        assert_eq!(
            LogLevel::from_str("Emergency").unwrap(),
            LogLevel::Emergency
        );
        assert_eq!(LogLevel::from_str("emerg").unwrap(), LogLevel::Emergency);

        assert_eq!(LogLevel::from_str("alert").unwrap(), LogLevel::Alert);
        assert_eq!(LogLevel::from_str("ALERT").unwrap(), LogLevel::Alert);

        assert_eq!(LogLevel::from_str("critical").unwrap(), LogLevel::Critical);
        assert_eq!(LogLevel::from_str("crit").unwrap(), LogLevel::Critical);

        assert_eq!(LogLevel::from_str("error").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::from_str("err").unwrap(), LogLevel::Error);

        assert_eq!(LogLevel::from_str("warning").unwrap(), LogLevel::Warning);
        assert_eq!(LogLevel::from_str("warn").unwrap(), LogLevel::Warning);

        assert_eq!(LogLevel::from_str("notice").unwrap(), LogLevel::Notice);
        assert_eq!(LogLevel::from_str("info").unwrap(), LogLevel::Info);
        assert_eq!(LogLevel::from_str("debug").unwrap(), LogLevel::Debug);

        // Test numeric strings
        assert_eq!(LogLevel::from_str("0").unwrap(), LogLevel::Emergency);
        assert_eq!(LogLevel::from_str("3").unwrap(), LogLevel::Error);
        assert_eq!(LogLevel::from_str("7").unwrap(), LogLevel::Debug);

        // Test invalid inputs
        assert!(LogLevel::from_str("invalid").is_err());
        assert!(LogLevel::from_str("8").is_err());
        assert!(LogLevel::from_str("999").is_err());
        assert!(LogLevel::from_str("").is_err());
    }

    #[test]
    fn test_log_level_increment() {
        // Test incrementing from Emergency
        assert_eq!(LogLevel::Emergency.increment(0), LogLevel::Emergency);
        assert_eq!(LogLevel::Emergency.increment(1), LogLevel::Alert);
        assert_eq!(LogLevel::Emergency.increment(3), LogLevel::Error);
        assert_eq!(LogLevel::Emergency.increment(7), LogLevel::Debug);
        assert_eq!(LogLevel::Emergency.increment(10), LogLevel::Debug); // Capped at Debug

        // Test incrementing from Warning
        assert_eq!(LogLevel::Warning.increment(0), LogLevel::Warning);
        assert_eq!(LogLevel::Warning.increment(1), LogLevel::Notice);
        assert_eq!(LogLevel::Warning.increment(2), LogLevel::Info);
        assert_eq!(LogLevel::Warning.increment(3), LogLevel::Debug);
        assert_eq!(LogLevel::Warning.increment(100), LogLevel::Debug); // Capped at Debug

        // Test incrementing from Debug (already at max)
        assert_eq!(LogLevel::Debug.increment(0), LogLevel::Debug);
        assert_eq!(LogLevel::Debug.increment(1), LogLevel::Debug);
        assert_eq!(LogLevel::Debug.increment(255), LogLevel::Debug);
    }

    #[test]
    fn test_log_level_as_filter() {
        assert_eq!(LogLevel::Emergency.as_filter(), "error");
        assert_eq!(LogLevel::Alert.as_filter(), "error");
        assert_eq!(LogLevel::Critical.as_filter(), "error");
        assert_eq!(LogLevel::Error.as_filter(), "error");
        assert_eq!(LogLevel::Warning.as_filter(), "warn");
        assert_eq!(LogLevel::Notice.as_filter(), "info");
        assert_eq!(LogLevel::Info.as_filter(), "info");
        assert_eq!(LogLevel::Debug.as_filter(), "debug");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Emergency.to_string(), "emergency");
        assert_eq!(LogLevel::Alert.to_string(), "alert");
        assert_eq!(LogLevel::Critical.to_string(), "critical");
        assert_eq!(LogLevel::Error.to_string(), "error");
        assert_eq!(LogLevel::Warning.to_string(), "warning");
        assert_eq!(LogLevel::Notice.to_string(), "notice");
        assert_eq!(LogLevel::Info.to_string(), "info");
        assert_eq!(LogLevel::Debug.to_string(), "debug");
    }

    #[test]
    fn test_effective_log_level() {
        // Test with no log level and no verbose
        let args = GlobalArgs {
            config: String::from("deckard.yaml"),
            verbose: 0,
            log_level: None,
        };
        assert_eq!(effective_log_level(&args), LogLevel::Error); // Default

        // Test with explicit log level, no verbose
        let args = GlobalArgs {
            config: String::from("deckard.yaml"),
            verbose: 0,
            log_level: Some(LogLevel::Warning),
        };
        assert_eq!(effective_log_level(&args), LogLevel::Warning);

        // Test with no log level, with verbose
        let args = GlobalArgs {
            config: String::from("deckard.yaml"),
            verbose: 3,
            log_level: None,
        };
        assert_eq!(effective_log_level(&args), LogLevel::Info); // Error + 3 = Info

        // Test with log level and verbose
        let args = GlobalArgs {
            config: String::from("deckard.yaml"),
            verbose: 2,
            log_level: Some(LogLevel::Warning),
        };
        assert_eq!(effective_log_level(&args), LogLevel::Info); // Warning + 2 = Info

        // Test capping at Debug
        let args = GlobalArgs {
            config: String::from("deckard.yaml"),
            verbose: 10,
            log_level: Some(LogLevel::Warning),
        };
        assert_eq!(effective_log_level(&args), LogLevel::Debug); // Capped at Debug
    }
}
