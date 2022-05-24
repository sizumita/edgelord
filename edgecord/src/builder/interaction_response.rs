use twilight_model::channel::message::MessageFlags;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};

pub struct InteractionResponseBuilder {
    _content: String,
    _flags: MessageFlags,
}

impl InteractionResponseBuilder {
    pub fn new() -> Self {
        Self {
            _content: String::default(),
            _flags: MessageFlags::empty(),
        }
    }

    pub fn flag(&mut self, flag: MessageFlags) -> &mut Self {
        self._flags = MessageFlags::from_bits_truncate(flag.bits() | self._flags.bits());
        self
    }

    pub fn content<T>(&mut self, content: T) -> &mut Self
    where
        T: ToString,
    {
        self._content = content.to_string();
        self
    }

    pub fn build(&self, kind: InteractionResponseType) -> InteractionResponse {
        InteractionResponse {
            kind,
            data: Some(InteractionResponseData {
                allowed_mentions: None,
                attachments: None,
                choices: None,
                components: None,
                content: if self._content.is_empty() {
                    None
                } else {
                    Some(self._content.clone())
                },
                custom_id: None,
                embeds: None,
                flags: if self._flags.is_empty() {
                    None
                } else {
                    Some(self._flags)
                },
                title: None,
                tts: None,
            }),
        }
    }
}

impl Default for InteractionResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::InteractionResponseBuilder;
    use twilight_model::http::interaction::{
        InteractionResponse, InteractionResponseData, InteractionResponseType,
    };

    #[test]
    fn test_builder() {
        let response = InteractionResponseBuilder::new()
            .content("abc")
            .build(InteractionResponseType::ChannelMessageWithSource);
        assert_eq!(
            response,
            InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(InteractionResponseData {
                    allowed_mentions: None,
                    attachments: None,
                    choices: None,
                    components: None,
                    content: Some("abc".to_string()),
                    custom_id: None,
                    embeds: None,
                    flags: None,
                    title: None,
                    tts: None
                })
            }
        )
    }
}
