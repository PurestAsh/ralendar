use std::collections::HashMap;

use crate::backend::Category;

/// 缓存，缓存category，缓存events
pub(super) struct Cache {
    category_cache: HashMap<i64, Category>,
}

impl Cache {
    pub(super) fn new() -> Self {
        Self {
            category_cache: HashMap::new(),
        }
    }

    pub(super) fn get_category(&self, id: i64) -> Option<&Category> {
        self.category_cache.get(&id)
    }

    pub(super) fn add_category(&mut self, category: Category) {
        let id = category.id;
        self.category_cache.insert(category.id, category);
    }
}
