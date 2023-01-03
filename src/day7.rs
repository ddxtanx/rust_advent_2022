use std::cmp::min;

use crate::file_lines::{get_file_lines, self};
use crate::tree::{ArenaTree, Node};

#[derive(PartialEq, Eq, Debug, Clone)]
enum FileObject {
    File(String, usize),
    Directory(String)
}

struct Filesystem { 
    dir_tree: ArenaTree<FileObject>
}


impl Filesystem {
    fn get_size_recursively(&self, node: &Node<FileObject>, vec: &mut Vec<usize>) {
        match node.val {
            FileObject::File(_, size) => vec[node.get_idx()] = size,
            FileObject::Directory(_) => {
                let size = self.dir_tree.get_node_children(node).unwrap().iter().
                    map(|node| {
                        self.get_size_recursively(node, vec);
                        vec[node.get_idx()]
                    }).
                    reduce(|tot, size| tot+size).unwrap_or(0);
                vec[node.get_idx()] = size;

            } 
        }
    }

    pub fn get_fs_size(&mut self) -> Vec<usize> {
        let mut ret_vec = vec![0; self.dir_tree.get_num_nodes()];
        if let Some(root) = self.dir_tree.get_root_node() {
            self.get_size_recursively(root, &mut ret_vec);
        }
        ret_vec
    }
}

enum ChangeDir {
    ParentDir,
    ChildDir(FileObject)
}

enum Command {
    CD(ChangeDir),
    LS(Vec<FileObject>)
}



impl Filesystem {
    pub fn parse_commands(cmds: Vec<Command>) -> Filesystem {
        let mut dir_tree: ArenaTree<FileObject> = ArenaTree::new();

        let root_dir = FileObject::Directory("/".to_string());

        dir_tree.new_node(root_dir);

        let mut root_node = 0;

        for command in cmds {
            match command{
                Command::CD(cd) => match cd {
                    ChangeDir::ParentDir => root_node = dir_tree.get_parent_node(root_node).unwrap().get_idx(),
                    ChangeDir::ChildDir(dir_name) => {
                        root_node = dir_tree.get_child_by_value(root_node, dir_name).unwrap().get_idx()
                    }
                },
                Command::LS(fobjs) => for fobj in fobjs {
                    dir_tree.new_node_as_child(root_node, fobj).unwrap()
                }
            }
        }

        Filesystem {
            dir_tree
        }
    }
}

fn parse_file_to_command_list() -> Vec<Command> {
    let file = "./inputs/day7.txt".to_string();

    let mut lines = file_lines::get_file_lines(file).peekable();
    lines.next();
    let mut command_vec = vec![];

    loop {
        let next_line = lines.next();

        if next_line.is_none(){
            break;
        }

        let next_line = next_line.unwrap();

        let (first_four, args) = next_line.split_at(4);
        let args = args.trim();
        if first_four == "$ cd" {
            if args == ".." {
                command_vec.push(Command::CD(ChangeDir::ParentDir));
            } else {
                let child_dir = FileObject::Directory(args.to_string());
                command_vec.push(Command::CD(ChangeDir::ChildDir(child_dir)));
            }
        } else {
            let mut ls_vec: Vec<FileObject> = vec![];// in LS case
            while let Some(line) = lines.next_if(|line| !line.starts_with('$')) {
                let mut line_spl = line.split(' ');
                let metad = line_spl.next().unwrap();
                let fname = line_spl.
                    fold("".to_owned(), |acc, part| {
                        acc + " " + part
                    }).
                    trim().
                    to_string();
                if metad == "dir" {
                    let fobj = FileObject::Directory(fname);
                    ls_vec.push(fobj);
                } else {
                    let fsize = metad.parse::<usize>().unwrap();
                    let fobj = FileObject::File(fname, fsize);
                    ls_vec.push(fobj);
                }
            }
            command_vec.push(Command::LS(ls_vec));
        }
    }
    command_vec
}

pub fn get_largest_dirs() -> usize {
    let command_ls = parse_file_to_command_list();
    let mut fs = Filesystem::parse_commands(command_ls);
    let fs_vec = fs.get_fs_size();
    fs_vec.iter().enumerate().
        filter(|(idx, size)| {
            let node = fs.dir_tree.get_node_by_idx(*idx).unwrap();
            matches!(node.val, FileObject::Directory(_)) && **size <= 100000
        }).
        fold(0, |acc, (_, size)| acc + *size)
} 

pub fn get_delete_dir() -> usize {
    let command_ls = parse_file_to_command_list();
    let mut fs = Filesystem::parse_commands(command_ls);
    let fs_vec = fs.get_fs_size();
    let tot_size = fs_vec[0];
    let nec_size = 30000000 - (70000000 - tot_size);
    *fs_vec.iter().enumerate().
        filter(|(idx, size)| {
            let node = fs.dir_tree.get_node_by_idx(*idx).unwrap();
            matches!(node.val, FileObject::Directory(_)) && **size >= nec_size
        }).
        map(|(_, size)| size).
        min().unwrap()

}
