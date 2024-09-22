use crate::Document;
use mongodb::bson::doc;

pub fn create_guild(guild_name: &str, guild_id: &str) -> Document {
    doc! {
        "guild_name": guild_name,
        "guild_idd": guild_id,
    }
}
