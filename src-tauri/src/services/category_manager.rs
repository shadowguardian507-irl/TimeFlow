use crate::error::{Result, TimeFlowError};
use crate::models::Category;
use crate::services::storage::StorageService;

pub struct CategoryManager {
    storage: StorageService,
}

impl CategoryManager {
    pub fn new(storage: StorageService) -> Self {
        Self { storage }
    }

    /// Get the full category tree
    pub fn get_category_tree(&self) -> Result<Category> {
        self.storage.load_categories()
    }

    /// Get all visible category paths (for picker)
    pub fn get_visible_paths(&self) -> Result<Vec<String>> {
        let tree = self.storage.load_categories()?;
        Ok(tree.get_visible_paths())
    }

    /// Add a new category at the specified path
    pub fn add_category(&self, path: &str) -> Result<Category> {
        let mut tree = self.storage.load_categories()?;
        
        if !tree.add_at_path(path) {
            return Err(TimeFlowError::InvalidCategoryPath(path.to_string()));
        }

        self.storage.save_categories(&tree)?;
        Ok(tree)
    }

    /// Hide a category (and its children)
    pub fn hide_category(&self, path: &str) -> Result<()> {
        let mut tree = self.storage.load_categories()?;
        
        let category = tree
            .find_by_path_mut(path)
            .ok_or_else(|| TimeFlowError::CategoryNotFound(path.to_string()))?;

        Self::hide_recursive(category);
        self.storage.save_categories(&tree)?;
        Ok(())
    }

    fn hide_recursive(category: &mut Category) {
        category.hidden = true;
        for child in &mut category.children {
            Self::hide_recursive(child);
        }
    }

    /// Unhide a category
    pub fn unhide_category(&self, path: &str) -> Result<()> {
        let mut tree = self.storage.load_categories()?;
        
        let category = tree
            .find_by_path_mut(path)
            .ok_or_else(|| TimeFlowError::CategoryNotFound(path.to_string()))?;

        category.hidden = false;
        self.storage.save_categories(&tree)?;
        Ok(())
    }

    /// Validate that a category path exists and is not hidden
    pub fn validate_category_path(&self, path: &str) -> Result<bool> {
        let tree = self.storage.load_categories()?;
        
        match tree.find_by_path(path) {
            Some((category, _)) => {
                if category.hidden {
                    Err(TimeFlowError::CategoryHidden(path.to_string()))
                } else {
                    Ok(true)
                }
            }
            None => Ok(false),
        }
    }

    /// Check if a category exists (regardless of hidden status)
    pub fn category_exists(&self, path: &str) -> Result<bool> {
        let tree = self.storage.load_categories()?;
        Ok(tree.find_by_path(path).is_some())
    }
}
