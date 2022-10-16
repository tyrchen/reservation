use std::{ops::Deref, path::Path};

use abi::Config;
use sqlx_db_tester::TestDb;

pub struct TestConfig {
    #[allow(dead_code)]
    tdb: TestDb,
    pub config: Config,
}

impl Deref for TestConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl TestConfig {
    #[allow(dead_code)]
    pub fn new(filename: impl AsRef<Path>) -> Self {
        let mut config = Config::load(filename).unwrap();
        let tdb = TestDb::new(
            &config.db.host,
            config.db.port,
            &config.db.user,
            &config.db.password,
            "../migrations",
        );

        config.db.dbname = tdb.dbname.clone();
        Self { tdb, config }
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new("fixtures/config.yml")
    }
}
