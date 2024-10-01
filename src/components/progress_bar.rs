use indicatif::{ProgressBar, ProgressStyle};

pub fn progressBar() -> ProgressBar{
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("=>-")
    );
    progress_bar.set_message("Progress");
    progress_bar
}