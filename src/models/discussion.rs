use diesel::prelude::*;
use crate::schema::discussions;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::discussions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Discussion {
    pub id: Uuid,
    pub body: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(AsChangeset)]
#[derive(Insertable)]
#[diesel(table_name = discussions)]
pub struct NewDiscussion<'a> {
    pub id: Uuid,
    pub body: &'a str,
}   