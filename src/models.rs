use crate::schema::memos;
use crate::schema::memos::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Result<T> = std::result::Result<T, diesel::result::Error>;

#[derive(Queryable)]
pub struct Memo {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "memos"]
pub struct NewMemo<'a> {
    pub body: &'a str,
}

pub fn db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub fn all_memos(conn: &PgConnection) -> Result<Vec<Memo>> {
    memos.load(conn)
}

pub fn find_memo(memo_id: i32, conn: &PgConnection) -> Result<Option<Memo>> {
    memos.find(memo_id).first(conn).optional()
}

pub fn insert_new_memo(memo_body: &str, conn: &PgConnection) -> Result<()> {
    let new_memo = NewMemo { body: memo_body };
    diesel::insert_into(memos).values(&new_memo).execute(conn)?;
    Ok(())
}

pub fn update_memo_body(memo_id: i32, memo_body: &str, conn: &PgConnection) -> Result<()> {
    diesel::update(memos.find(memo_id))
        .set(body.eq(memo_body))
        .execute(conn)?;
    Ok(())
}

pub fn remove_memo(memo_id: i32, conn: &PgConnection) -> Result<()> {
    diesel::delete(memos.find(memo_id)).execute(conn)?;
    Ok(())
}
