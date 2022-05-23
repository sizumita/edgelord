use darling::util::Flag;
use darling::FromMeta;
use twilight_model::channel::ChannelType;

#[derive(Debug, Clone, darling::FromMeta, Default)]
pub struct ChannelTypes {
    pub guild_text: Flag,
    pub private: Flag,
    pub guild_voice: Flag,
    pub group: Flag,
    pub guild_category: Flag,
    pub guild_news: Flag,
    pub guild_news_thread: Flag,
    pub guild_public_thread: Flag,
    pub guild_private_thread: Flag,
    pub guild_stage_voice: Flag,
    pub guild_directory: Flag,
    pub guild_forum: Flag,
}

impl From<ChannelTypes> for Vec<ChannelType> {
    fn from(types: ChannelTypes) -> Self {
        let mut vec = vec![];
        if types.guild_text.is_present() {
            vec.push(ChannelType::GuildText)
        }
        if types.private.is_present() {
            vec.push(ChannelType::Private)
        }
        if types.guild_voice.is_present() {
            vec.push(ChannelType::GuildVoice)
        }
        if types.group.is_present() {
            vec.push(ChannelType::Group)
        }
        if types.guild_category.is_present() {
            vec.push(ChannelType::GuildCategory)
        }
        if types.guild_news.is_present() {
            vec.push(ChannelType::GuildNews)
        }
        if types.guild_news_thread.is_present() {
            vec.push(ChannelType::GuildNewsThread)
        }
        if types.guild_public_thread.is_present() {
            vec.push(ChannelType::GuildPublicThread)
        }
        if types.guild_stage_voice.is_present() {
            vec.push(ChannelType::GuildStageVoice)
        }
        if types.guild_directory.is_present() {
            vec.push(ChannelType::GuildDirectory)
        }
        if types.guild_forum.is_present() {
            vec.push(ChannelType::GuildForum)
        }

        vec
    }
}

impl ChannelTypes {
    pub fn to_vec_token(&self) -> proc_macro2::TokenStream {
        let mut type_tokens = vec![];
        let types: Vec<ChannelType> = self.clone().into();
        for ty in types {
            let ident = syn::Ident::from_string(ty.name()).unwrap();
            type_tokens.push(quote::quote! {
                ::edgecord::model::channel::ChannelType::#ident
            });
        }
        quote::quote! {
            vec![ #(#type_tokens), * ]
        }
    }
}
