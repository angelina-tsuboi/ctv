use std::{fs};
use std::path::{PathBuf};
use std::error::Error;
use crate::protocols::{File, EnvManager};
use crate::protocols::file::{FileStyle};

#[derive(Clone)]
pub struct TreeGenerator {
    root_dir: PathBuf,
    tree: Vec<String>,
    pipe: String,
    elbow: String, 
    tee: String,
    pipe_prefix: String,
    space_prefix: String,
    show_dir_metadata: String,
    show_file_metadata: String,
    file_styles: FileStyle,
    time_format: String,
    time_type: String,
    layer_limit: i32,
    show_extension: bool
}

impl TreeGenerator {
    pub fn init(root_dir: PathBuf, env_manager: EnvManager) -> Self {
        let file_style: FileStyle = FileStyle::new(
            env_manager.file_size_position,
            env_manager.file_owner_position,
            env_manager.file_perms_position,
            env_manager.dir_name_color,
            env_manager.file_name_color,
            env_manager.file_time_color,
            env_manager.file_size_color,
            env_manager.file_owner_color,
            env_manager.file_perms_color,
            env_manager.file_extension_color,
            env_manager.dir_name_style,
            env_manager.file_name_style,
            env_manager.file_size_style,
            env_manager.file_owner_style,
            env_manager.file_perms_style,
            env_manager.file_time_style,
            env_manager.file_extension_style
        );

        Self {
            tree: Vec::new(),
            pipe:  env_manager.pipe,
            elbow:  env_manager.elbow,
            tee:  env_manager.tee,
            pipe_prefix:  env_manager.pipe_prefix,
            space_prefix:  env_manager.space_prefix,
            root_dir: root_dir,
            show_dir_metadata:  env_manager.show_dir_metadata,
            show_file_metadata: env_manager.show_file_metadata,
            file_styles: file_style,
            time_format: env_manager.file_time_format,
            time_type: env_manager.file_time_type,
            layer_limit: env_manager.tree_layer_limit,
            show_extension: env_manager.file_extension_position != 0
        }   
    }
    pub fn build_tree(&mut self) -> Vec<String>{
        self.tree_head();
        self.tree_body(self.root_dir.clone(), &"".to_string(), self.layer_limit);
        return self.tree.clone();
    }

    fn sort_dir_first(&self, directory: PathBuf) -> Result<Vec<fs::DirEntry>, Box<dyn Error>>{
        let mut dir_vec: Vec<fs::DirEntry> = Vec::new();
        let mut file_vec: Vec<fs::DirEntry> = Vec::new(); 
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
    
            let metadata = fs::metadata(&path)?;
    
            if metadata.is_file(){
                file_vec.push(entry);
            }else if metadata.is_dir(){
                dir_vec.push(entry);
            }
        }

        dir_vec.append(&mut file_vec);
        Ok(dir_vec)
    }

    fn get_dir_item_amount(&self, directory: PathBuf) -> usize {
        return directory.iter().count();
    }

    fn tree_head(&mut self) {
        let dir_file = File::new(self.root_dir.clone(), &self.time_format, &self.time_type, &self.file_styles, self.show_extension, self.env_manager);
        self.tree.push(dir_file.display_format()); // prints out head dir
        self.tree.push(self.pipe.clone()); //print pipe under head dir
    }

    fn tree_body(&mut self, directory: PathBuf, prefix: &String, limit: i32) {

        let entries = self.sort_dir_first(directory).unwrap();
        let entries_count = entries.len();
        

        for (index, entry) in entries.iter().enumerate(){
            let connector;
            let metadata = fs::metadata(entry.path()).unwrap();

            if index == entries_count - 1 && (!metadata.is_dir() ||  self.get_dir_item_amount(entry.path()) == 0) {
                connector = &self.elbow;
            }else{
                connector = &self.tee;
            }

            if metadata.is_dir() {
                self.add_directory(
                    entry.path(), entry.path(), index, entries_count, prefix.to_string(), connector.to_string(), limit - 1
                )
            }else {
                self.add_file(entry.path(), prefix.to_string(), connector.to_string())
            }
        }   
    }

    fn add_directory(&mut self, directory: PathBuf, directory2: PathBuf, index: usize, entries_count: usize, mut prefix: String, connector: String, limit: i32) {
        let new_file = File::new(directory, &self.time_format, &self.time_type, &self.file_styles, self.show_extension);
        let file_name = if self.show_dir_metadata == "TRUE" {new_file.display_format()} else {new_file.get_name()};
        self.tree.push(format!("{}{} {}", prefix, connector, file_name));
        if index != entries_count - 1 {
            prefix += &self.pipe_prefix;
        }else {
            prefix += &self.space_prefix;
        }
        if limit > 0 {
            self.tree_body(directory2, &prefix.to_string(), limit)
        }
        
    }

    fn add_file(&mut self, file: PathBuf, prefix: String, connector: String) {
        let new_file = File::new(file, &self.time_format, &self.time_type, &self.file_styles, self.show_extension, self.env_manager);
        let file_name: String = if self.show_file_metadata == "TRUE" {new_file.display_format()} else {new_file.get_name()};
        self.tree.push(format!("{}{} {}", prefix, connector, file_name));
    }
}