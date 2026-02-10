use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

fn main() {
    let total_size = 1000; // Simulating a total size of 1 KB (1000 bytes)

    // 1. Create the progress bar
    let pb = ProgressBar::new(total_size);

    // 2. Define a custom Style
    // {bar:40.cyan/blue} -> 40 chars wide, cyan filled, blue empty
    // {bytes}/{total_bytes} -> Automatic unit conversion
    // {eta} -> Estimated Time of Arrival
    // {spinner:.green} -> A spinner that is green
    // {msg} -> Custom message that we will set in the loop
    pb.set_style(
        ProgressStyle::with_template(
            "{msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("#>-"),
    ); // The characters used for the bar itself

    println!(
        "{}\n{}",
        style("Update detected").bold().green(),
        style("Starting high-speed download...").bold().green()
    );

    // 3. The Loop (Simulating actual work)
    for i in 0..total_size / 10 {
        pb.set_message(format!("Downloading chunk #{}", i));
        pb.inc(10); // Increment by 100 bytes (simulate downloading 100 bytes per iteration)
        // Simulate varying speeds
        thread::sleep(Duration::from_millis(50));
    }

    // 4. Finish
    pb.finish_with_message("Download Complete!");
    println!("{}", style("✔ Update saved to disk.").bold().yellow());

    println!(
        "{}\n{}",
        style("Dependencies detected (foo, bar)").bold().green(),
        style("Starting high-speed download...").bold().green()
    );
    pb.finish_with_message("Download Complete!");
    println!("{}", style("✔ Dependency saved to disk.").bold().yellow());
    pb.reset(); // Reset the progress bar for installation

    // 3. The Loop (Simulating actual work)
    for i in 0..total_size / 10 {
        pb.set_message(format!("Downloading chunk #{} of foo", i));
        pb.inc(10); // Increment by 100 bytes (simulate downloading 100 bytes per iteration)
        // Simulate varying speeds
        thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_message("Download Complete!");
    println!("{}", style("✔ Dependency saved to disk.").bold().yellow());

    pb.reset(); // Reset the progress bar for installation

    // 3. The Loop (Simulating actual work)
    for i in 0..total_size / 10 {
        pb.set_message(format!("Downloading chunk #{} of bar", i));
        pb.inc(10); // Increment by 100 bytes (simulate downloading 100 bytes per iteration)
        // Simulate varying speeds
        thread::sleep(Duration::from_millis(50));
    }
    // 5. Simulate Installation
    println!("{}", style("Starting verification...").bold().green());
    pb.reset(); // Reset the progress bar for installation

    for i in 0..total_size / 10 {
        pb.set_message(format!("Veryifing chunk #{}", i));
        pb.inc(10); // Increment by 100 bytes (simulate downloading 100 bytes per iteration)
        // Simulate varying speeds
        thread::sleep(Duration::from_millis(50));
    }
    // 6. Finish
    pb.finish_with_message("Verification Complete!");
    println!("{}", style("✔ Updated app.").bold().yellow());
}
