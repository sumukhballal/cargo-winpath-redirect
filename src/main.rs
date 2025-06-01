use std::{
    env,
    fs,
    path::{PathBuf},
    process::{exit, Command, Stdio},
};
use fs_extra::dir::{copy, CopyOptions};

// Todo: Support different drives.
const DEFAULT_SHORT_BASE_PATH: &str = "c:\\b";
const PATH_LENGTH_LIMIT: usize = 10;
const PRESERVE_SHORT_PATH_ENV_VAR: &str = "CARGO_WIN_REDIRECT_PRESERVE";

fn main() {
    let project_dir = env::current_dir().expect("Failed to get current dir");
    let path_len = project_dir.to_string_lossy().len();
    let cargo_args: Vec<String> = env::args().collect();

    if cargo_args.len() < 3 { // Need at least cargo-win-redirect, win-redirect, and a command
        eprintln!("cargo-win-redirect: A wrapper to run cargo commands in a short path.");
        eprintln!("Usage: cargo winpath-redirect <cargo_command> [cargo_args...]");
        eprintln!("Example: cargo winpath-redirect run --release -- --app-arg");
        eprintln!("Example: cargo winpath-redirect build --target riscv32imc-esp-espidf");
        eprintln!("\nConfiguration via Environment Variables:");
        eprintln!("Base path for short project copies (default: {})", DEFAULT_SHORT_BASE_PATH);
        eprintln!("{}: Set to 'true' or '1' to preserve the short path directory after execution for debugging.", PRESERVE_SHORT_PATH_ENV_VAR);
        std::process::exit(1);
    }

    if path_len < PATH_LENGTH_LIMIT {
        println!("[win-redirect] Skipping win redirect path because Project Path length < 10 chars");
        let status = Command::new("cargo")
            .arg("run")
            .args(&cargo_args)
            .status()
            .expect("Failed to execute cargo run");
        exit(status.code().unwrap_or(1));
    }

    
    let cargo_command_to_run = &cargo_args[2].clone();
    let cargo_sub_args = &cargo_args[3..];

    let build_directory = PathBuf::from(DEFAULT_SHORT_BASE_PATH);
    println!("[win-redirect] Running in windows path redirect mode. Outputs will be piped to {}", build_directory.display());

    match env::var("SKIP_PROJECT_COPY") {
        Ok(val) if val == "true" => {
            println!("[win-redirect] Skipping project build copy..!");
        }

        _ => {
             // Clean previous copy
            if build_directory.exists() {
                println!("[win-redirect] Build path exists. Clearing..!");
                fs::remove_dir_all(&build_directory).expect("Failed to clean old short path");
            }

            fs::create_dir_all(&build_directory).expect("Failed to create short path root");
            println!("[win-redirect] Created build directory {}", build_directory.display());

            // TODO: Skip specific directories like Target from source.

            // Copy project
            let copy_opts = CopyOptions {
                copy_inside: true,
                overwrite: true,
                content_only: false,
                ..Default::default()
            };

            println!("[win-redirect] Copying project {} to build directory {}", project_dir.display(), build_directory.display());
            copy(&project_dir, &build_directory, &copy_opts).expect("Failed to copy project");
            println!("[win-redirect] Copied project {} to build directory {}", project_dir.display(), build_directory.display());
        }
    }

    // Get project build directory
    let project_build_directory = build_directory.join(project_dir.file_name().unwrap());

    // Clean directory
    clean(&project_build_directory);

    // Build in project build directory
    run(&project_build_directory, cargo_command_to_run, cargo_sub_args);

    // TODO: Copy built project target back to original project.
}

fn run(build_directory: &PathBuf, cargo_command_to_run: &String, cargo_sub_args: &[String]) {
    println!("[win-redirect] Running project in directory {}", build_directory.display());

    let status = Command::new("cargo")
    .arg(cargo_command_to_run)
    .args(cargo_sub_args)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .current_dir(&build_directory)
    .status()
    .expect("Failed to execute cargo run from short path");

    if !status.success() {
        eprintln!("[win-redirect] Build failed.");
        exit(status.code().unwrap_or(1));
    }
}

fn clean(build_directory: &PathBuf) {
    println!("[win-redirect] Cleaning up build directory {} from any previous runs.", build_directory.display());

    Command::new("cargo")
    .arg("clean")
    .current_dir(&build_directory)
    .status()
    .expect("Failed to execute cargo clean from short path");
}