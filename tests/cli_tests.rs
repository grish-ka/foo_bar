#[cfg(test)]
mod tests {
    // use super::*; // Import functions from the main part
    use console::style;
    use foo_bar::*;
    use indicatif::{MultiProgress, ProgressBar};
    use rand::seq::SliceRandom;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_size_math() {
        // Create some fake data
        let fake_deps = vec![("Small Lib", 100), ("Big Lib", 200)];

        // Use our function
        let result = calculate_total_size(&fake_deps);

        // Assert that 100 + 200 = 300
        assert_eq!(result, 300);
    }

    #[test]
    fn test_single_task_bar() {
        // 1. Setup the manager
        let m = MultiProgress::new();

        // 2. Define the style you want to test (Header Style)
        let header_style = get_header_style();

        let wide_stacked_style = get_wide_style();

        // 3. Create the bar
        let th = m.add(ProgressBar::new_spinner());
        th.set_style(header_style);
        th.set_prefix("[TEST]");
        th.set_message("Checking multi bar functionality...");

        let mut build_handles = vec![];
        for _i in 1..=3 {
            let pb = m.add(ProgressBar::new(20));
            pb.set_style(wide_stacked_style.clone());
            let mut rng = rand::thread_rng();
            let pkg = PACKAGES.choose(&mut rng).expect("PACKAGES empty");

            let handle = thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for _ in 0..20 {
                    let cmd = COMMANDS.choose(&mut rng).expect("COMMANDS empty");
                    pb.set_message(format!("  {} {}: {}", style("🔨").yellow(), pkg, cmd));
                    pb.inc(1);
                    thread::sleep(Duration::from_millis(20));
                }
                pb.finish_and_clear();
            });
            build_handles.push(handle);
        }

        for h in build_handles {
            let _ = h.join();
        }

        th.finish();
    }
}
