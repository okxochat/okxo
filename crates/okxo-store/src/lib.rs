use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelID(pub Uuid);

impl ChannelID {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for ChannelID {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ChannelID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.hyphenated())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub channel_id: ChannelID,
    pub seq: u64,
    pub author_id: Uuid,
    pub body: String,
    pub client_msg_id: String,
    pub created_at_ms: i64,
}

#[derive(Debug, Clone)]
pub struct NewMessage {
    pub channel_id: ChannelID,
    pub author_id: Uuid,
    pub body: String,
    pub client_msg_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channel_id_display_is_fixed_width() {
        for _ in 0..100 {
            assert_eq!(ChannelID::new().to_string().len(), 36);
        }
    }
}
