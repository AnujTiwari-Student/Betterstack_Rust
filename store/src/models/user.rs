use crate::{schema::users, store::Store};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

pub struct SigninResult {
    pub success: bool,
    pub id: Option<String>,
}

impl Store {
    pub fn signup_user(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {
        let u: User = User {
            id: Uuid::new_v4().to_string(),
            username,
            password,
        };
        let result = diesel::insert_into(users::table)
            .values(&u)
            .returning(User::as_returning())
            .get_result(&mut self.conn)
            .expect("Error saving new user");

        Ok(result.id.to_string())
    }

    pub fn signin_user(
        &mut self,
        username: String,
        password: String,
    ) -> Result<String, diesel::result::Error> {

        println!("Looking for username: '{}'", username);

        let user = users::table
            .filter(users::username.eq(username))
            .select(User::as_select())
            .first(&mut self.conn)?;

        println!("Found user: {}", user.username);

        if user.password != password {
            println!("Password mismatch");
            return Err(diesel::result::Error::NotFound);
        }
        
        Ok(user.id)
    }
}
