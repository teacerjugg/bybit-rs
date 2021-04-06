use dotenv::dotenv;
use std::error::Error;

pub type BEResult = Result<(), Box<dyn Error>>;

pub fn init() {
    dotenv().ok();
    pretty_env_logger::init();
}
