pub mod pg {
    //use chrono::NaiveDate;
    use postgres::{Client, Error, NoTls};

    pub struct PgConnection {
    }

    impl PgConnection {
        pub async fn new() -> Result<Client, Error> {
            let client = Client::connect(
                "host=localhost user=jowtro password=123qwe. dbname=testbox",
                NoTls,
            );
            match client {
                Ok(client) => Ok(client),
                Err(err) => Err(err),
            }
        }
    }
}
