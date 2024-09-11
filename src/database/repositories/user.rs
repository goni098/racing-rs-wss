use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::database::entities::user;

pub async fn find_by_telegram_id(
    db: &DatabaseConnection,
    telegram_id: u64,
) -> Result<Option<user::Model>, DbErr> {
    user::Entity::find_by_id(telegram_id).one(db).await
}
