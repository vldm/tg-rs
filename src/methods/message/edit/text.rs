use crate::methods::method::*;
use crate::types::{ChatId, EditMessageResult, InlineKeyboardMarkup, Integer, ParseMode};
use serde::Serialize;

/// Edit text and game messages sent by the bot or via the bot (for inline bots)
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageText {
    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * text - New text of the message
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, message_id: Integer, text: S) -> Self {
        EditMessageText {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageText
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * text - New text of the message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, text: S) -> Self {
        EditMessageText {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Parse mode
    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    /// Disables link previews for links in this message
    pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }

    /// Inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(&mut self, reply_markup: I) -> &mut Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageText {
    type Response = EditMessageResult;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("editMessageText"),
            body: RequestBody::json(&self)?,
        })
    }
}
