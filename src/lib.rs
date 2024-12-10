pub mod gemini;

pub use gemini::GeminiClient;

use dotenv::dotenv;
use std::env;

pub fn init_logging() {
    dotenv().ok();
    env_logger::init();
}
