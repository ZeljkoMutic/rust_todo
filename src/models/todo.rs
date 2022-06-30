use super::super::schema::todos;
use super::Paginated;
use diesel::result;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

#[derive(PartialEq, Debug, serde::Serialize)]

pub struct Todo{
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub checked: bool,
}

impl Todo{
    pub fn show(connection: &crate::diesel::PgConnection, id: &str) -> Result<Self, result::Error>{
        let mut results: Vec<Todo> = todos::table
            .filter(todos::id.eq(&id))
            .load::<Self>(connection)?;

    match results.pop(){
        Some(todo) => Ok(todo),
        _ => Err(result::Error::NotFound),
    }
    }
}