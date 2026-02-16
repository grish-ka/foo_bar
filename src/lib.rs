use clap::Parser;
use indicatif::ProgressStyle;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Allow force mode (skip confirmations)
    #[arg(short, long)]
    pub force: bool,

    /// Number of parallel jobs to run
    #[arg(short = 'j', long, default_value_t = 4)]
    pub jobs: usize,
}

pub static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
    "react-dom",
    "lodash",
];

pub static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "gcc foo.c -o foo",
    "npm install",
    "optimizing assets",
    "linking binary",
    "rebuilding cache",
];

// Logic functions moved here!
// Notice the "pub" keyword -- this lets other files use them.
// 1. Turn it into a function
pub fn get_header_style() -> ProgressStyle {
    ProgressStyle::with_template("{prefix:.bold.dim} {msg}")
        .expect("Failed to create header progress style")
}

// 2. Turn this into a function too
pub fn get_wide_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{msg}\n    {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .expect("Failed to create wide progress style")
    .progress_chars("#>-")
}
pub fn is_force_enabled(args: &Args) -> bool {
    args.force
}

pub fn calculate_total_size(deps: &[(&str, u32)]) -> u64 {
    deps.iter().map(|(_name, size)| *size as u64).sum()
}
