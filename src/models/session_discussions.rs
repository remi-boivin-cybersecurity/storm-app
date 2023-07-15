use diesel::prelude::*;
use crate::schema::{session_discussions};
use crate::models::discussion::Discussion;
use crate::models::session::Session;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Associations, Debug)]
// #[derive(Identifiable, Selectable, Queryable, Associations)]
#[diesel(belongs_to(Session))]
#[diesel(belongs_to(Discussion))]
#[diesel(table_name = session_discussions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SessionDiscussions {
    pub id: Uuid,
    pub session_id: Uuid,
    pub discussion_id: Uuid,

    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(AsChangeset)]
#[derive(Insertable)]
// #[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
// #[diesel(belongs_to(Session))]
// #[diesel(belongs_to(Discussion))]
#[diesel(table_name = session_discussions)]
pub struct NewSessionDiscussions {
    pub id: Uuid,
    pub session_id: Uuid,
    pub discussion_id: Uuid,

    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}