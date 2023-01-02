use std::{fs, io};
use std::path::Path;
use std::time::SystemTime;

pub struct FileUtility {}

// Implementação para obter dados dos arquivos
impl FileUtility {

    // Método responsável por obter a data de criação do arquivo a partir do caminho passado no argumento
    pub fn get_creation_date(file_path: &str) -> SystemTime {
        let metada = fs::metadata(file_path).unwrap();
        metada.created().unwrap()
    }

    // Método responsável por obter a data de modificação do arquivo a partir do caminho passado no argumento
    pub fn get_modified_date(file_path: &str) -> SystemTime {
        let metada = fs::metadata(file_path).unwrap();
        metada.modified().unwrap()
    }

    // Método responsável por verificar se o caminho/arquivo existe
    pub fn exists(file_path: &str) -> bool {
        let path = Path::new(file_path);
        path.exists()
    }

    // Método responsável por verificar se o caminho é um diretório
    pub fn is_directory(file_path: &str) -> bool {
        let path = Path::new(file_path);
        path.is_dir()
    }

    // Método para criar um diretório, recursivamente se for necessário
    pub fn create_dir(dir_path: &str) -> io::Result<()> {
        fs::create_dir_all(dir_path)
    }
}