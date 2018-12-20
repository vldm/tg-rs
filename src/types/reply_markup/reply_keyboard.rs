use std::ops::Not;

/// This object represents a custom keyboard with reply options
#[derive(Clone, Debug, Serialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ReplyKeyboardMarkup {
    /// Returns an empty KeyboardMarkup
    pub fn new() -> Self {
        ReplyKeyboardMarkup {
            keyboard: Vec::new(),
            resize_keyboard: false,
            one_time_keyboard: false,
            selective: false,
        }
    }

    /// Returns a KeyboardMarkup with given keyboard
    pub fn with_keyboard(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        ReplyKeyboardMarkup {
            keyboard,
            resize_keyboard: false,
            one_time_keyboard: false,
            selective: false,
        }
    }

    /// Requests clients to resize the keyboard vertically for optimal fit
    /// (e.g., make the keyboard smaller if there are just two rows of buttons)
    /// Defaults to false, in which case the custom keyboard
    /// is always of the same height as the app's standard keyboard
    pub fn resize_keyboard(&mut self) -> &mut Self {
        self.resize_keyboard = true;
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user
    /// can press a special button in the input field to see the custom keyboard again
    /// Defaults to false
    pub fn one_time_keyboard(&mut self) -> &mut Self {
        self.one_time_keyboard = true;
        self
    }

    /// Use this parameter if you want to show the keyboard to specific users only
    /// Targets:
    /// 1) users that are @mentioned in the text of the Message object;
    /// 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language
    /// Other users in the group don’t see the keyboard
    pub fn selective(&mut self) -> &mut Self {
        self.selective = true;
        self
    }

    /// Adds a row to keyboard
    pub fn add_row(&mut self, row: Vec<KeyboardButton>) {
        self.keyboard.push(row);
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyKeyboardMarkup {
    fn from(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::with_keyboard(keyboard)
    }
}

/// This object represents one button of the reply keyboard
/// For simple text buttons String can be used instead of this object to specify text of the button
#[derive(Clone, Debug, Serialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Not::not")]
    request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    request_location: bool,
}

impl KeyboardButton {
    /// # Arguments
    ///
    /// * text - Text of the button
    ///          If none of the optional fields are used,
    ///          it will be sent as a message when the button is pressed
    pub fn new<S: Into<String>>(text: S) -> Self {
        KeyboardButton {
            text: text.into(),
            request_contact: false,
            request_location: false,
        }
    }

    /// The user's phone number will be sent as a contact when the button is pressed
    /// Available in private chats only
    pub fn with_request_contact(mut self) -> Self {
        self.request_contact = true;
        self.request_location = false;
        self
    }

    /// The user's current location will be sent when the button is pressed
    /// Available in private chats only
    pub fn with_request_location(mut self) -> Self {
        self.request_location = true;
        self.request_contact = false;
        self
    }
}

/// Requests clients to remove the custom keyboard
/// (user will not be able to summon this keyboard;
/// if you want to hide the keyboard from sight but keep it accessible,
/// use one_time_keyboard in ReplyKeyboardMarkup)
#[derive(Clone, Debug, Serialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    /// Returns an new keyboard
    pub fn new() -> Self {
        ReplyKeyboardRemove {
            remove_keyboard: true,
            selective: None,
        }
    }

    /// Use this parameter if you want to remove the keyboard for specific users only
    /// Targets:
    /// 1) users that are @mentioned in the text of the Message object;
    /// 2) if the bot's message is a reply (has reply_to_message_id), sender of the original message
    /// Example: A user votes in a poll, bot returns confirmation message
    /// in reply to the vote and removes the keyboard for that user,
    /// while still showing the keyboard with poll options to users who haven't voted yet
    pub fn with_selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }
}
