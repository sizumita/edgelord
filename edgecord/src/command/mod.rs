mod choice;
mod context;
pub mod i18n;
pub mod option;

use futures::future::LocalBoxFuture;
use std::collections::HashMap;
use std::rc::Rc;
use twilight_model::application::interaction::ApplicationCommand;

use crate::InteractionResponse;
pub use choice::*;
pub use context::*;

type I18nMap = Option<HashMap<i18n::Locales, String>>;
type AsyncCommandFn<'a> = Rc<
    dyn 'a
        + Fn(
            ChatInputCommandContext,
            Box<ApplicationCommand>,
        ) -> LocalBoxFuture<'a, worker::Result<worker::Response>>,
>;

/**
Discord Chat Input Command Structure.
 **/
#[derive(Clone)]
pub struct Command<'a> {
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,

    pub options: Vec<CommandOption>,

    pub action: AsyncCommandFn<'a>,
}

#[derive(Clone)]
pub struct CommandOption {
    pub name: String,
    pub description: String,
    pub i18n_names: I18nMap,
    pub i18n_descriptions: I18nMap,
}

impl<'a> Command<'a> {
    pub async fn invoke(
        &self,
        ctx: ChatInputCommandContext,
        interaction: Box<ApplicationCommand>,
    ) -> InteractionResponse {
        (self.action)(ctx, interaction).await
    }
}
