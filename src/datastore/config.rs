pub struct DBConfig {
    pub name: String,
    pub log_path_db: String,
    pub log_path_index: String,
    pub path_offset: String,
}

impl DBConfig {
    pub fn new(
        name: String,
        log_path_db: String,
        log_path_index: String,
        path_offset: String,
    ) -> Self {
        Self {
            name,
            log_path_db,
            log_path_index,
            path_offset,
        }
    }

    pub fn get_log_path_db(&self) -> &str {
        &self.log_path_db
    }

    pub fn get_log_path_index(&self) -> &str {
        &self.log_path_index
    }

    pub fn get_path_offset(&self) -> &str {
        &self.path_offset
    }

    pub fn print(&self) -> String {
        format!(
            "Database: {}, DB Log Path: {}, Index Log Path: {}, Offset Path: {},",
            self.name, self.log_path_db, self.log_path_index, self.path_offset
        )
    }
}
