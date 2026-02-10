use console::{Emoji, style};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use rand::Rng;
use rand::seq::SliceRandom;
use std::thread;
use std::time::{Duration, Instant};

static PACKAGES: &[&str] = &[
    "fs-events",
    "my-awesome-module",
    "emoji-speaker",
    "wrap-ansi",
    "stream-browserify",
    "acorn-dynamic-import",
    "react-dom",
    "lodash",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "gcc foo.c -o foo",
    "npm install",
    "optimizing assets",
    "linking binary",
    "rebuilding cache",
];

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚 ", "");
static CLIP: Emoji<'_, '_> = Emoji("🔗 ", "");
static PAPER: Emoji<'_, '_> = Emoji("📃 ", "");
static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", ":-)");

fn main() {
    let started = Instant::now();
    let m = MultiProgress::new();

    // 1. Header Style for the [1/4] titles
    let header_style = ProgressStyle::with_template("{prefix:.bold.dim} {msg}").unwrap();

    // 2. The Wide Stacked Style you wanted: Message on top, 40-char bar below
    let wide_stacked_style = ProgressStyle::with_template(
        "{msg}\n    {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )
    .unwrap()
    .progress_chars("#>-");

    // --- STEP 1: RESOLVING ---
    let h1 = m.add(ProgressBar::new_spinner());
    h1.set_style(header_style.clone());
    h1.set_prefix("[1/4]");
    h1.set_message(format!("{} Resolving packages...", LOOKING_GLASS));
    thread::sleep(Duration::from_secs(2)); // Artificial pause
    h1.finish();

    // --- STEP 2: FETCHING ---
    let h2 = m.add(ProgressBar::new_spinner());
    h2.set_style(header_style.clone());
    h2.set_prefix("[2/4]");
    h2.set_message(format!("{} Fetching packages...", TRUCK));

    // Simulate 3 parallel downloads that take a while
    let mut handles = vec![];
    let deps = vec![("core-api", 2000), ("ui-theme", 1500), ("db-driver", 3000)];

    for (name, size) in deps {
        let pb = m.add(ProgressBar::new(size));
        pb.set_style(wide_stacked_style.clone());
        pb.set_message(format!(
            "  {} Downloading {}",
            style("→").blue(),
            style(name).bold()
        ));
        pb.enable_steady_tick(Duration::from_millis(100));

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            // This loop ensures each bar takes roughly 10-15 seconds
            for _ in 0..100 {
                pb.inc(size / 100);
                let speed = rng.gen_range(100..150);
                thread::sleep(Duration::from_millis(speed));
            }
            pb.finish_and_clear();
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
    h2.finish();

    // --- STEP 3: LINKING ---
    let h3 = m.add(ProgressBar::new_spinner());
    h3.set_style(header_style.clone());
    h3.set_prefix("[3/4]");
    h3.set_message(format!("{} Linking dependencies...", CLIP));

    let pb_link = m.add(ProgressBar::new(1000));
    pb_link.set_style(wide_stacked_style.clone());
    pb_link.set_message(format!("  {} Running system linker", style("→").blue()));

    for _ in 0..100 {
        pb_link.inc(10);
        thread::sleep(Duration::from_millis(80)); // 8 second slow crawl
    }
    pb_link.finish_and_clear();
    h3.finish();

    // --- STEP 4: BUILDING ---
    let h4 = m.add(ProgressBar::new_spinner());
    h4.set_style(header_style);
    h4.set_prefix("[4/4]");
    h4.set_message(format!("{} Building fresh packages...", PAPER));

    // Final parallel "building" step
    let mut build_handles = vec![];
    for i in 1..=3 {
        let pb = m.add(ProgressBar::new(50));
        pb.set_style(wide_stacked_style.clone());
        let mut rng = rand::thread_rng();
        let pkg = PACKAGES.choose(&mut rng).unwrap();

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..50 {
                let cmd = COMMANDS.choose(&mut rng).unwrap();
                pb.set_message(format!("  {} {}: {}", style("🔨").yellow(), pkg, cmd));
                pb.inc(1);
                thread::sleep(Duration::from_millis(rng.gen_range(100..250)));
            }
            pb.finish_and_clear();
        });
        build_handles.push(handle);
    }
    for h in build_handles {
        let _ = h.join();
    }
    h4.finish();

    println!("\n{} Done in {}", SPARKLE, HumanDuration(started.elapsed()));
}
