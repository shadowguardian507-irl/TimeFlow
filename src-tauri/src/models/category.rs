use serde::{Deserialize, Serialize};

pub const CATEGORY_SEPARATOR: &str = " > ";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Category {
    pub name: String,
    pub hidden: bool,
    #[serde(default)]
    pub children: Vec<Category>,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hidden: false,
            children: Vec::new(),
        }
    }

    /// Get the full path of this category given its ancestors
    pub fn get_path(&self, ancestors: &[String]) -> String {
        if ancestors.is_empty() {
            self.name.clone()
        } else {
            let mut parts = ancestors.to_vec();
            parts.push(self.name.clone());
            parts.join(CATEGORY_SEPARATOR)
        }
    }

    /// Find a category by path, returning the category and its ancestors
    pub fn find_by_path(&self, path: &str) -> Option<(&Category, Vec<String>)> {
        self.find_by_path_recursive(path, Vec::new())
    }

    fn find_by_path_recursive(
        &self,
        path: &str,
        ancestors: Vec<String>,
    ) -> Option<(&Category, Vec<String>)> {
        let current_path = self.get_path(&ancestors);

        if current_path == path {
            return Some((self, ancestors));
        }

        // Check children
        let mut child_ancestors = ancestors.clone();
        if !self.name.is_empty() {
            child_ancestors.push(self.name.clone());
        }

        for child in &self.children {
            if let Some(result) = child.find_by_path_recursive(path, child_ancestors.clone()) {
                return Some(result);
            }
        }

        None
    }

    /// Find a mutable category by path
    pub fn find_by_path_mut(&mut self, path: &str) -> Option<&mut Category> {
        self.find_by_path_mut_recursive(path, Vec::new())
    }

    fn find_by_path_mut_recursive(
        &mut self,
        path: &str,
        ancestors: Vec<String>,
    ) -> Option<&mut Category> {
        let current_path = self.get_path(&ancestors);

        if current_path == path {
            return Some(self);
        }

        let mut child_ancestors = ancestors.clone();
        if !self.name.is_empty() {
            child_ancestors.push(self.name.clone());
        }

        for child in &mut self.children {
            if let Some(result) = child.find_by_path_mut_recursive(path, child_ancestors.clone()) {
                return Some(result);
            }
        }

        None
    }

    /// Get all visible categories as flat list of paths
    pub fn get_visible_paths(&self) -> Vec<String> {
        self.get_visible_paths_recursive(Vec::new())
    }

    fn get_visible_paths_recursive(&self, ancestors: Vec<String>) -> Vec<String> {
        let mut paths = Vec::new();

        if !self.hidden && !self.name.is_empty() {
            paths.push(self.get_path(&ancestors));
        }

        let mut child_ancestors = ancestors.clone();
        if !self.name.is_empty() {
            child_ancestors.push(self.name.clone());
        }

        for child in &self.children {
            paths.extend(child.get_visible_paths_recursive(child_ancestors.clone()));
        }

        paths
    }

    /// Add a category at the specified path, creating parent nodes as needed
    pub fn add_at_path(&mut self, path: &str) -> bool {
        let parts: Vec<&str> = path.split(CATEGORY_SEPARATOR).collect();
        self.add_at_path_recursive(&parts, 0)
    }

    fn add_at_path_recursive(&mut self, parts: &[&str], index: usize) -> bool {
        if index >= parts.len() {
            return true;
        }

        let part = parts[index];

        // Find or create child
        let child_index = self.children.iter().position(|c| c.name == part);

        match child_index {
            Some(idx) => self.children[idx].add_at_path_recursive(parts, index + 1),
            None => {
                let mut new_child = Category::new(part.to_string());
                new_child.add_at_path_recursive(parts, index + 1);
                self.children.push(new_child);
                true
            }
        }
    }
}

/// Root category tree (name is empty for root)
pub fn create_root() -> Category {
    Category {
        name: String::new(),
        hidden: false,
        children: Vec::new(),
    }
}
