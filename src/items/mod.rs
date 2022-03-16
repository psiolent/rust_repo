mod item;

pub use item::*;

use crate::storage::MongoRepo;

pub type MongoItemsRepo = MongoRepo<Item, ItemSpec>;
