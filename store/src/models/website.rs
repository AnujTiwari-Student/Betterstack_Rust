use crate::{schema::websites, store::Store};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use chrono::Utc;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website{
    pub url: String,
    pub id: String,
    pub user_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Store {
    pub fn create_website(&mut self, user_id: String, url: String) -> Result<Website, diesel::result::Error>{
        let w: Website = Website {
            url: url,
            id: Uuid::new_v4().to_string(),
            user_id: user_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        let result = diesel::insert_into(websites::table)
            .values(&w)
            .returning(Website::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new website");

        Ok(result)

    }

    pub fn get_website(&mut self, id: String) -> Result<Website, diesel::result::Error>{
        let website = websites::table
            .filter(websites::id.eq(id))
            .select(Website::as_select())
            .first(&mut self.conn)?;

        if !website.id.is_empty(){
            return Err(diesel::result::Error::NotFound)
        }

        Ok(website)        
    }
}