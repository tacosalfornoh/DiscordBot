// ! Module: database
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection, Database};
use mongodb::error::Error;
use crate::custom_fn::dbprintln;
use crate::templates::guild::document as doc_guild;
use crate::templates::member::document as doc_member;

static URI: &str = "mongodb+srv://root:qq68T*LLkxXvXCQ@albionguildmanagerdsbot.gh6amky.mongodb.net/?retryWrites=true&w=majority&appName=AlbionGuildManagerDSBot";

pub(crate) async fn db_connection() -> Result<Database, Error> {
    let result = Ok(Client::with_uri_str(URI).await?.database("root"));
    result
}

pub(crate) async fn db_insert_guild(guild_id: i64, guild_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("guilds");

    match collection.find_one(doc! { "guild_id": guild_id }, None).await? {
        Some(_) => {
            dbprintln("Guild already exists");
            Ok(())
        },
        None => {
            let guild = doc_guild(guild_id, guild_name);
            collection.insert_one(guild, None).await?;
            dbprintln("Guild inserted");
            Ok(())
        },
    }
}

pub(crate) async fn db_delete_guild(guild_id: i64) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("guilds");

    match collection.find_one(doc! { "guild_id": guild_id }, None).await? {
        Some(_) => {
            collection.delete_one(doc! { "guild_id": guild_id }, None).await?;
            dbprintln("Guild deleted");
            Ok(())
        },
        None => {
            dbprintln("Guild does not exist");
            Ok(())
        },
    }
}

pub(crate) async fn db_update_guild(guild_id: i64, guild_name: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("guilds");

    match collection.find_one(doc! { "guild_id": guild_id }, None).await? {
        Some(_) => {
            collection.update_one(doc! { "guild_id": guild_id }, doc! { "$set": { "guild_name": guild_name } }, None).await?;
            dbprintln("Guild updated");
            Ok(())
        },
        None => {
            dbprintln("Guild does not exist");
            Ok(())
        },
    }
}

pub(crate) async fn db_insert_member(member_id: i64, member_name: &str, join_date: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("members");

    match collection.find_one(doc! { "member_id": member_id }, None).await? {
        Some(_) => {
            dbprintln("Member already exists");
            Ok(())
        },
        None => {
            let member = doc_member(member_id, member_name, join_date);
            collection.insert_one(member, None).await?;
            dbprintln("Member inserted");
            Ok(())
        },
    }
}
pub(crate) async fn db_update_member_experience(member_id: i64, experience: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("members");

    match collection.find_one(doc! { "member_id": member_id }, None).await? {
        Some(member) => {
            collection.update_one(doc! { "member_id": member_id }, doc! { "$set": { "experience": experience } }, None).await?;
            dbprintln("Member experience updated");
            Ok(())
        },
        None => {
            dbprintln("Member does not exist");
            Ok(())
        },
    }
}

pub(crate) async fn db_find_member_experience(member_id: i64) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("members");

    match collection.find_one(doc! { "member_id": member_id }, None).await? {
        Some(member) => {
            if let Some(experience) = member.get_i32("experience").ok() {
                dbprintln("Member experience found");
                Ok(experience)
            } else {
                dbprintln("Experience field not found");
                Ok(0)
            }
        },
        None => {
            dbprintln("Member does not exist");
            Ok(0)
        },
    }
}

pub(crate) async fn db_update_member_level(member_id: i64, level: i32) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection: Collection<Document> = db_connection().await?.collection("members");

    match collection.find_one(doc! { "member_id": member_id }, None).await? {
        Some(_) => {
            collection.update_one(doc! { "member_id": member_id }, doc! { "$set": { "level": level } }, None).await?;
            dbprintln("Member level updated");
            Ok(())
        },
        None => {
            dbprintln("Member does not exist");
            Ok(())
        },
    }
}