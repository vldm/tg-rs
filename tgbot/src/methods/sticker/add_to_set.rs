use crate::{
    methods::Method,
    request::{Form, RequestBuilder},
    types::{InputFile, Integer, MaskPosition},
};
use failure::Error;

/// Add a new sticker to a set created by the bot
#[derive(Debug)]
pub struct AddStickerToSet {
    form: Form,
}

impl AddStickerToSet {
    /// Creates a new AddStickerToSet
    ///
    /// # Arguments
    ///
    /// * user_id - User identifier of sticker set owner
    /// * name - Sticker set name
    /// * png_sticker - Png image with the sticker, must be up to 512 kilobytes in size,
    ///                 dimensions must not exceed 512px,
    ///                 and either width or height must be exactly 512px
    /// * emojis - One or more emoji corresponding to the sticker
    pub fn new<N, P, E>(user_id: Integer, name: N, png_sticker: P, emojis: E) -> Self
    where
        N: Into<String>,
        P: Into<InputFile>,
        E: Into<String>,
    {
        let mut form = Form::new();
        form.insert_field("user_id", user_id);
        form.insert_field("name", name.into());
        form.insert_field("png_sticker", png_sticker.into());
        form.insert_field("emojis", emojis.into());
        AddStickerToSet { form }
    }

    /// Position where the mask should be placed on faces
    pub fn mask_position(mut self, value: MaskPosition) -> Result<Self, Error> {
        let value = serde_json::to_string(&value)?;
        self.form.insert_field("mask_position", value);
        Ok(self)
    }
}

impl Method for AddStickerToSet {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::form("addStickerToSet", self.form)
    }
}
