use std::env;
use std::fs;
use std::collections::HashMap;

pub trait FsElement {
    fn size(&self, map: &HashMap<String, Directory>) -> u32; 
}

#[derive(Clone)]
pub struct Directory {
    children: Vec<String>,
    pub local_size: u32,
}

impl FsElement for Directory {
    fn size(&self, map: &HashMap<String, Directory>) -> u32 {
        let mut ret: u32 = self.local_size;
        for ckey in self.children.iter() {
            let child = map.get(ckey).unwrap();
            ret += child.size(map);
        }
        // println!("Size {}({:p}): {}", self.path, self, ret);
        return ret;
    }
}

fn get_path(root: &Vec<String>) -> String {
    let mut ret = String::from("");
    for ii in root {
        ret += ii;
        ret += "/";
    }
    return ret;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Input file: {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut path: Vec<String> = Vec::new();
    let mut directories: HashMap<String, Directory> = HashMap::new();
    directories.insert(String::from("//"), Directory {
        children: Vec::new(),
        local_size: 0,
    });
    for line in contents.lines() {
        if line.starts_with("$") {
            let cmd = String::from(line);
            if cmd == "$ ls" {
                // println!("FOUND LS: {}", cmd);
            } else {
                // only ls or cd in command
                let directory = &cmd[5..];
                // println!("CD '{}'", directory);
                match directory {
                    ".." => {
                        path.pop();
                    },
                    "/" => {
                        path.clear();
                        path.push(String::from(directory));
                    }
                    _ => {
                        path.push(String::from(directory));
                    }
                };
                // println!("{:#?}", get_path(&path));
            }
        } else {
            let elem = String::from(line);
            if elem.starts_with("dir") {
                let dir_name = &elem[4..];
                // println!("DIRECTORY '{}'", dir_name);
                let child_key = get_path(&path) + dir_name + "/";
                if !directories.contains_key(&child_key) {
                    let child = Directory{
                        children: Vec::new(),
                        local_size: 0,
                    };
                    // println!("Insert at {}", &child_key);
                    directories.insert(child_key.clone(), child.clone());
                } 
                let key = get_path(&path);
                // println!("Parent key {}", &key);
                let parent = directories.get_mut(&key).unwrap();
                parent.children.push(child_key);
            } else {
                let key = get_path(&path);
                let directory = directories.get_mut(&key).unwrap(); 
                let mut iter = elem.split_whitespace();
                let size: u32 = iter.next().unwrap().parse().unwrap();
                // let name = iter.next().unwrap();
                // println!("{}: {:#?}", name, size);
                directory.local_size += size;
            }
        }
    }

    // display size
    let mut part1_size = 0;
    for (_key, value) in directories.clone() {
        let dir_size = value.size(&directories);
        // println!("{}: {}", key, value.size(&directories));
        if dir_size <= 100000 {
            part1_size += dir_size;
        }
    }
    println!("Part 1: {}", part1_size);
    
    let total_size: u32 = 70000000;
    let wanted_free_size: u32 = 30000000;
    let used_size: u32 = directories.get(&String::from("//")).unwrap().size(&directories);
    let free_size = total_size - used_size;
    let to_free = wanted_free_size - free_size;
    println!("Free size: {} -> {}", free_size, to_free);
    let mut check_size = total_size;
    for (_key, value) in directories.clone() {
        let dir_size = value.size(&directories);
        if (dir_size >= to_free) && (dir_size < check_size) {
            check_size = dir_size;
        }
    }
    println!("Part 2: {}", check_size);

}
