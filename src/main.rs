use std::fs;

use chrono::Datelike;
use clap::Parser;

use file_organizer::utils::date_utility::DateUtility;
use file_organizer::utils::file_utility::FileUtility;

fn main() {

    let extensions: Vec<&str> = vec!["png", "bmp", "jpeg", "jpg", "gif", "tiff", "raw", "mov", "avi", "mp4", "mpeg", "mpg", "webm",
                                     "flv", "vob", "mkv", "wmv", "mpv", "amv", "m4v", "3gp", "siv", "qt", "yuv"];

    let args = Args::parse();
    let path_src: String = String::from(args.src);
    let path_dst: String = String::from(&args.dst);

    // verificar a pasta de origem
    let src_is_dir: bool = FileUtility::is_directory(&path_src);
    let src_exists: bool = FileUtility::exists(&path_src);

    // verificar a pasta de destino
    let _dst_is_dir: bool = FileUtility::is_directory(&path_dst);
    let dst_exists: bool = FileUtility::exists(&path_dst);

    // Senão existir a pasta de destino, cria-se
    match dst_exists {
        true => None,
        false => Some(FileUtility::create_dir(&path_dst))
    };

    // Verifica a validade da pasta de origem
    match src_exists {
        true => match src_is_dir {
            true => None::<()>,
            false => panic!("Erro! O caminho {} não é uma pasta válida!", path_src)
        },
        false => panic!("Erro! o caminho {} não existe!", path_src)
    };

    for entry in fs::read_dir(&path_src).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        let extension = match file_path.as_path().extension() {
            None => String::from(""),
            Some(ext) => ext.to_str().unwrap().to_lowercase()
        };

        if file_path.is_file() && extensions.contains(&extension.as_str()){
            let metadata = fs::metadata(&file_path).unwrap();
            let _last_modified = DateUtility::get_date(metadata.modified().unwrap());
            let created = DateUtility::get_date(metadata.created().unwrap());
            let filename = file_path.as_path().file_name().unwrap().to_str().unwrap();
            let new_file_path = format!("{}/{}", &path_dst, format!("{}/{}", created.year(), created.month()));

            if !FileUtility::exists(&new_file_path) {
                match FileUtility::create_dir(&new_file_path) {
                    Ok(_) => {}
                    Err(_) => {println!("Erro ao criar a pasta {}", &new_file_path)}
                }
            }

            // match fs::rename(file_path.to_str().unwrap(), format!("{}/{}", new_file_path, filename)) {
            //     Ok(_) => { println!("Arquivo {} movido de {} para {}", filename, path_src, new_file_path) }
            //     Err(error) => { println!("Erro ao mover arquivo {} de {} para {} Erro: {}", filename, path_src, new_file_path, error)  }
            // }
        }
    }
}

/// Options
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {

    /// pasta de origem
    #[clap(short, long, default_value_t= String::from("."))]
    src: String,

    /// pasta de destino
    #[clap(short, long, default_value_t = String::from("."))]
    dst: String,
}

