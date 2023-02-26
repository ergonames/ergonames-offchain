pub mod registry;
pub mod utils;

use registry::sync::sync_registry;

fn main() {
    sync_registry();
}