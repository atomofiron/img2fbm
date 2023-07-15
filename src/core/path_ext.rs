use std::path::PathBuf;


pub trait PathExt {
    fn to_string(&self) -> String;
    fn get_path_name(&self) -> String;
    fn get_name(&self) -> String;
    fn get_parent(&self) -> String;
    fn get_ext(&self) -> String;
}

impl PathExt for PathBuf {

    fn to_string(&self) -> String {
        String::from(self.to_str().unwrap())
    }

    fn get_path_name(&self) -> String {
        let path = self.to_str().unwrap();
        let ext = self.file_name().unwrap();
        return String::from(&path[..(path.len() - ext.len())]);
    }

    fn get_name(&self) -> String {
        let value = self.file_name().unwrap().to_str().unwrap();
        String::from(value)
    }

    fn get_parent(&self) -> String {
        if let Some("/") = self.to_str() {
            return String::new();
        }
        let mut value = self.parent().unwrap().to_str().unwrap();
        if value.is_empty() {
            value = "./"
        }
        String::from(value)
    }

    fn get_ext(&self) -> String {
        let value = self.extension().unwrap().to_str().unwrap();
        String::from(value)
    }
}
