#[derive(Debug)]
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

pub struct SizeFilter {
    size: u64
}
pub struct DateFilter {
    create_time: u64
}

pub struct ExtensionFilter {
    extension: String
}

impl ExtensionFilter {
    pub fn new(extension: String) -> Self {
        ExtensionFilter { extension }
    }
}

impl Filter for SizeFilter {
    fn apply(&self, value: &INode) -> bool {
        self.size < value.size
    }
}

impl Filter for DateFilter {
    fn apply(&self, value: &INode) -> bool {
        self.create_time < value.create_time
    }
}

impl Filter for ExtensionFilter {
    fn apply(&self, value: &INode) -> bool {
        self.extension == value.extension
    }
}

pub struct FileSystem {
    filters: Vec<Box<dyn Filter>>,
}


impl FileSystem {

    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        FileSystem { filters }
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    fn traverse_inode<'a>(&'a self, inode: &'a INode) -> Vec<&str> {
        let mut results: Vec<&str> = vec![];

        for n in inode.children() {
            if n.is_directory {
                let mut r = self.traverse_inode(n);
                results.append(&mut r);
            }

            for f in &self.filters {
                if f.apply(n) {
                    results.push(n.name());
                }
            }
        }

        results
    }

    pub fn traverse<'a>(&'a self, root: &'a INode) -> Vec<&str> {
        self.traverse_inode(root)
    }
}
