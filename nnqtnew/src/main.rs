mod data;
mod model;
mod train;
mod plot;

use train::train_model;

fn main() {
    println!("🚦 Starting traffic classifier...");
    train_model();
}
