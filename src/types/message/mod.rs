use crate::types::chat::Chat;
use crate::types::message::raw::RawMessage;
use crate::types::primitive::Integer;
use serde::{de::Error, Deserialize, Deserializer};

mod data;
mod forward;
mod kind;
mod raw;
#[cfg(test)]
mod tests;
mod text;

pub use self::data::*;
pub use self::forward::*;
pub use self::kind::*;
pub(crate) use self::raw::RawMessageEntity;
pub use self::text::*;

/// This object represents a message
#[derive(Clone, Debug)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub id: Integer,
    /// Date the message was sent in Unix time
    pub date: Integer,
    /// Contains chat-specific data
    pub kind: MessageKind,
    /// Forwarded data
    pub forward: Option<Forward>,
    /// For replies, the original message
    /// Note that the Message object in this field will not contain further
    /// reply_to fields even if it itself is a reply
    pub reply_to: Option<Box<Message>>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<Integer>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<String>,
    /// Contains message data
    pub data: MessageData,
}

impl Message {
    fn from_raw(raw: RawMessage) -> Result<Message, String> {
        macro_rules! required {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(format!("\"{}\" field is missing", stringify!($name))),
                }
            }};
        };

        let forward_info = match (
            raw.forward_date,
            raw.forward_from,
            raw.forward_from_chat,
            raw.forward_from_message_id,
            raw.forward_signature,
        ) {
            (Some(date), Some(user), None, None, None) => Some(Forward {
                date,
                from: ForwardFrom::User(user),
            }),
            (Some(date), None, Some(Chat::Channel(chat)), Some(message_id), signature) => {
                Some(Forward {
                    date,
                    from: ForwardFrom::Channel {
                        chat,
                        message_id,
                        signature,
                    },
                })
            }
            (None, None, None, None, None) => None,
            _ => return Err(String::from("Unexpected forward_* fields combination")),
        };

        let message_kind = match raw.chat {
            Chat::Channel(chat) => MessageKind::Channel {
                chat,
                author_signature: raw.author_signature,
            },
            Chat::Group(chat) => MessageKind::Group {
                chat,
                from: required!(from),
            },
            Chat::Private(chat) => MessageKind::Private {
                chat,
                from: required!(from),
            },
            Chat::Supergroup(chat) => MessageKind::Supergroup {
                chat,
                from: required!(from),
            },
        };

        let caption = match raw.caption {
            Some(data) => Some(Text {
                data,
                entities: raw.caption_entities,
            }),
            None => None,
        };

        let reply_to_message = match raw.reply_to_message {
            Some(x) => Some(Box::new(Message::from_raw(*x)?)),
            None => None,
        };

        macro_rules! message_data {
            ($variant:ident($attr:ident)) => {
                if let Some(data) = raw.$attr {
                    message_data!(MessageData::$variant(data));
                }
            };
            ($variant:ident($attr:ident, caption)) => {
                if let Some(data) = raw.$attr {
                    message_data!(MessageData::$variant { caption, data });
                }
            };
            ($variant:ident($attr:ident, flag)) => {
                if raw.$attr.unwrap_or(false) {
                    message_data!(MessageData::$variant);
                }
            };
            ($data:expr) => {
                return Ok(Message {
                    id: raw.message_id,
                    date: raw.date,
                    kind: message_kind,
                    forward: forward_info,
                    reply_to: reply_to_message,
                    edit_date: raw.edit_date,
                    media_group_id: raw.media_group_id,
                    data: $data,
                });
            };
        };

        message_data!(Animation(animation));
        message_data!(Audio(audio, caption));
        message_data!(ChannelChatCreated(channel_chat_created, flag));
        message_data!(ConnectedWebsite(connected_website));
        message_data!(Contact(contact));
        message_data!(DeleteChatPhoto(delete_chat_photo, flag));
        message_data!(Document(document, caption));
        message_data!(Game(game));
        message_data!(GroupChatCreated(group_chat_created, flag));
        message_data!(Invoice(invoice));
        message_data!(LeftChatMember(left_chat_member));
        message_data!(Location(location));
        message_data!(MigrateFromChatId(migrate_from_chat_id));
        message_data!(MigrateToChatId(migrate_to_chat_id));
        message_data!(NewChatMembers(new_chat_members));
        message_data!(NewChatPhoto(new_chat_photo));
        message_data!(NewChatTitle(new_chat_title));
        message_data!(PassportData(passport_data));
        message_data!(Photo(photo, caption));
        message_data!(Sticker(sticker));
        message_data!(SuccessfulPayment(successful_payment));
        message_data!(SupergroupChatCreated(supergroup_chat_created, flag));
        message_data!(Venue(venue));
        message_data!(Video(video, caption));
        message_data!(VideoNote(video_note));
        message_data!(Voice(voice, caption));

        if let Some(data) = raw.pinned_message {
            let data = Message::from_raw(*data)?;
            message_data!(MessageData::PinnedMessage(Box::new(data)));
        }

        if let Some(data) = raw.text {
            message_data!(MessageData::Text(Text {
                data,
                entities: raw.entities,
            }));
        }

        Err(String::from("Can not get message data"))
    }
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_msg: RawMessage = Deserialize::deserialize(deserializer)?;
        Message::from_raw(raw_msg).map_err(D::Error::custom)
    }
}

/// Result of editMessage* requests
#[derive(Clone, Debug, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum EditMessageResult {
    /// Returned if edited message is sent by the bot
    Message(Message),
    /// Returned if edited message is NOT sent by the bot
    Bool(bool),
}
