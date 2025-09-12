use crate::{schema::users, store::Store};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct User{
    pub id: String,
    pub username: String,
    pub password: String,
}

impl Store {
    pub fn signup_user(&mut self, username: String, password: String) -> Result<String, diesel::result::Error>{
        let u: User = User {
            id: Uuid::new_v4().to_string(),
            username: username,
            password: password,
        };
        let result = diesel::insert_into(users::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new user");

        Ok(result.id.to_string())
    }

    pub fn signin_user(&self, username: String, password: String) -> bool{

        true
    }
}