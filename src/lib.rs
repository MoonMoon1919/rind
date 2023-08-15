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
}

pub trait Filter {
    fn apply(&self, value: &INode) -> bool;
}

struct SizeFilter {
    size: u64
}
struct DateFilter {
    create_time: u64
}

struct ExtensionFilter {
    extension: String
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
