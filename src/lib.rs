use std::io;
use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq, Clone)]
pub struct INode {
    name: String,
    is_directory: bool,
    size: u64,
    create_time: u64,
    extension: String,
    children: Vec<INode>,
}

impl INode {
    pub fn new(name: String, is_directory: bool, size: u64, extension: String, create_time: u64) -> Self {
        INode {
            name,
            is_directory,
            size,
            create_time,
            extension,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: INode) {
        self.children.push(child);
    }

    pub fn children(&self) -> &Vec<INode> {
        &self.children
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub trait Filter {
    fn apply(&self, value: &INode) -> bool;
}


// Extension filter
pub struct ExtensionFilter {
    extension: String
}

impl ExtensionFilter {
    pub fn new(extension: String) -> Self {
        ExtensionFilter { extension }
    }
}

impl Filter for ExtensionFilter {
    fn apply(&self, value: &INode) -> bool {
        self.extension == value.extension
    }
}


// Size filter
pub struct SizeFilter {
    size: u64
}

impl SizeFilter {
    pub fn new(size: u64) -> Self {
        SizeFilter { size }
    }
}

impl Filter for SizeFilter {
    fn apply(&self, value: &INode) -> bool {
        self.size < value.size
    }
}

// Filesystem
pub struct FileSystem {
    root: INode,
}


impl FileSystem {
    pub fn new(root: INode) -> Self {
        FileSystem { root }
    }

    fn filter_inodes<'a>(&'a self, inode: &'a INode, filters: &Vec<Box<dyn Filter>>) -> Vec<&str> {
        let mut results: Vec<&str> = vec![];

        for n in inode.children() {
            if n.is_directory {
                let mut r = self.filter_inodes(n, &filters);
                results.append(&mut r);
            }

            // OR implementation
            for f in filters {
                if f.apply(n) {
                    results.push(n.name());
                }
            }
        }

        results
    }

    pub fn filter<'a>(&'a self, filters: &Vec<Box<dyn Filter>>) -> Vec<&str> {
        self.filter_inodes(&self.root, filters)
    }
}


pub fn visit_dir(path: &Path, parent: &mut INode) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.to_string_lossy().into_owned();

        let entry_meta = entry.metadata().unwrap();
        let size = entry_meta.len();

        if path.is_dir() {
            let mut inode = INode::new(filename, true, size, String::from(""), 1);
            visit_dir(&path, &mut inode)?;
            parent.add_child(inode);
        } else {
            let extension_split = filename.split(".");
            let extension = extension_split.last().unwrap().to_string();
            let child = INode::new(filename, false, size, extension, 1);
            parent.add_child(child);
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inode_can_add_child() {
        let mut parent = INode::new(String::from("parent"), true, 1, String::from(""), 1);
        let child = INode::new(String::from("child"), false, 1, String::from("rs"), 1);
        let cloned_child = child.clone();

        parent.add_child(child);

        assert!(parent.children().contains(&cloned_child));
    }

    #[test]
    fn filesystem_can_filter_by_extension() {
        // Given
        let mut parent = INode::new(String::from("parent"), true, 1, String::from(""), 1);
        let child = INode::new(String::from("child"), false, 1, String::from("rs"), 1);
        let cloned_child = child.clone();

        parent.add_child(child);
        let fs = FileSystem::new(parent);

        let filters: Vec<Box<dyn Filter>> = vec![Box::new(ExtensionFilter { extension: String::from("rs")})];

        // When
        let results = fs.filter(&filters);

        // Then
        assert!(results.contains(&cloned_child.name()));
    }

    #[test]
    fn filesystem_can_filter_by_size() {
        // Given
        let mut parent = INode::new(String::from("parent"), true, 1, String::from(""), 1);
        let child = INode::new(String::from("child"), false, 100, String::from("rs"), 1);
        let cloned_child = child.clone();

        parent.add_child(child);
        let fs = FileSystem::new(parent);

        let filters: Vec<Box<dyn Filter>> = vec![Box::new(SizeFilter { size: 99 })];

        // When
        let results = fs.filter(&filters);

        // Then
        assert!(results.contains(&cloned_child.name()));
    }

    #[test]
    fn filter_can_handle_empty_children() {
        // Given
        let parent = INode::new(String::from("parent"), true, 1, String::from(""), 1);
        let fs = FileSystem::new(parent);
        let filters: Vec<Box<dyn Filter>> = vec![Box::new(SizeFilter { size: 99 })];

        // When
        let results = fs.filter(&filters);

        // Then
        assert!(results.is_empty());
    }
}
