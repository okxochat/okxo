//! Builds the SlateDB keys for a channel.
//!
//! Layout (docs/architecture.md#keys):
//!   ch/{cid}/m/{seq:020}            one message
//!   ch/{cid}/head                   the channel's latest seq
//!   ch/{cid}/idem/{client_msg_id}   duplicate-send detection

use crate::ChannelId;
use std::fmt;

/// A SlateDB key. The field is private, so only this module can create one.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Key {
    bytes: Vec<u8>,
}

/// Key for one message: `ch/{cid}/m/{seq:020}`.
pub fn msg(ch: ChannelId, seq: u64) -> Key {
    Key {
        bytes: format!("ch/{ch}/m/{seq:020}").into_bytes(),
    }
}

/// Key holding the channel's latest seq.
pub fn head(ch: ChannelId) -> Key {
    Key {
        bytes: format!("ch/{ch}/head").into_bytes(),
    }
}

/// Key mapping a client's message id to the seq it was stored under.
pub fn idem(ch: ChannelId, client_msg_id: &str) -> Key {
    Key {
        bytes: format!("ch/{ch}/idem/{client_msg_id}").into_bytes(),
    }
}

// Lets SlateDB read a key's bytes without being able to change them.
impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

// Prints keys as text, so failing tests show `Key(ch/.../head)` instead of raw bytes.
impl fmt::Debug for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key({})", String::from_utf8_lossy(&self.bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Zero-padding keeps message keys in seq order (9 before 10).
    #[test]
    fn message_keys_sort_by_seq() {
        let ch = ChannelId::new();
        assert!(msg(ch, 9) < msg(ch, 10));
        assert!(msg(ch, 99) < msg(ch, 100));
    }

    // The message key has exactly the documented layout.
    #[test]
    fn message_key_format() {
        let ch = ChannelId::new();
        let key = msg(ch, 7);
        let string = String::from_utf8(key.bytes).unwrap();
        let expected = format!("ch/{ch}/m/00000000000000000007");
        assert_eq!(expected, string);
    }

    // as_ref exposes the raw key bytes.
    #[test]
    fn key_as_bytes() {
        let ch = ChannelId::new();
        let key = head(ch);
        let bytes: &[u8] = key.as_ref();
        assert!(bytes.starts_with(b"ch/"));
    }

    // The same channel always gets the same head key.
    #[test]
    fn head_key_is_stable() {
        let ch = ChannelId::new();
        assert_eq!(head(ch), head(ch));
    }
}
