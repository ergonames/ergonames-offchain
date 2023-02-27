pub mod mempool;
pub mod registry;
pub mod utils;

use mempool::tracker::track_mempool;
use utils::database::create_database_schema;
use registry::sync::sync_registry;

fn main() {
    create_database_schema();
    sync_registry();
    track_mempool();
}