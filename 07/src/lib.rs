use std::cell::RefCell;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub struct FileSystem {
    pub root: Rc<RefCell<FileEntry>>,
    pub working_dir: Rc<RefCell<FileEntry>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FileEntry {
    name: String,
    size: usize,
    parent: Option<Rc<RefCell<FileEntry>>>,
    items: Vec<Rc<RefCell<FileEntry>>>,
    file_type: FileType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FileType {
    File,
    Directory,
}

#[derive(Debug, PartialEq)]
pub enum SeventhError {
    MissingArgumentForChangeDirectory,
    UnknownCommand(String),
    InvalidFileSize(String),
    DirectoryDoesNotExist(String),
    RootDirectoryHasNoParent,
    NoCandidateFound,
}
impl Error for SeventhError {}
impl Display for SeventhError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        let wd = Rc::new(RefCell::new(FileEntry::folder("", Vec::new())));
        Self {
            root: wd.clone(),
            working_dir: wd,
        }
    }
}

impl FileEntry {
    pub fn file(name: &str, size: usize) -> Self {
        Self {
            name: name.to_owned(),
            size,
            parent: None,
            items: Vec::new(),
            file_type: FileType::File,
        }
    }
    pub fn folder(name: &str, items: Vec<FileEntry>) -> Self {
        Self {
            name: name.to_owned(),
            size: 0,
            parent: None,
            items: items
                .into_iter()
                .map(|item| Rc::new(RefCell::new(item)))
                .collect(),
            file_type: FileType::Directory,
        }
    }

    pub fn size(&self) -> usize {
        match self.file_type {
            FileType::File => self.size,
            FileType::Directory => self.items.iter().map(|item| item.borrow().size()).sum(),
        }
    }
}

impl FileSystem {
    pub fn new(command_file: &str) -> Result<FileSystem, Box<dyn Error>> {
        let content = std::fs::read_to_string(command_file)?;
        let mut fs = FileSystem::default();
        for cmd in interprete_history(&content)? {
            fs.execute(cmd)?;
        }
        Ok(fs)
    }
    pub fn pwd(&self) -> String {
        let mut path = self.working_dir.borrow().clone();
        if path.parent.is_none() {
            return "/".to_string();
        }
        let mut result = vec![path.name];
        while let Some(p) = path.parent {
            result.push(p.borrow().name.clone());
            path = p.borrow().clone();
        }

        result.into_iter().rev().collect::<Vec<_>>().join("/")
    }
    pub fn exists(&self, name: &str) -> bool {
        self.working_dir
            .borrow()
            .items
            .iter()
            .any(|item| item.borrow().name == name)
    }

    pub fn touch(&mut self, name: &str, size: usize) {
        println!("$ touch {}", name);
        let mut file = FileEntry::file(name, size);
        file.parent = Some(self.working_dir.clone());
        self.add(file)
    }

    fn add(&mut self, fe: FileEntry) {
        self.working_dir
            .borrow_mut()
            .items
            .push(Rc::new(RefCell::new(fe)));
    }
    pub fn mkdir(&mut self, name: &str) {
        println!("$ mkdir {}", name);
        let mut dir = FileEntry::folder(name, Vec::new());
        dir.parent = Some(self.working_dir.clone());
        self.add(dir)
    }

    pub fn cd(&mut self, dir: &str) -> Result<(), SeventhError> {
        println!("$ cd {}", dir);
        match dir {
            ".." => {
                let new = self
                    .working_dir
                    .borrow()
                    .parent
                    .as_ref()
                    .map(|p| p.clone())
                    .ok_or(SeventhError::RootDirectoryHasNoParent)?;
                self.working_dir = new;
            }
            "/" => {
                self.working_dir = self.root.clone();
            }
            _ => {
                let new = self
                    .working_dir
                    .borrow()
                    .items
                    .iter()
                    .map(|i| i.clone())
                    .filter(|child| child.borrow().name == dir)
                    .next()
                    .ok_or(SeventhError::DirectoryDoesNotExist(dir.to_owned()))?;

                self.working_dir = new;
            }
        }
        Ok(())
    }

    pub fn execute(&mut self, cmd: Cmd) -> Result<(), SeventhError> {
        match cmd {
            Cmd::Cd { dir } => self.cd(dir),
            Cmd::Ls { items } => {
                println!("$ ls ({} items)", items.len());
                for item in items {
                    match item.file_type {
                        FileType::File => self.touch(&item.name, item.size),
                        FileType::Directory => self.mkdir(&item.name),
                    }
                }
                Ok(())
            }
        }?;
        Ok(())
    }

    pub fn disk_usage(&self) -> usize {
        self.root.borrow().size()
    }

    pub fn folders_with<F>(&self, predicate: F) -> Vec<(String, usize)>
    where
        F: Fn(usize) -> bool,
    {
        fn walker<F>(path: &FileEntry, predicate: &F) -> Vec<(String, usize)>
        where
            F: Fn(usize) -> bool,
        {
            let mut dirs = Vec::new();
            let s = path.size();
            if path.file_type == FileType::Directory && predicate(s) {
                dirs.push((path.name.to_owned(), s));
            }
            for item in path.items.iter() {
                dirs.append(&mut walker(&item.borrow(), predicate));
            }
            dirs
        }

        walker(&self.root.borrow(), &predicate)
    }
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ">>> {}\n{}", self.pwd(), self.root.borrow())
    }
}
impl Display for FileEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format(this: &FileEntry, indent: &str) -> String {
            match this.file_type {
                FileType::File => format!("{}- {} ({} B)", indent, this.name, this.size),
                FileType::Directory => {
                    let deeper_indent = format!("{}  ", indent);
                    let x = this
                        .items
                        .iter()
                        .map(|item| format(&item.borrow(), &deeper_indent))
                        .collect::<Vec<_>>()
                        .join("\n");
                    format!("{}+ {} ({} B)\n{}", indent, this.name, this.size(), x)
                }
            }
        }
        write!(f, "{}", format(self, ""))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cmd<'a> {
    Cd { dir: &'a str },
    Ls { items: Vec<FileEntry> },
}

pub fn interprete_history(log: &String) -> Result<Vec<Cmd>, SeventhError> {
    log.split("$")
        .filter(|line| !line.is_empty())
        .map(|line| line.trim())
        .map(|line| line.split_once("\n").unwrap_or((line, "")))
        .map(|(cmd, output)| {
            if cmd.starts_with("cd") {
                cmd.split_once(" ")
                    .map(|x| Cmd::Cd { dir: x.1 })
                    .ok_or(SeventhError::MissingArgumentForChangeDirectory)
            } else if cmd.starts_with("ls") {
                Ok(Cmd::Ls {
                    items: output
                        .lines()
                        .filter(|line| !line.is_empty())
                        .flat_map(|line| line.split_once(" "))
                        .map(|(s, n)| match s {
                            "dir" => Ok(FileEntry::folder(n, Vec::new())),
                            _ => Ok(FileEntry::file(
                                n,
                                s.parse::<usize>()
                                    .map_err(|_| SeventhError::InvalidFileSize(s.to_owned()))?,
                            )),
                        })
                        // .map(Box::new)
                        .collect::<Result<Vec<_>, _>>()?,
                })
            } else {
                Err(SeventhError::UnknownCommand(cmd.to_owned()))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn fs_display_file() {
        let fs = FileEntry::file("foo", 42);
        assert_eq!(format!("{}", fs), "- foo (42 B)")
    }

    #[test]
    fn fs_display_folder() {
        let fs = FileEntry::folder(
            "foo",
            vec![FileEntry::file("bar", 42), FileEntry::file("baz", 84)],
        );
        assert_eq!(
            format!("{}", fs),
            vec!["+ foo (126 B)", "  - bar (42 B)", "  - baz (84 B)"].join("\n")
        )
    }

    #[test]
    fn interpretes_history_finds_cd_cmd() {
        assert_eq!(
            interprete_history(&"$ cd foo".to_owned()),
            Ok(vec![Cmd::Cd { dir: "foo" }])
        );
    }

    #[test]
    fn interpretes_history_finds_ls_cmd() {
        let cmd = ["$ ls", "dir a", "1234 foo.txt"].join("\n");
        assert_eq!(
            interprete_history(&cmd),
            Ok(vec![Cmd::Ls {
                items: vec![
                    FileEntry::folder("a", Vec::new()),
                    FileEntry::file("foo.txt", 1234)
                ],
            }])
        );
    }

    #[test]
    fn interpretes_history_returns_err_for_missing_cd_operand() {
        assert_eq!(
            interprete_history(&"$ cd".to_owned()),
            Err(SeventhError::MissingArgumentForChangeDirectory)
        );
    }

    #[test]
    fn interpretes_history_returns_err_for_unknown_command() {
        assert_eq!(
            interprete_history(&"$ touch".to_owned()),
            Err(SeventhError::UnknownCommand("touch".to_owned()))
        );
    }
    #[test]
    fn interpretes_history_for_sample() -> Result<(), Box<dyn Error>> {
        let content = std::fs::read_to_string("sample.txt")?;

        let history = interprete_history(&content)?;
        assert_eq!(history[0], Cmd::Cd { dir: "/" });
        assert_eq!(history[2], Cmd::Cd { dir: "a" });
        assert_eq!(
            history[1],
            Cmd::Ls {
                items: vec![
                    FileEntry::folder("a", Vec::new()),
                    FileEntry::file("b.txt", 14848514),
                    FileEntry::file("c.dat", 8504156),
                    FileEntry::folder("d", Vec::new()),
                ]
            }
        );
        Ok(())
    }

    #[test]
    fn fs_execute_cd_returns_error_if_dir_doesnt_exist() {
        let mut fs = FileSystem::default();
        assert_eq!(fs.exists("foo"), false);
        let result = fs.execute(Cmd::Cd { dir: "foo" });
        assert_eq!(
            result,
            Err(SeventhError::DirectoryDoesNotExist("foo".to_owned()))
        );
    }

    #[test]
    fn sample_a_displays_tree() -> Result<(), Box<dyn Error>> {
        let mut fs = FileSystem::default();
        let content = std::fs::read_to_string("sample.txt")?;

        for cmd in interprete_history(&content)? {
            fs.execute(cmd)?;
        }

        println!("{}", fs);
        assert_eq!(
            format!("{}", fs),
            vec![
                ">>> /d",
                "+  (48381165 B)",
                "  + a (94853 B)",
                "    + e (584 B)",
                "      - i (584 B)",
                "    - f (29116 B)",
                "    - g (2557 B)",
                "    - h.lst (62596 B)",
                "  - b.txt (14848514 B)",
                "  - c.dat (8504156 B)",
                "  + d (24933642 B)",
                "    - j (4060174 B)",
                "    - d.log (8033020 B)",
                "    - d.ext (5626152 B)",
                "    - k (7214296 B)",
            ]
            .join("\n")
        );
        Ok(())
    }

    #[test]
    fn sample_b() -> Result<(), Box<dyn Error>> {
        let fs = FileSystem::new("sample.txt")?;
        let total_fs_size = 70_000_000;
        let required_free_space = 30_000_000;

        let free_space = total_fs_size - fs.disk_usage();
        assert_eq!(free_space, 21_618_835);

        let min_space_to_free = required_free_space - free_space;
        assert_eq!(min_space_to_free, 8_381_165);

        let mut candiates = fs.folders_with(|size| size >= min_space_to_free);
        candiates.sort_by_key(|(_, size)| *size);

        assert_eq!(
            candiates,
            vec![("d".to_owned(), 24933642), ("".to_owned(), 48381165)]
        );
        Ok(())
    }

    #[test]
    fn sample_a_computes_disk_size() -> Result<(), Box<dyn Error>> {
        let fs = FileSystem::new("sample.txt")?;
        assert_eq!(fs.disk_usage(), 48381165);
        Ok(())
    }

    #[test]
    fn sample_a_dirs_with_max_100_000_bytes() -> Result<(), Box<dyn Error>> {
        let fs = FileSystem::new("sample.txt")?;
        let dirs = fs.folders_with(|size| size <= 100_000);
        assert_eq!(dirs, vec![("a".to_owned(), 94853), ("e".to_owned(), 584)]);
        assert_eq!(dirs.iter().map(|(_, s)| s).sum::<usize>(), 95437);

        Ok(())
    }
}
