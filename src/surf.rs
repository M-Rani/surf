use webbrowser;
use colored::Colorize;

//open a target link
pub fn open_file(target: &str, print_out: bool) {
    if webbrowser::open(target).is_ok() {
        if print_out {
            println!(
                "{}... {} [ {} ]",
                format!("Surfin'").green().bold(),
                target,
                format!("OK").green().bold()
            )
        }
    }
}

//open a random link
pub fn open_random_url(print_out: bool) {
    if webbrowser::open("https://wiby.me/surprise/").is_ok() {
        if print_out {
            println!(
                "{}... {}",
                format!("Surfin'").green().bold(),
                format!("???").bold()
            );
        }
    }
}

pub fn url_custom_querey(domain: String) -> String {
	let custom_engine: String = "https://".to_owned() + &domain[..] + ".com/?q=";
	return custom_engine;
}
