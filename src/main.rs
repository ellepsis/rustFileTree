use std::fs::read_dir;
use std::fs::DirEntry;
use std::env;

#[derive(Debug)]
struct Dir {
    name: String,
    dirs: Vec<Dir>,
    files: Vec<File>,
    errs: Vec<Err>,
    size: u64,
}

#[derive(Debug)]
struct Err {
    name: String,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn read_directory<'a>(path: &str, dir_name: String) -> Result<Dir, Err> {
    let dir_content = read_dir(path);
    match dir_content {
        Ok(read_dir) => {
            let mut errs = Vec::new();
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            for entry in read_dir {
                match entry {
                    Ok(dir_entry) => {
                        parse_entry(&mut errs, &mut dirs, &mut files, dir_entry)
                    }
                    Err(_) => { errs.push(Err { name: String::new() }) }
                }
            }
            let files_size: u64 = files.iter().map(|file| file.size).sum();
            let dirs_size: u64 = dirs.iter().map(|dir| dir.size).sum();
            let total_size = files_size + dirs_size;
            Result::Ok(Dir {
                name: dir_name,
                dirs,
                errs,
                files,
                size: total_size,
            })
        }
        Err(_) => Result::Err(Err { name: path.to_string() }),
    }
}

fn parse_entry(errs: &mut Vec<Err>, dirs: &mut Vec<Dir>, files: &mut Vec<File>, dir_entry: DirEntry) -> () {
    let file_name = dir_entry.file_name();
    let file_name2 = file_name.into_string().unwrap();
    match dir_entry.file_type() {
        Ok(file_type) => {
            if file_type.is_dir() {
                let result = read_directory(dir_entry.path().to_str().unwrap(), file_name2.clone());
                match result {
                    Ok(dir) => { dirs.push(dir) }
                    Err(_) => { errs.push(Err { name: file_name2 }) }
                }
            } else {
                let file_size = dir_entry.metadata().map(|entry| entry.len()).unwrap_or_else(|_| 0);
                files.push(File { name: file_name2, size: file_size })
            }
        }
        Err(_) => errs.push(Err { name: file_name2 })
    }
}

impl Dir {
    pub fn print(&self) {
        self.print_with_level(1);
    }

    fn print_with_level(&self, level: u32) {
        let mut res = String::with_capacity(128);
        Dir::add_level_offset(level, &mut res);
        res.push_str(self.name.as_str());
        res.push_str(self.size.to_string().as_str());
        res.push_str(":\n");
        let level = level + 1;
        for file in &self.files {
            Dir::add_level_offset(level, &mut res);
            res.push_str(file.to_string().as_str());
            res.push('\n');
        }
        print!("{}", res);
        for dir in &self.dirs {
            dir.print_with_level(level)
        }
    }

    fn add_level_offset(level: u32, res: &mut String) {
        for _i in 0..(level - 1) {
            res.push(' ');
        }
        res.push('\\');
        res.push('-');

    }
}

impl ToString for File {
    fn to_string(&self) -> String {
        let mut res = self.name.clone();
        res.push('\t');
        res.push_str(self.size.to_string().as_str());
        res.push_str(" bytes");
        res
    }
}

impl ToString for Dir {
    fn to_string(&self) -> String {
        let mut res = self.name.clone();
        res.push_str(self.size.to_string().as_str());
        res.push_str(" bytes");
        res
    }
}


fn main() {
    let root_dir;
    {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            root_dir = args[1].clone();
        } else {
            let buf = env::current_dir().unwrap();
            let cow = buf.to_str();
            let x = cow.unwrap();
            root_dir = String::from(x);
        }
    }
    let dir = read_directory(root_dir.as_str(), root_dir.clone());
    match dir {
        Ok(dir) => { dir.print() }
        Err(_) => { print!("error has occurred") }
    }
}
