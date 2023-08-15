use std::io;
use std::fs;
use std::path::Path;

use rind::ExtensionFilter;
use rind::FileSystem;
use rind::INode;

fn visit_dir(path: &Path, parent: &mut INode) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.to_string_lossy().into_owned();

        if path.is_dir() {
            let mut inode = INode::new(filename, true, 1, String::from(""), 1);
            visit_dir(&path, &mut inode)?;
            parent.add_child(inode);
        } else {
            let extension_split = filename.split(".");
            let extension = extension_split.last().unwrap().to_string();
            let child = INode::new(filename, false, 1, extension, 1);
            parent.add_child(child);
        }
    }

    Ok(())
}


fn main() {
    let mut root = INode::new(String::from("."), true, 1, String::from(""), 1);

    // Build the tree
    match visit_dir(Path::new("."), &mut root) {
        Err(e) => println!("Error! {}", e),
        _ => (),
    }

    let fs = FileSystem::new(vec![Box::new(ExtensionFilter::new(String::from("rs")))]);

    for ele in fs.traverse(&root) {
        println!("{ele}")
    };

}
