use copypasta_ext::prelude::*;
use copypasta_ext::x11_fork::ClipboardContext;
use snilsgen::Snils;

const COMMAND_NAME: &str = "snilsgen";
const COMMAND_VERSION: &str = "0.1.1";

fn help() {
    println!(
        "Generate a new SNILS value.

Usage:
 {COMMAND_NAME} [options]

Options:
 -c, --clipboard  copy value to the clipboard

 -h, --help       display this help
 -V, --version    display version"
    );
}

fn version() {
    println!("{COMMAND_NAME} {COMMAND_VERSION}");
}

fn invalid_arg(arg: &str) {
    println!(
        "{COMMAND_NAME}: invalid option -- '{}'
Try '{COMMAND_NAME}' --help' for more information.",
        arg.trim_matches('-')
    );
}

fn generate() {
    println!("{}", Snils::new());
}

fn generate_with_clipboard() {
    let snils = Snils::new();

    println!("{}", snils);

    if let Ok(mut ctx) = ClipboardContext::new() {
        match ctx.set_contents(snils.to_string().to_owned()) {
            Ok(_) => println!("Snils copied to clipboard."),
            Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
        }
    } else {
        eprintln!("Clipboard is not available on this system.");
    }
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or("".to_string());

    match arg.as_str() {
        "-h" | "--help" => help(),
        "-V" | "--version" => version(),
        "-c" | "--clipboard" => generate_with_clipboard(),
        "" => generate(),
        arg => invalid_arg(arg),
    }
}
