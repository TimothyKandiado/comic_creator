use std::ffi::OsStr;
use std::io::{Write, Read};
use std::{path::PathBuf, fs};
use std::error::Error;
use zip;

pub fn get_directories(working_dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut directories = Vec::new();

    //let working_dir = env::current_dir()?;

    let dir = fs::read_dir(working_dir)?;

    for entry in dir {
        if let Err(err) = entry {
            println!("{}", err);
            continue;
        }

        let entry = entry.unwrap();
        let filetype = entry.file_type();

        if let Err(err) = filetype {
            println!("{}", err);
            continue;
        }
        let filetype = filetype.unwrap();

        if !filetype.is_dir() {
            continue;
        }

        directories.push(entry.path());
    }

    Ok(directories)
}

/// Get image files in a folder
pub fn get_image_files(path: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let dir = fs::read_dir(path)?;
    let mut image_files = Vec::new();

    for entry in dir {
        if let Err(err) = entry {
            println!("{}", err);
            continue;
        }

        let entry = entry.unwrap();

        let filetype = entry.file_type();
        if let Err(err) = filetype {
            println!("{}", err);
            continue;
        }

        let filetype = filetype.unwrap();

        if !filetype.is_file() {
            continue;
        }

        if let Some(extension) = entry.path().extension() {
            if !is_viable_extension(extension) {
                continue;
            }

            image_files.push(entry.path())
        }
    }

    Ok(image_files)
}

pub fn create_cbz_file(image_files: &Vec<PathBuf>, output_folder: &PathBuf) -> Result<(), Box<dyn Error>> {
    let filename = output_folder.file_name().unwrap().to_str().unwrap();
    let filename = format!("{}.cbz", filename);
    let file = fs::File::create(output_folder.join(&filename))?;

    let mut zip_file = zip::ZipWriter::new(file);
    let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    for image_file in image_files {
        let filename = image_file.file_name().unwrap().to_str().unwrap();
        let mut file_handle = fs::File::open(image_file)?;
        let mut buffer = Vec::new();
        
        file_handle.read_to_end(&mut buffer)?;
        zip_file.start_file(filename, options)?;
        zip_file.write(&buffer)?;
    }
    Ok(())
}

pub fn clean_image_files(image_files: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for image_file in image_files {
        let result = fs::remove_file(image_file);

        if let Err(err) = result {
            println!("{}", err);
            continue;
        }
    }

    Ok(())
}

/// checks if the file extension is an image
fn is_viable_extension(extension: &OsStr) -> bool{
    let extension = extension.to_str().unwrap();

    matches!(extension, "jpg" | "png")
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsStr, path::Path};

    use crate::{is_viable_extension, get_image_files, create_cbz_file};

    #[test]
    fn test_is_viable_extension() {
        assert_eq!(is_viable_extension(OsStr::new("jpg")), true);
        assert_eq!(is_viable_extension(OsStr::new("png")), true);
        assert_eq!(is_viable_extension(OsStr::new("mp4")), false);
        assert_eq!(is_viable_extension(OsStr::new("exe")), false);
        assert_eq!(is_viable_extension(OsStr::new("txt")), false);
    }

    #[test]
    fn test_comic_creation() {
        let path = Path::new("test").to_path_buf();
        if !path.exists() {
            //println!("Create a folder 'test' and place placeholder images to test formation of comic cbr file");
            panic!("Create a folder 'test' and place placeholder images to test formation of comic cbr file");
        }
        let image_files = get_image_files(&path).unwrap();
        println!("Archiving {:#?}", image_files);

        create_cbz_file(&image_files, &path).unwrap();

        let cbz_path = path.join("test.cbz");

        if !cbz_path.exists() {
            panic!("cbz file was no successfully created")
        }
    }
}