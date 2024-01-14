#[macro_export]
macro_rules! prepare_debug_log {
    () => {{
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true) // Truncate the file if it already exists
            .write(true)
            .open("debug_log.txt")
            .expect("Failed to open or create debug_log.txt");

        file.flush().expect("Failed to flush debug_log.txt");
    }};
}

#[macro_export]
macro_rules! debug_log {
    ($log:expr) => {{
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("debug_log.txt")
            .expect("Failed to open or create debug_log.txt");

        for line in $log.lines() {
            writeln!(file, "{}\n", line).expect("Failed to write to debug_log.txt");
        }

        file.flush().expect("Failed to flush debug_log.txt");
    }};
}
