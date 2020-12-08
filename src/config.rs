use anyhow::Result;
use chrono::Duration;
use std::collections::HashMap;

use crate::errors::*;

pub fn parse_channel_retention(input: String) -> Result<HashMap<String, Duration>> {
    let mut channel_retention = HashMap::new();
    for channel in input.split(",") {
        let parts: Vec<&str> = channel.split(":").collect();
        let channel_name = parts
            .get(0)
            .and_then(|str| Some(str.to_string()))
            .ok_or(ParseChannelConfigError::InvalidFormat)?;
        let mut channel_duration_str = parts
            .get(1)
            .and_then(|str| Some(str.to_string()))
            .ok_or(ParseChannelConfigError::InvalidFormat)?;
        let channel_duration = match channel_duration_str
            .pop()
            .ok_or(ParseChannelConfigError::NoDuration)?
        {
            'h' => Ok(Duration::hours(channel_duration_str.parse::<i64>()?)),
            'd' => Ok(Duration::days(channel_duration_str.parse::<i64>()?)),
            'w' => Ok(Duration::weeks(channel_duration_str.parse::<i64>()?)),
            other => Err(ParseChannelConfigError::InvalidDurationSuffix(other)),
        }?;
        channel_retention.insert(channel_name, channel_duration);
    }
    Ok(channel_retention)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_channel_retention_simple() {
        let channel_retention = parse_channel_retention("foo:1h,bar:2d,baz:3w".to_owned()).unwrap();
        assert_eq!(channel_retention.get("foo").unwrap(), &Duration::hours(1));
        assert_eq!(channel_retention.get("bar").unwrap(), &Duration::days(2));
        assert_eq!(channel_retention.get("baz").unwrap(), &Duration::weeks(3));
    }

    #[test]
    fn test_parse_channel_retention_invalid_duration() {
        let result = parse_channel_retention("foo:1z".to_owned());
        if let Err(e) = result {
            match e.downcast_ref::<ParseChannelConfigError>() {
                Some(ParseChannelConfigError::InvalidDurationSuffix('z')) => {} // Ok
                _ => panic!("Expected ParseChannelConfigError::InvalidDurationSuffix"),
            };
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    fn test_parse_channel_retention_invalid_format() {
        let result = parse_channel_retention("foo".to_owned());
        if let Err(e) = result {
            match e.downcast_ref::<ParseChannelConfigError>() {
                Some(ParseChannelConfigError::InvalidFormat) => {} // Ok
                _ => panic!("Expected ParseChannelConfigError::InvalidFormat"),
            };
        } else {
            panic!("Expected error");
        }
    }

    #[test]
    fn test_parse_channel_retention_no_duration() {
        let result = parse_channel_retention("foo:".to_owned());
        if let Err(e) = result {
            print!("{:?}", e);
            match e.downcast_ref::<ParseChannelConfigError>() {
                Some(ParseChannelConfigError::NoDuration) => {} // Ok
                _ => panic!("Expected ParseChannelConfigError::NoDurationSuffix"),
            };
        } else {
            panic!("Expected error");
        }
    }
}
