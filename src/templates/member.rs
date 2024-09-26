use mongodb::bson::{doc, Document};

pub(crate) fn document(member_id: i64, member_name: &str, join_date: String) -> Document {
    doc! {
        "member_id": member_id,
        "member_name": member_name,
        "balance": 0,
        "level": 0,
        "experience": 0,
        "join_date": join_date,
    }
}
