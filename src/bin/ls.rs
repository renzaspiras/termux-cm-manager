use std::env;
use std::fs;
//use std::path::Path;

fn main() {
    // Get the directory path from command line arguments
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // If no directory path is provided, use the current directory
        match env::current_dir() {
            Ok(path) => path.display().to_string(),
            Err(err) => {
                eprintln!("Error: {}", err);
                return;
            }
        }
    };

    // Read the contents of the specified directory
    match fs::read_dir(&dir_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        let entry_name = entry.file_name();
                        if let Some(name) = entry_name.to_str() {
                            // Determine if the entry is a file or directory
                            let entry_type = if entry_path.is_dir() {
                                ""
                            } else {
                                // Extract file extension
                                if let Some(extension) = entry_path.extension() {
                                    match extension.to_str() {
                                        Some("py") => "󰌠",
                                        Some("java") => "󰬷",
                                        //More icons coming soon....
                                        _ => "",
                                    }
                                } else {
                                    ""
                                }
                            };
                            println!("{} {}", entry_type, name);
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
