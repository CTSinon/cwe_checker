use nix::{sys::stat, unistd};
use std::{process::Command, thread::JoinHandle};
use std::thread;
use std::path::{Path, PathBuf};

pub fn get_ghidra_result(binary_path: &Path) -> (JoinHandle<()>, PathBuf) {
    let ghidra_path: std::path::PathBuf = PathBuf::from(env!("GHIDRA_INSTALL_DIR"));
    let headless_path = ghidra_path.join("support/analyzeHeadless");

    // Find the correct paths for temporary files.
    let project_dirs = directories::ProjectDirs::from("", "", "cwe_checker")
        .expect("Could not determine path for temporary files");
    let tmp_folder = if let Some(folder) = project_dirs.runtime_dir() {
        folder
    } else {
        Path::new("/tmp/cwe_checker")
    };
    if !tmp_folder.exists() {
        std::fs::create_dir(tmp_folder).expect("Unable to create temporary folder");
    }
    // We add a timestamp suffix to file names
    // so that if two instances of the cwe_checker are running in parallel on the same file
    // they do not interfere with each other.
    let timestamp_suffix = format!(
        "{:?}",
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let filename = binary_path
        .file_name()
        .expect("Invalid file name")
        .to_string_lossy()
        .to_string();

    // Create a unique name for the pipe
    let fifo_path = tmp_folder.join(format!("pcode_{}.pipe", timestamp_suffix));

    // Create a new fifo and give read and write rights to the owner
    if let Err(err) = unistd::mkfifo(&fifo_path, stat::Mode::from_bits(0o600).unwrap()) {
        eprintln!("Error creating FIFO pipe: {}", err);
        std::process::exit(101);
    }

    let thread_fifo_path = fifo_path.clone();
    let thread_file_path = binary_path.to_path_buf();
    let thread_tmp_folder = tmp_folder.to_path_buf();
    // Execute Ghidra in a new thread and return a Join Handle, so that the thread is only joined
    // after the output has been read into the cwe_checker
    let ghidra_subprocess = thread::spawn(move || {
        let output = match Command::new(&headless_path)
            .arg(&thread_tmp_folder) // The folder where temporary files should be stored
            .arg(format!("PcodeExtractor_{}_{}", filename, timestamp_suffix)) // The name of the temporary Ghidra Project.
            .arg("-import") // Import a file into the Ghidra project
            .arg(thread_file_path) // File import path
            .arg("-postScript") // Execute a script after standard analysis by Ghidra finished
            .arg("PcodeExtractor.java") // Path to the PcodeExtractor.java
            .arg(thread_fifo_path) // The path to the named pipe (fifo)
            .arg("-deleteProject") // Delete the temporary project after the script finished
            .arg("-analysisTimeoutPerFile") // Set a timeout for how long the standard analysis can run before getting aborted
            .arg("3600") // Timeout of one hour (=3600 seconds) // TODO: The post-script can detect that the timeout fired and react accordingly.
            .output() // Execute the command and catch its output.
        {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Error: Ghidra could not be executed:\n{}", err);
                std::process::exit(101);
            }
        };

        if !output.status.success() {
            match output.status.code() {
                Some(code) => {
                    eprintln!("{}", String::from_utf8(output.stdout).unwrap());
                    eprintln!("{}", String::from_utf8(output.stderr).unwrap());
                    eprintln!("Execution of Ghidra plugin failed with exit code {}", code);
                    std::process::exit(101);
                }
                None => {
                    eprintln!("Execution of Ghidra plugin failed: Process was terminated.");
                    std::process::exit(101);
                }
            }
        }
    });
    
    (ghidra_subprocess, fifo_path.clone())
}