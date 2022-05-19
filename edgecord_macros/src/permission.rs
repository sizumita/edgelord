use darling::util::Flag;
use twilight_model::guild::Permissions;

macro_rules! flag_bits {
    (struct $name:ident { $($fname:ident : $ftype:ty),* }) => {
        #[derive(Default, Debug, darling::FromMeta)]
        pub struct $name {
            pub $($fname : $ftype),*
        }

        impl $name {
            pub fn bits(&self) -> Permissions {
                macro_rules! flag {
                    ($flag_name:ident) => {
                        paste::item! {
                            if self.$flag_name.is_present() {Permissions::[<$flag_name:upper>].bits()} else {0}
                        }
                    }
                }

                Permissions::from_bits_truncate(
                    $(flag!($fname)) | *
                )
            }
        }
    }
}

flag_bits! {
    struct PermissionFlagBits {
        create_invite: Flag,
        kick_members: Flag,
        ban_members: Flag,
        administrator: Flag,
        manage_channels: Flag,
        manage_guild: Flag,
        add_reactions: Flag,
        view_audit_log: Flag,
        priority_speaker: Flag,
        stream: Flag,
        view_channel: Flag,
        send_messages: Flag,
        send_tts_messages: Flag,
        manage_messages: Flag,
        embed_links: Flag,
        attach_files: Flag,
        read_message_history: Flag,
        mention_everyone: Flag,
        use_external_emojis: Flag,
        view_guild_insights: Flag,
        connect: Flag,
        speak: Flag,
        mute_members: Flag,
        deafen_members: Flag,
        move_members: Flag,
        use_vad: Flag,
        change_nickname: Flag,
        manage_nicknames: Flag,
        manage_roles: Flag,
        manage_webhooks: Flag,
        manage_emojis_and_stickers: Flag,
        request_to_speak: Flag,
        manage_events: Flag,
        manage_threads: Flag,
        create_public_threads: Flag,
        create_private_threads: Flag,
        use_external_stickers: Flag,
        send_messages_in_threads: Flag,
        use_embedded_activities: Flag,
        moderate_members: Flag
    }
}
