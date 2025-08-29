use console::style;

pub fn log(message: &str) {
    println!("{}", message);
}

pub fn ilog(message: &str) {
    println!("{} {}", style(">").cyan().bold(), message);
}

pub fn wlog(message: &str) {
    println!("{} {}", style("!").color256(208).bold(), message);
}

// pub fn elog(message: &str) {
//     println!("{} {}", style("🞪").red().bold(), message);
// }

pub fn slog(message: &str) {
    println!("{} {}", style("✓").color256(046).bold(), message);
}
