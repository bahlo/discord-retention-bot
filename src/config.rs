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
            .ok_or(ParseChannelConfigError::NoDurationSuffix)?
        {
            'd' => Ok(Duration::days(channel_duration_str.parse::<i64>()?)),
            'w' => Ok(Duration::weeks(channel_duration_str.parse::<i64>()?)),
            other => Err(ParseChannelConfigError::InvalidDurationSuffix(other)),
        }?;
        channel_retention.insert(channel_name, channel_duration);
    }
    Ok(channel_retention)
}
