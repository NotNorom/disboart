use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use poise::serenity_prelude::{timestamp::InvalidTimestamp, ChannelId, GuildId, Timestamp, UserId};

#[derive(Debug, PartialEq, Eq)]
pub struct Streak {
    pub time: Timestamp,
    pub user: UserId,
    pub channel: ChannelId,
    pub guild: GuildId,
}

impl FromStr for Streak {
    type Err = StreakError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        let mut kv_parts = HashMap::with_capacity(parts.len());

        for part in parts {
            let (key, value) = part.split_once('_').ok_or(StreakError::Decode)?;
            kv_parts.insert(key, value);
        }

        let time = Timestamp::from_unix_timestamp(
            kv_parts
                .get("time")
                .ok_or(StreakError::MissingTime)?
                .parse::<i64>()?,
        )?;
        let user = kv_parts
            .get("user")
            .ok_or(StreakError::MissingUser)?
            .parse::<u64>()?
            .into();
        let channel = kv_parts
            .get("channel")
            .ok_or(StreakError::MissingChannel)?
            .parse::<u64>()?
            .into();
        let guild = kv_parts
            .get("guild")
            .ok_or(StreakError::MissingGuild)?
            .parse::<u64>()?
            .into();

        Ok(Self {
            time,
            user,
            channel,
            guild,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StreakLog {
    pub streaks: Vec<Streak>,
}

impl FromStr for StreakLog {
    type Err = StreakError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut streaks = Vec::new();
        for parsed_line in s.lines().map(FromStr::from_str) {
            let streak = parsed_line?;
            streaks.push(streak);
        }

        Ok(Self { streaks })
    }
}


#[derive(Debug, thiserror::Error)]
pub enum StreakError {
    #[error("decode")]
    Decode,
    #[error("invalid timestamp: {0}")]
    InvalidTimestamp(#[from] InvalidTimestamp),
    #[error("value not a digit: {0}")]
    ValueNotADigit(#[from] ParseIntError),
    #[error("missing time")]
    MissingTime,
    #[error("missing user")]
    MissingUser,
    #[error("missing channel")]
    MissingChannel,
    #[error("missing guild")]
    MissingGuild,
}

#[cfg(test)]
mod test_streak {
    use super::*;

    #[test]
    fn deserialization() -> Result<(), StreakError> {
        let raw_log = "time_1685653778.user_160518747713437696.channel_158390767923101706.guild_116969616370040841";
        let log = StreakLog::from_str(raw_log)?;

        let should_be = StreakLog {
            streaks: vec![Streak {
                time: Timestamp::from_unix_timestamp(1685653778).unwrap(),
                user: UserId(160518747713437696),
                channel: ChannelId(158390767923101706),
                guild: GuildId(116969616370040841),
            }],
        };

        assert_eq!(log, should_be);
        Ok(())
    }
}
