use tracing::{subscriber::set_global_default, Level};

pub mod cli_args;

/// A macro to format a debug message with the file and line number
#[macro_export]
macro_rules! fdbg {
    ($msg:literal $(,)?) => {
        format!("{} - {}", format!("{}:{}", file!(), line!()), $msg)
    };
    ($fmt:expr, $($arg:tt)*) => (format!("{} {}", format!("{}:{}", file!(), line!()), format!($fmt, $($arg)*)));
}

pub fn setup_log() -> anyhow::Result<()> {
    color_eyre::install().expect("Unable to setup color eyre");
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .compact()
        // .without_time()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .with_max_level(Level::TRACE)
        .finish();
    set_global_default(subscriber)?;
    Ok(())
}
