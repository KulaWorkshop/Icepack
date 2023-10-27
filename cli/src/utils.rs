use colored::Colorize;

pub fn display_error(message: &str) {
    eprintln!("{}{} {}", "error".red().bold(), ":".bold(), message);
    std::process::exit(1);
}

pub fn display_warning(message: &str) {
    println!("{}{} {}", "warning".yellow().bold(), ":".bold(), message);
}
