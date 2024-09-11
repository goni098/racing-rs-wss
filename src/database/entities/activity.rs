use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "activity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Double")]
    pub points: f64,
    pub activity: String,
    pub date: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Decimal(Some((60, 0)))")]
    pub telegram_id: Decimal,
    #[sea_orm(column_type = "Decimal(Some((60, 0)))", nullable)]
    pub friend_id: Option<Decimal>,
    #[sea_orm(unique)]
    pub root_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::TelegramId",
        to = "super::user::Column::TelegramId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
