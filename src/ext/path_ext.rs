use std::path::PathBuf;


pub trait PathExt {
    fn to_string(&self) -> String;
    fn get_path_name(&self) -> String;
    fn get_name_no_ext(&self) -> String;
    fn get_parent(&self) -> String;
    fn get_ext(&self) -> String;
    fn as_dir(&self) -> String;
}

impl PathExt for PathBuf {

    fn to_string(&self) -> String {
        String::from(self.to_str().unwrap())
    }

    fn get_path_name(&self) -> String {
        let path = self.to_str().unwrap();
        let ext = self.extension().unwrap();
        return String::from(&path[..(path.len() - ext.len() - 1)]);
    }

    fn get_name_no_ext(&self) -> String {
        let ext = self.extension().unwrap();
        let value = self.file_name().unwrap().to_str().unwrap();
        String::from(&value[..(value.len() - ext.len() - 1)])
    }

    fn get_parent(&self) -> String {
        if let Some("/") = self.to_str() {
            return String::new();
        }
        let mut value = self.parent().unwrap().to_str().unwrap();
        if value.is_empty() {
            value = "."
        }
        String::from(format!("{value}/"))
    }

    fn get_ext(&self) -> String {
        let value = self.extension().unwrap().to_str().unwrap();
        String::from(value)
    }

    fn as_dir(&self) -> String {
        let path = self.to_string();
        match path.ends_with('/') {
            true => path,
            _ => format!("{path}/"),
        }
    }
}
