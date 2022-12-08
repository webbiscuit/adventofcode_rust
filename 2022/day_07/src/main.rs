use std::{
    collections::HashMap,
    io::{self, prelude::*},
};

#[derive(Debug)]
enum Artefact {
    File { name: String, size: usize },
    Directory { name: String },
}

#[derive(Debug)]
struct DirectorySize {
    size: usize,
    path: String,
}

#[derive(Debug)]
struct FileSystem {
    //artefacts: Vec<Artefact>,
    current_dir: Vec<String>,
    file_index: HashMap<String, Artefact>,
}

impl FileSystem {
    fn new() -> Self {
        let root = Artefact::Directory {
            name: "/".to_string(),
        };
        let mut file_index = HashMap::new();
        file_index.insert("/".to_string(), root);

        Self {
            file_index,
            current_dir: vec!["/".to_string()],
        }
    }

    fn dir_parts_to_string(parts: &Vec<&str>) -> String {
        if parts.len() == 1 {
            return "/".to_string();
        }

        format!("/{}", parts[1..].join("/"))
    }

    fn get_current_dir(&self) -> String {
        Self::dir_parts_to_string(
            &self
                .current_dir
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        )
    }

    fn get_full_path(&self, file_name: &str) -> String {
        let current_dir = if self.get_current_dir() == "/" {
            "".to_string()
        } else {
            self.get_current_dir()
        };

        format!("{}/{}", current_dir, file_name)
    }

    fn process_command(&mut self, command: Command) {
        match command {
            Command::ChangeDirectory { path } => {
                if path == "/" {
                    self.current_dir = vec![path];
                } else if path == ".." {
                    self.current_dir.pop();
                } else {
                    self.current_dir.push(path);
                }
                // println!("{:?}", self.get_current_dir());
            }
            Command::ListDirectory { results } => {
                for result in results {
                    let split = result.split_whitespace().collect::<Vec<&str>>();
                    if split[0] == "dir" {
                        let name = split[1];
                        let path = self.get_full_path(name);
                        let dir = Artefact::Directory {
                            name: name.to_string(),
                        };
                        self.file_index.insert(path, dir);
                    } else {
                        let name = split[1];
                        let size = split[0].parse::<usize>().unwrap();
                        let path = self.get_full_path(name);
                        let file = Artefact::File {
                            name: name.to_string(),
                            size,
                        };
                        self.file_index.insert(path, file);
                    }
                }
            }
        }
    }

    fn calculate_directory_sizes(&self) -> Vec<DirectorySize> {
        let mut directory_sizes: HashMap<String, DirectorySize> = HashMap::new();

        for (path, artefact) in self.file_index.iter() {
            if let Artefact::Directory { name } = artefact {
                if !directory_sizes.contains_key(path) {
                    directory_sizes.insert(
                        path.to_string(),
                        DirectorySize {
                            size: 0,
                            path: path.to_string(),
                        },
                    );
                }
            } else if let Artefact::File { name, size } = artefact {
                let mut dirs = path.split("/").collect::<Vec<&str>>();
                dirs.pop();
                dirs[0] = "/";
                while dirs.len() > 0 {
                    let path = Self::dir_parts_to_string(&dirs);
                    //println!("{:?} {}", path, name);

                    if !directory_sizes.contains_key(&path) {
                        directory_sizes.insert(
                            path.to_string(),
                            DirectorySize {
                                size: 0,
                                path: path.to_string(),
                            },
                        );
                    }
                    dirs.pop();

                    if let Some(dir_size) = directory_sizes.get_mut(&path) {
                        dir_size.size += size;
                        //println!("{:?}", dir_size);
                    }
                }
            }
        }

        directory_sizes.drain().map(|(_, v)| v).collect()
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory { path: String },
    ListDirectory { results: Vec<String> },
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut commands = Vec::new();

    for line_iter in lines {
        let line = line_iter.unwrap();

        if line.starts_with("$") {
            let command = line.split_whitespace().collect::<Vec<&str>>();
            match command[1] {
                "cd" => {
                    let path = command[2];
                    commands.push(Command::ChangeDirectory {
                        path: path.to_string(),
                    });
                }
                "ls" => {
                    commands.push(Command::ListDirectory {
                        results: Vec::new(),
                    });
                }
                _ => println!("Unknown command"),
            }
        } else {
            // It's a directory listing
            if let Command::ListDirectory { results } = commands.last_mut().unwrap() {
                results.push(line);
            }
        }
    }

    // println!("{:?}", commands);

    let mut file_system = FileSystem::new();
    for command in commands {
        file_system.process_command(command);
    }

    // println!("{:?}", file_system.file_index);
    // println!("{:?}", file_system.calculate_directory_sizes());

    let directory_sizes = file_system.calculate_directory_sizes();

    let max_dir_size = 100000;
    let directory_sum = directory_sizes
        .iter()
        .filter(|dir| dir.size <= max_dir_size)
        .fold(0, |acc, dir| acc + dir.size);

    println!(
        "The sum of total sizes of directories with at most 100000 size is {}.",
        directory_sum
    );

    let total_space = 70000000;
    let space_used = directory_sizes
        .iter()
        .max_by(|a, b| a.size.cmp(&b.size))
        .unwrap()
        .size;

    let free_space = total_space - space_used;
    let space_needed = 30000000 - free_space;

    let found_dir = directory_sizes
        .iter()
        .filter(|dir| dir.size >= space_needed)
        .min_by(|a, b| a.size.cmp(&b.size))
        .unwrap();

    println!("The size of the directory to delete is {}.", found_dir.size);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_parser() {
        let mut file_system = FileSystem::new();
        assert_eq!(file_system.get_current_dir(), "/");
        file_system.process_command(Command::ChangeDirectory {
            path: "dir1".to_string(),
        });
        assert_eq!(file_system.get_current_dir(), "/dir1");
    }
}
