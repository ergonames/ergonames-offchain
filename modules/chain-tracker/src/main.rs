pub mod mempool;
pub mod registry;
pub mod utils;

use mempool::tracker::track_mempool;
use utils::database::{create_database_schema, wait_for_database};
use registry::sync::{initial_registry_sync, continuous_registry_sync};

fn main() {
    wait_for_database();
    create_database_schema();
    initial_registry_sync();
    loop {
        track_mempool();
        continuous_registry_sync();
    }
}