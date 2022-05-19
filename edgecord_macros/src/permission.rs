use darling::util::Flag;
use twilight_model::guild::Permissions;

macro_rules! flag_bits {
    ($($fname:ident),*) => {
        #[derive(Default, Debug, darling::FromMeta)]
        pub struct PermissionFlagBits {
            pub $($fname : Flag),*
        }

        impl PermissionFlagBits {
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
    create_invite,
    kick_members,
    ban_members,
    administrator,
    manage_channels,
    manage_guild,
    add_reactions,
    view_audit_log,
    priority_speaker,
    stream,
    view_channel,
    send_messages,
    send_tts_messages,
    manage_messages,
    embed_links,
    attach_files,
    read_message_history,
    mention_everyone,
    use_external_emojis,
    view_guild_insights,
    connect,
    speak,
    mute_members,
    deafen_members,
    move_members,
    use_vad,
    change_nickname,
    manage_nicknames,
    manage_roles,
    manage_webhooks,
    manage_emojis_and_stickers,
    request_to_speak,
    manage_events,
    manage_threads,
    create_public_threads,
    create_private_threads,
    use_external_stickers,
    send_messages_in_threads,
    use_embedded_activities,
    moderate_members
}
