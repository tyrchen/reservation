use abi::Config;
use sqlx_db_tester::TestDb;
use std::{ops::Deref, path::Path};

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

    pub fn with_server_port(port: u16) -> Self {
        let mut config = TestConfig::default();
        config.config.server.port = port;
        config
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self::new("fixtures/config.yml")
    }
}
