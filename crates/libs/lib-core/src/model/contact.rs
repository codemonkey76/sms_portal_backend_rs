use lib_utils::time::Rfc3339;
use modql::{field::Fields, filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue}};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::FromRow;
use crate::model::Result;
use time::OffsetDateTime;
use crate::{ctx::Ctx, generate_common_bmc_fns, model::modql_utils::time_to_sea_value};
use crate::model::base;
use super::{base::DbBmc, ModelManager};


#[serde_as]
#[derive(Debug, Clone, Fields, FromRow, Serialize)]
pub struct Contact {
    pub id: i64,
    
    pub phone: String,
    pub first_name: String,
    pub last_name: String,
    pub company_name: String,

    pub customer_id: i64,

    pub cid: i64,
    #[serde_as(as = "Rfc3339")]
    pub ctime: OffsetDateTime,
    pub mid: i64,
    #[serde_as(as = "Rfc3339")]
    pub mtime: OffsetDateTime
}

#[derive(Fields, Deserialize)]
pub struct ContactForCreate {

}

#[derive(Fields, Deserialize)]
pub struct ContactForUpdate {

}

#[derive(FilterNodes, Default, Deserialize)]
pub struct ContactFilter {
    pub id: Option<OpValsInt64>,

    pub phone: Option<OpValsString>,
    pub first_name: Option<OpValsString>,
    pub last_name: Option<OpValsString>,
    pub company_name: Option<OpValsString>,

    pub customer_id: Option<OpValsInt64>,

    pub cid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub ctime: Option<OpValsValue>,
    pub mid: Option<OpValsInt64>,
    #[modql(to_sea_value_fn = "time_to_sea_value")]
    pub mtime: Option<OpValsValue>,
}

pub struct ContactBmc {}

impl DbBmc for ContactBmc {
    const TABLE: &'static str = "contacts";
}

generate_common_bmc_fns!(
    Bmc: ContactBmc,
    Entity: Contact,
    ForCreate: ContactForCreate,
    ForUpdate: ContactForUpdate,
    Filter: ContactFilter
);