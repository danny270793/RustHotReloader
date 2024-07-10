use std::{path, env, fs, process, time, thread};

fn help() {
    println!("Usage of hotreloader -- command argument");
}

fn version() {
    println!("hotreloader 1.0.0");
}

fn error(message: &str) {
    println!("{}", message);
    help();
    process::exit(0x0100);
}

fn main() {
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() < 2 {
        error("missing arguments");
    }

    let program: &String = &arguments[1];
    let argument: &String = &arguments[2];

    let argument_full_path: std::path::PathBuf = fs::canonicalize(argument).unwrap();

    let argument_parent_folder_as_string: String = argument_full_path.parent().unwrap().display().to_string();
    let argument_parent_folder: &str = argument_parent_folder_as_string.as_str();
    let mut last_modified: time::SystemTime = get_latest_modification_time(argument_parent_folder);

    let mut child_process: Option<process::Child> = Some(process::Command::new(program).arg(argument)
        .spawn()
        .unwrap()
    );

    loop {
        let start: time::Instant = time::Instant::now();
        let (changed_files, total_files) = get_files_modified_before(last_modified, argument_parent_folder);
        let duration: time::Duration = start.elapsed();

        if changed_files.len() > 0 {
            println!("{} files changed:", changed_files.len());
            for file in changed_files {
                println!("\t{}", file.path().display());
                let metadata: fs::Metadata = fs::metadata(file.path()).unwrap();
                let modified_time: time::SystemTime = metadata.modified().unwrap();
                if modified_time > last_modified {
                    last_modified = modified_time;
                }
            }
            println!("took {:?} to scan {} files", duration, total_files);
            
            if let Some(ref mut child) = child_process {
                let _ = child.kill();
            }

            child_process = Some(process::Command::new(program).arg(argument)
                .spawn()
                .unwrap()
            );
        }

        thread::sleep(time::Duration::from_secs(2));
    }
}

fn get_files_modified_before<P: AsRef<path::Path>>(since: time::SystemTime, path: P) -> (Vec<fs::DirEntry>, usize) {
    let mut files: Vec<fs::DirEntry> = Vec::new();
    let mut total_files: usize = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path: path::PathBuf = entry.path();
                let error_message: String = format!("can not get metadata from {:?}", path.display());
                let metadata: fs::Metadata = fs::metadata(path.clone()).expect(&error_message);
                if metadata.is_dir() {
                    let (sub_files, files_counter) = get_files_modified_before(since, path);
                    files.extend(sub_files);
                    total_files += files_counter;
                } else {
                    let modified_time: time::SystemTime = metadata.modified().unwrap();

                    total_files += 1;
                    if modified_time > since {
                        files.push(entry);
                    }
                }
            }
        }
    }

    (files, total_files)
}

fn get_latest_modification_time<P: AsRef<path::Path>>(path: P) -> time::SystemTime {
    let mut latest_time: time::SystemTime = time::SystemTime::UNIX_EPOCH;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let metadata: fs::Metadata = fs::metadata(entry.path()).unwrap();
                let modified_time: time::SystemTime = metadata.modified().unwrap();

                if modified_time > latest_time {
                    latest_time = modified_time;
                }
            }
        }
    }

    latest_time
}
