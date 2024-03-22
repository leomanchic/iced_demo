use entity::book::{self, ActiveModel, Entity as Book};
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use sea_orm::{prelude::Decimal, ActiveValue, Database, DatabaseConnection, EntityTrait};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseBackend, ExecResult, QueryResult, Statement,
};
// use tokio_postgres::{Error, NoTls};
use crate::Choice;

use std::error::Error;
#[tokio::main]
pub async fn process(request: String, choice: &Choice) -> Result<(), Box<dyn Error>> {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@localhost/diesel_demo").await?;

    match choice {
        Choice::Execute => {
            let exec_res: ExecResult = db
                .execute(Statement::from_string(DatabaseBackend::Postgres, &request))
                .await?;
        }
        Choice::Querry => {
            let query_res_vec: Vec<QueryResult> = db
                .query_all(Statement::from_string(DatabaseBackend::Postgres, &request))
                .await?;
            for i in query_res_vec {
                println!("{:?}", i);
            }
        }
    }
    // let cheese: Option<<Book as EntityTrait>::Model> = Book::find_by_id(2).one(&db).await?;
    // println!("{:?}", );
    // cheese.unwrap().display();
    // let books: Vec<<Book as EntityTrait>::Model> = Book::find().all(&db).await?;

    // Book insertion
    // let pear = book::ActiveModel {
    //     book_id: ActiveValue::Set(14),
    //     title: ActiveValue::Set(Some("War and peace".to_string())),
    //     author: ActiveValue::Set(Some(apauthor)),
    //     price: ActiveValue::Set(Decimal::from_f32_retain(12.3)),
    //     amount: ActiveValue::Set(Some(12)),
    // };
    // let pear: book::Model = pear.insert(&db).await?;

    // pear.insert(&db).await?;

    // Connect to the database.
    // let (client, connection) = tokio_postgres::connect(
    //     "host=localhost user=postgres password='postgres' dbname=diesel_demo",
    //     NoTls,
    // )
    // .await?;

    // // The connection object performs the actual communication with the database,
    // // so spawn it off to run on its own.
    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    // // Now we can execute a simple statement that just returns its parameter.
    // let rows = client.query(&apauthor, &[]).await?;

    // // And then check that we got back the same string we sent over.
    // // let value: &str = rows[0].get(0);
    // for i in rows {
    //     println!("{:?}", i);
    // }

    Ok(())
}
