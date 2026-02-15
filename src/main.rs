use console::{Emoji, Term, style};
// Removed 'ProgressStyle' from here because it is unused
use foo_bar::*;
use indicatif::{HumanBytes, HumanDuration, MultiProgress, ProgressBar};
use rand::Rng;
use rand::seq::SliceRandom;
use std::env;
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
    let args: Vec<String> = env::args().collect();
    let force = args.iter().any(|arg| arg == "--force" || arg == "-f");

    if force {
        println!(
            "{}",
            style("! Force mode enabled. Skipping confirmations...")
                .yellow()
                .bold()
        );
    }

    let started = Instant::now();
    let m = MultiProgress::new();
    let header_style = get_header_style();
    let wide_stacked_style = get_wide_style();
    let loadsec = 3;

    // --- STEP 1: RESOLVING ---
    let h1 = m.add(ProgressBar::new_spinner());
    for _ in 0..(20 * loadsec) {
        h1.inc(1);
        thread::sleep(Duration::from_millis(50));
    }
    h1.set_style(header_style.clone());
    h1.set_prefix("[1/4]");
    h1.set_message(format!("{} Resolving packages...", LOOKING_GLASS));
    thread::sleep(Duration::from_secs(2));
    h1.finish();

    // --- STEP 2: FETCHING ---
    let h2 = m.add(ProgressBar::new_spinner());
    h2.set_style(header_style.clone());
    h2.set_prefix("[2/4]");
    h2.set_message(format!("{} Fetching packages...", TRUCK));

    let deps = vec![("core-api", 2000), ("ui-theme", 1500), ("db-driver", 3000)];
    let total_fetch_size: u64 = calculate_total_size(&deps);

    if !is_force_enabled(&args) {
        println!(
            "{} {}",
            style("Fetcher:").bold().dim(),
            style("Ready to fetch packages?").cyan()
        );

        println!(
            "{} {}{}{}",
            style("Fetcher:").bold().dim(),
            style("After this operation, ").cyan(),
            style(HumanBytes(total_fetch_size)).bold().cyan(),
            style(" of additional disk space will be used.").cyan()
        );

        println!(
            "{} {} [{}/{}]",
            style("Fetcher:").bold().dim(),
            style("Do you want to proceed?").cyan(),
            style("Y").bold().green(),
            style("n").bold().red()
        );

        let term = Term::stdout();
        match term.read_char() {
            Ok('y') | Ok('Y') | Ok('\n') => {
                println!("{}\n", style("Proceeding...").italic().dim());
            }
            _ => {
                h2.abandon();
                println!("{}", style("Installation aborted by user.").red());
                return;
            }
        }
    }

    let mut handles = vec![];
    for (name, size) in deps {
        // FIX 1: Convert size to u64 here
        let size = size as u64;

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
            for _ in 0..100 {
                // FIX 2: Now 'size' is already u64, so this math works
                pb.inc(std::cmp::max(1, size / 100));
                let speed = rng.gen_range(100..150);
                thread::sleep(Duration::from_millis(speed));
            }
            pb.finish_and_clear();
        });
        handles.push(handle);
    }

    for h in handles {
        if let Err(e) = h.join() {
            eprintln!("Download thread panicked: {:?}", e);
        }
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
        thread::sleep(Duration::from_millis(80));
    }
    pb_link.finish_and_clear();
    h3.finish();

    // --- STEP 4: BUILDING ---
    let h4 = m.add(ProgressBar::new_spinner());
    h4.set_style(header_style.clone());
    h4.set_prefix("[4/4]");
    h4.set_message(format!("{} Building fresh packages...", PAPER));
    h4.enable_steady_tick(Duration::from_millis(100));

    let mut build_handles = vec![];
    for _i in 1..=3 {
        let pb = m.add(ProgressBar::new(50));
        pb.set_style(wide_stacked_style.clone());
        let mut rng = rand::thread_rng();
        let pkg = PACKAGES.choose(&mut rng).expect("PACKAGES empty");

        let handle = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..50 {
                let cmd = COMMANDS.choose(&mut rng).expect("COMMANDS empty");
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
