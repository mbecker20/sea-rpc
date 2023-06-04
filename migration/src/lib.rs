pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_build_table;
mod m20220101_000001_create_deployment_table;
mod m20220101_000001_create_server_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_server_table::Migration),
            Box::new(m20220101_000001_create_deployment_table::Migration),
            Box::new(m20220101_000001_create_build_table::Migration),
        ]
    }
}
