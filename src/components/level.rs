use crate::handlers::database::{db_find_member_experience, db_update_member_experience, db_update_member_level};
const MAX_LEVEL: u8 = 100;
const MAX_EXPERIENCE: u32 = 2147483647;

pub(crate) async fn add_experience(member_id: i64) {
    // Retrieve current experience
    let current_experience = db_find_member_experience(member_id).await.expect("Failed to find member experience");

    // Calculate experience gain based on message count
    let experience = current_experience.saturating_add(1);

    println!("User has gained {} experience", experience);
    db_update_member_experience(member_id, experience).await.expect("Failed to update member experience");
    level_up(member_id, experience).await;
}

async fn level_up(member_id: i64, experience: i32) {
    // Calculate the level based on experience using linear scaling
    let level = ((experience as f32 / MAX_EXPERIENCE as f32) * MAX_LEVEL as f32).round() as u8;

    // Ensure the level is not less than 0 and does not exceed MAX_LEVEL
    let level = level.clamp(0, MAX_LEVEL);


    println!("User has leveled up to level {} ({} experience)", level, experience);

    db_update_member_level(member_id, level as i32).await.expect("Failed to update member level");
}