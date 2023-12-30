use std::env;

use comic_creator;

struct Options {
    pub clean_after: bool
}

impl Default for Options {
    fn default() -> Self {
        Self { clean_after: false }
    }
}

fn main() {
    start()
}

fn get_options() -> Result<Options, ()> {
    let args = env::args().collect::<Vec<_>>();
    if args.len() == 1 {
        return Ok(Options::default());
    }

    let mut args = args.into_iter();
    args.next();
    let mut options = Options::default();

    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        match arg {
            "clean" => {options.clean_after = true},
            "help" => {
                print_help();
                return Err(())
            }
            _ => {
                println!("unknown option: {}", arg);
                print_help();
                return Err(())
            }
        }
    }

    Ok(options)
}

fn start() {
    let options = get_options();

    if options.is_err() {
        return;
    }

    let options = options.unwrap();

    let working_dir = env::current_dir().expect("could not get current dir");
    let comic_folders = comic_creator::get_directories(&working_dir).expect("Could not get comic directories");

    for comic_folder in &comic_folders {
        let image_files = comic_creator::get_image_files(comic_folder);
        if let Err(err) = image_files {
            println!("{}", err);
            continue;
        }
        let image_files = image_files.unwrap();

        if image_files.len() == 0 {
            println!("no image files found in folder {}, skipping...", comic_folder.file_name().unwrap().to_str().unwrap());
            continue;
        }
        else {
            println!("image files found in folder {}, archiving...", comic_folder.file_name().unwrap().to_str().unwrap());
        }

        let result = comic_creator::create_cbz_file(&image_files, comic_folder);

        if let Err(err) = result {
            println!("{}", err);
            continue;
        }

        if options.clean_after {
            let result = comic_creator::clean_image_files(&image_files);

            if let Err(err) = result {
                println!("{}", err);
                continue;
            }
        }
    }
}

fn print_help() {
    println!("Command line tool for archiving comics");
    println!("Usage: create_comic [options]");
    println!("Options: ");
    println!("         clean          =>   clean/delete image files after archiving");
    println!("         help           =>   print help");
}