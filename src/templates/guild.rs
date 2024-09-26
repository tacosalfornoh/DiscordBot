use mongodb::bson::{doc, Document};

pub(crate) fn document(guild_id: i64, guild_name: &str) -> Document {
    doc! {
        "guild_id": guild_id,
        "guild_name": guild_name,
        "chat_level": {
            "enabled": false,
        },
        "ticket": {
            "enabled": false,
            "category_id": "",
            "role_id": "",
        },
        "balance": {
            "enabled": false,
            "role_id": "",
        },
        "join_auto_role": {
            "enabled": false,
            "role_id": "",
        },
        "application": {
            "enabled": false,
            "recruitment": false,
            "category_id": "",
            "role_officer_id": "",
            "role_member_id": "",
        },
    }
}
