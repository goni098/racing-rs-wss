use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "mini_app_user")]
pub struct Model {
    #[sea_orm(
        primary_key,
        auto_increment = false,
        column_type = "Decimal(Some((60, 0)))"
    )]
    pub telegram_id: Decimal,
    pub is_premium: bool,
    pub username: Option<String>,
    pub photo_url: Option<String>,
    pub ref_code: String,
    pub ref_by: Option<String>,
    pub fuel_tank_lv: i32,
    pub turbo_changer_lv: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::activity::Entity")]
    Activity,
}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
