use surrealdb::err::Error::Thrown;
use surrealdb::Error;
use crate::infrastructure::data::db_context::surreal_context::DB;

pub struct Repository {
    table: String
}

impl<T> Repository {
     pub fn new(table: &str) -> Self {
         Repository {
             table: String::from(table),
         }
     }

     pub async fn get_all(&self) -> Result<Vec<T>, Error> {
         let records:Vec<T> = DB.select(&self.table).await?;
         Ok(records)
     }

    pub async fn get_by_id(&self, id: String) -> Result<T, Error> {
         if let Some(record) = DB.select((&self.table, id.clone())).await? {
             return Ok(record);
         }

         let error = Error::Db(
             Thrown(
                 format!("Todo with id {} not found", id)
             )
         );
         Err(error)
     }

    pub async fn create_repository(&self, content: T) -> Result<Vec<T>, Error> {
         let record:Vec<T> = DB.create(&self.table).content(content).await?;
         Ok(record)
     }

    pub async fn update_repository(&self, id: String, content: T) -> Result<T, Error> {
         let record = DB
             .update((&self.table, id))
             .content(content)
             .await?
             .unwrap();
         Ok(record)
     }

    pub async fn delete_repository(&self, id: String) -> Result<T, Error> {
         let result = DB.delete((&self.table, id)).await?.unwrap();
         Ok(result)
     }
 }