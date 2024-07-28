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
    pub root_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::RootId",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    SelfRef,
    #[sea_orm(
        belongs_to = "super::mini_app_user::Entity",
        from = "Column::TelegramId",
        to = "super::mini_app_user::Column::TelegramId",
        on_update = "Cascade",
        on_delete = "Restrict"
    )]
    MiniAppUser,
}

impl Related<super::mini_app_user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MiniAppUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
