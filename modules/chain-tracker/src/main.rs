pub mod mempool;
pub mod registry;

use mempool::tracker::track_mempool;
use registry::sync::{initial_registry_sync, continuous_registry_sync};

use ergonames_utils::database::{create_database_schema, wait_for_database};

fn main() {
    wait_for_database();
    create_database_schema();
    initial_registry_sync();
    loop {
        track_mempool();
        continuous_registry_sync();
    }
}