use core::str;
use std::{
    collections::HashMap,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use argh::FromArgs;
use serde::{Deserialize, Serialize};

macro_rules! string_enum {
    (
        #[string_enum(name = $name_string:literal, doc = $doc:literal)]
        $(#[$meta:meta])*
        enum $name:ident {
            $($variant:ident = $string:literal),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[doc = $doc]
        enum $name {
            $($variant),*
        }

        impl std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($string => Ok(Self::$variant),)*
                    _ => Err(format!("Invalid {} '{}'", $name_string, s)),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Self::$variant => $string),*
                }.fmt(f)
            }
        }
    };
}

string_enum! {
    #[string_enum(
        name = "architecture",
        doc="Must be supported by both the [Rust image](https://hub.docker.com/_/rust/) and [Zig](https://ziglang.org/download/)."
    )]
    #[derive(Serialize, Deserialize, Clone)]
    enum Architecture {
        X86_64 = "x86_64",
        Arm64 = "arm64",
    }
}

impl Architecture {
    fn to_zig_string(&self) -> &'static str {
        match self {
            Architecture::X86_64 => "x86_64",
            Architecture::Arm64 => "aarch64",
        }
    }
}

string_enum! {
    #[string_enum(
        name = "Zig version",
        doc = "See the [Zig releases page](https://ziglang.org/download/) for more information."
    )]
    #[derive(Serialize, Deserialize, Clone)]
    enum ZigVersion {
        V0_13_0 = "0.13.0"
    }
}

/// Build a new image.
#[derive(FromArgs, Serialize, Deserialize, Clone)]
#[argh(subcommand, name = "build")]
struct BuildCommand {
    /// target architecture (arm64, x86_64)
    #[argh(option, short = 'a', long = "arch")]
    architecture: Architecture,

    /// version of zig to install (e.g., 0.13.0)
    #[argh(option, default = "ZigVersion::V0_13_0")]
    zig_version: ZigVersion,

    /// url to git repository of spade to package
    #[argh(
        option,
        default = "String::from(\"https://gitlab.com/spade-lang/spade\")"
    )]
    spade_git: String,

    /// version of spade to package, e.g. a branch name or commit hash
    #[argh(option, default = "String::from(\"main\")")]
    spade_rev: String,

    /// url to git repository of swim to package
    #[argh(
        option,
        default = "String::from(\"https://gitlab.com/spade-lang/swim\")"
    )]
    swim_git: String,

    /// version of swim to package, e.g. a branch name or commit hash
    #[argh(option, default = "String::from(\"main\")")]
    swim_rev: String,

    /// image tag, passed to `--tag` directly
    #[argh(option, short = 't')]
    tag: Option<String>,
}

/// List built images as JSON.
#[derive(FromArgs)]
#[argh(subcommand, name = "list")]
struct ListCommand {}

/// Prune built images.
#[derive(FromArgs)]
#[argh(subcommand, name = "clean")]
struct CleanCommand {}

/// Print data directory.
#[derive(FromArgs)]
#[argh(subcommand, name = "data-directory")]
struct DataDirectoryCommand {}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Build(BuildCommand),
    List(ListCommand),
    Clean(CleanCommand),
    DataDirectory(DataDirectoryCommand),
}

/// Manage Spade docker images.
#[derive(FromArgs)]
struct CliArgs {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

fn data_dir() -> PathBuf {
    dirs::data_local_dir().unwrap().join("spade-docker")
}

fn init_log_if_missing() -> io::Result<()> {
    fs::create_dir_all(data_dir())
}

fn log_image(hash: &str, build_command: BuildCommand) -> io::Result<()> {
    let mut logged_images = retrieve_logged_images()?;
    logged_images.insert(hash.to_string(), build_command);
    try_update_log(&logged_images)
}

fn retrieve_logged_images() -> io::Result<HashMap<String, BuildCommand>> {
    let log_file = data_dir().join("hashes.txt");
    if log_file.exists() {
        let bytes = fs::read(log_file)?;
        let contents = str::from_utf8(&bytes).expect("bug: non utf8 data written to log file");
        serde_json::from_str(contents).map_err(io::Error::other)
    } else {
        Ok(HashMap::default())
    }
}

fn try_update_log(new_log: &HashMap<String, BuildCommand>) -> io::Result<()> {
    let temp_file = data_dir().join("hashes.temp.txt");
    let log_file = data_dir().join("hashes.txt");
    fs::write(
        &temp_file,
        serde_json::to_string(new_log).expect("failed to serialize"),
    )?;
    fs::rename(temp_file, log_file)
}

fn main() -> io::Result<()> {
    init_log_if_missing()?;

    match argh::from_env::<CliArgs>().subcommand {
        Subcommand::Build(build_command) => {
            let mut stderr = Command::new("docker")
                .arg("build")
                .args([
                    "--platform",
                    &format!("linux/{}", build_command.architecture),
                ])
                .args([
                    "--build-arg",
                    &format!(
                        "ZIG_TARGET_PLATFORM={}",
                        build_command.architecture.to_zig_string()
                    ),
                ])
                .args([
                    "--build-arg",
                    &format!("ZIG_VERSION={}", build_command.zig_version),
                ])
                .args([
                    "--build-arg",
                    &format!("SPADE_GIT={}", build_command.spade_git),
                ])
                .args([
                    "--build-arg",
                    &format!("SPADE_REV={}", build_command.spade_rev),
                ])
                .args([
                    "--build-arg",
                    &format!("SWIM_GIT={}", build_command.swim_git),
                ])
                .args([
                    "--build-arg",
                    &format!("SWIM_REV={}", build_command.swim_rev),
                ])
                .args(
                    build_command
                        .tag
                        .as_ref()
                        .map(|tag| vec!["--tag", tag.as_str()])
                        .unwrap_or(vec![]),
                )
                .arg(".")
                .args(["--progress", "plain"])
                .stderr(Stdio::piped())
                .spawn()?
                .stderr
                .unwrap();

            let mut stderr_captured = String::new();
            let mut buffer = [0; 1024];
            while let Ok(amount) = stderr.read(&mut buffer) {
                if amount == 0 {
                    break;
                }
                stderr_captured.push_str(
                    str::from_utf8(&buffer[0..amount])
                        .expect("`docker build` produced invalid utf8 output"),
                );
                io::stderr()
                    .write_all(&buffer[0..amount])
                    .expect("failed to write to stderr");
                io::stderr().flush().expect("failed to flush stderr");
            }

            let last_line = stderr_captured
                .lines()
                .find(|line| line.contains("writing image sha256:"))
                .expect("`docker build` did not write image");
            let hash = last_line
                .split(' ')
                .map(str::trim)
                .find_map(|segment| segment.strip_prefix("sha256:"))
                .expect("no hash in `docker build` output");
            log_image(hash, build_command)
        }
        Subcommand::List(_list_command) => {
            let logged_images = retrieve_logged_images()?;
            println!(
                "{}",
                serde_json::to_string(&logged_images).expect("failed to serialize")
            );
            Ok(())
        }
        Subcommand::Clean(_clean_command) => {
            let mut logged_images = retrieve_logged_images()?;
            for (image_hash, _) in logged_images.clone() {
                let image_info_output = Command::new("docker")
                    .arg("image")
                    .arg("inspect")
                    .arg(&image_hash)
                    .output()?;
                let stdout = String::from_utf8(image_info_output.stdout)
                    .expect("`docker image inspect` output was unvalid utf8");
                let image_info: serde_json::Value = serde_json::from_str(&stdout)?;
                if image_info[0]["Config"]["Labels"]["tool"]
                    .as_str()
                    .map(|value| value == "spade-docker")
                    .unwrap_or_default()
                {
                    if let Ok(exit_status) = Command::new("docker")
                        .args(["rmi", "-f", &image_hash])
                        .spawn()
                        .and_then(|mut child| child.wait())
                    {
                        if exit_status.success() {
                            logged_images.remove(&image_hash);
                            try_update_log(&logged_images)?;
                        }
                    }
                } else {
                    println!("No longer tracking {} because the hash refers to an image not generated by this tool", image_hash);
                    logged_images.remove(&image_hash);
                    try_update_log(&logged_images)?;
                }
            }
            Ok(())
        }
        Subcommand::DataDirectory(_data_directory_command) => {
            println!("{}", data_dir().to_string_lossy());
            Ok(())
        }
    }
}
