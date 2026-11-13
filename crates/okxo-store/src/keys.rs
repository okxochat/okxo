// HINT: add a `//!` doc comment at the very top describing this file.
//   SlateDB keys are bytes, sorted by comparing bytes left to right.
//   Layout (docs/architecture.md#keys):
//     ch/{cid}/m/{seq:020}            one message
//     ch/{cid}/head                   the channel's latest seq
//     ch/{cid}/idem/{client_msg_id}   duplicate-send detection
use crate::ChannelId;

/// key for one message:
pub fn msg(ch: ChannelId, seq: u64) -> Vec<u8> {
    format!("ch/{ch}/m/{seq:020}").into_bytes()
}

/// Key holding the channel's latest seq
pub fn head(ch: ChannelId) -> Vec<u8> {
    format!("ch/{ch}/head").into_bytes()
}

// TODO: write a `///` doc comment, like the two above.
//
// HINT (signature): public, named `idem`, takes the channel id AND the
//   client's message id (text), and returns bytes like the others.
//   - You only *read* the text, so don't take ownership of it.
//   - The usual type for "borrowed, read-only text" is `&` followed by
//     the basic text type. It is NOT `String`.
//
// HINT (body): the key is "ch/{cid}/idem/{client_msg_id}". Otherwise
//   it's the same as `msg` and `head`.
pub fn idem(ch: ChannelId, client_msg_id: String) -> Vec<u8> {
    format!("ch/{ch}/idem/{client_msg_id}").into_bytes()
}

#[cfg(test)]
mod tests {
    // HINT: import everything from the parent module (the rest of this file).
    //   The parent module is `super`, and "everything" is `*`.

    // Written for you: the checkpoint for `msg`.
    use super::*;
    #[test]
    fn message_keys_sort_by_seq() {
        let ch = ChannelId::new();
        assert!(msg(ch, 9) < msg(ch, 10));
        assert!(msg(ch, 99) < msg(ch, 100));
        // QUESTION: why can we pass `ch` four times? (Look at its derives in lib.rs.)
    }

    // YOUR TURN: write a test called `message_key_format` that proves
    // msg(ch, 7) is exactly "ch/<the channel id>/m/00000000000000000007".
    //
    // HINTS:
    //   - Start with `#[test]` and `fn message_key_format() { ... }`.
    //   - Turn the bytes back into text: `String::from_utf8(bytes)`.
    //     It returns a `Result` (it can fail on invalid text). In tests
    //     it's fine to take the value out with `.unwrap()`.
    //   - Build the expected string with `format!` and the same `ch`.
    //   - Compare them with `assert_eq!(left, right)`.
    #[test]
    fn message_key_format() {
        let ch = ChannelId::new();
        let bytes = msg(ch, 7);
        let string = String::from_utf8(bytes).unwrap();
        let expected = format!("ch/{ch}/m/00000000000000000007");
        assert_eq!(expected, string);
    }

    // STRETCH: write tests for `head` and `idem` the same way.
}
