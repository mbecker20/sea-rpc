use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
use serde::Deserialize;

#[derive(Deserialize)]
struct Env {
    db_address: String,
    db_user: String,
    db_password: String,
    db_name: String,
}

impl Env {
    fn db_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}",
            self.db_user, self.db_password, self.db_address
        )
    }
}

pub struct State {
    pub db: DatabaseConnection,
}

impl State {
    pub async fn load() -> State {
        dotenv::dotenv().ok();

        let env: Env = envy::from_env().unwrap();

        let db_url = env.db_url();

        let db = Database::connect(&db_url).await.unwrap();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("DROP DATABASE IF EXISTS \"{}\";", env.db_name),
        ))
        .await
        .unwrap();

        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("CREATE DATABASE \"{}\";", env.db_name),
        ))
        .await
        .unwrap();

        let db_url = format!("{}/{}", db_url, env.db_name);

        let db = Database::connect(db_url).await.unwrap();

        State { db }
    }
}
