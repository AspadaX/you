use std::time::Duration;

use indicatif::ProgressBar;

pub fn start_spinner(message: String) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_message(message);
    spinner.enable_steady_tick(Duration::from_millis(100));
    
    spinner
}