use diesel::{
    mysql::MysqlConnection,
    r2d2::{self, ConnectionManager},
};

pub type Connection = MysqlConnection;
pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn get_db_pool() -> Pool {
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
