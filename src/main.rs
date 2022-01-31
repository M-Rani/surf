use clap::{App, Arg};
use colored::Colorize;
use webbrowser;

fn url_search_querey(search_querey: &str, engine: &str, print_out: bool) {
    let url: String = engine.to_owned() + search_querey;
    let target = &url[0..url.len()-1];

    if webbrowser::open(target).is_ok() {
        if print_out {
            print!("{}... {} {}",
            format!("Surfing").green().bold(),
            target,
            format!("✓").green().bold());
        }
    }
}

fn random_search_querey(print_out: bool) {
    if webbrowser::open("https://wiby.me/surprise/").is_ok() {
            if print_out {
                print!("{}... {} {}",
                format!("Surfing").green().bold(),
                "random",
                format!("✓").green().bold());
            }
        }
}

fn open_target_link(target : &str, print_out : bool) {
    if webbrowser::open(target).is_ok() {
        if print_out {
            print!("{}... {} {}",
            format!("Surfing").green().bold(),
            target,
            format!("✓").green().bold());
        }
    }
}

fn url_custom_querey(domain: String) -> String {
    let custom_engine: String = "https://".to_owned() + &domain[..] + ".com/?q=";
    return custom_engine;
}

fn main() {
    let matches = App::new("Search")
    //customized help screen
    .override_help("        Surf 2.0\n            Surf the web from the command line

        USAGE: 
            surf [OPTIONS] <search-querey>...

        OPTIONS:
            -d, --duckduckgo                surf with duckduckgo [default]
            -g, --google                    surf with google
                --wiby                      surf with wiby
                --yandex                    surf with yandex

            -h, --help                      print help information
            -v, --verbose                   output to console
            -r, --random                    open a random webpage
            -u, --unsafe                    treat querey as file, open with default app
            -l, --link                      open querey as https://<search-querey>
            -e, --search <search-egine>     enter your own search engine, may or may not work")
    //arguments
    .arg(Arg::new("search-querey")
        .required(false)
        .multiple_values(true))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose"))
    //search engines
    .arg(Arg::new("use-duckduckgo")
        .short('d')
        .long("duckduckgo"))
    .arg(Arg::new("use-google")
        .short('g')
        .long("google"))
    .arg(Arg::new("use-wiby")
        .long("wiby"))
    .arg(Arg::new("random")
        .short('r')
        .long("random"))
    .arg(Arg::new("use-yandex")
        .long("yandex"))
    .arg(Arg::new("use-amazon")
        .long("amazon"))
    //legacy surf options
    .arg(Arg::new("unsafe")
        .short('u')
        .long("unsafe"))
    .arg(Arg::new("as-url")
        .short('l')
        .long("link"))
    //custom search engine?
    .arg(Arg::new("search-engine")
        .short('e')
        .long("search-engine")
        .takes_value(true))
    .get_matches();

    //initialize search querey, current search engine
    let mut search_request: String = "".to_string();
    let mut current_engine: &str = "https://duckduckgo.com/?q=";
    let is_verbose = matches.is_present("verbose");
    let is_direct: bool = matches.is_present("as-url") || matches.is_present("unsafe");

    //construct search querey if present, format for url
    if matches.is_present("search-querey") && !is_direct {
        if let Some(items) = matches.values_of("search-querey") {
             for item in items {
                let s = item
                .replace("+","%2B")
                .replace("/","%2F")
                .replace("\\","%5C");
                search_request += &s.to_owned();
                search_request += &"+".to_owned();
            }  
        }
    }
    else {
        if let Some(item) = matches.value_of("search-querey") {
            search_request += &item.to_owned();
        }
    }

    if matches.is_present("search-querey") {
        let target: &str = &search_request[..];
        
        if matches.is_present("search-engine") && !is_direct {
            if let Some(ov) = matches.value_of("search-engine") {
                let selected_engine = ov;
                let custom: String = url_custom_querey(ov.to_string());

                current_engine = match selected_engine {
                    "google"              =>"https://google.com/search?q=",
                    "duckduckgo"          =>"https://duckduckgo.com/?q=",
                    "yahoo"               =>"https://search.yahoo.com/search?q=",
                    "wiby"                =>"https://wiby.me/?q=",
                    "yandex"              =>"https://yandex.com/search/?text=",
                    "openverse"           =>"https://wordpress.org/openverse/search/?q=",
                    "amazon"              =>"https://www.amazon.com/s?k=",
                    "aol"                 =>"https://search.aol.ca/aol/search?q=",
                    "wolfram"             =>"https://www.wolframalpha.com/input/?i=",
                    "math"                =>"https://www.wolframalpha.com/input/?i2d=true&i=",
                    "icon" | "iconfinder" =>"https://www.iconfinder.com/search?q=",
                    "answers"             =>"https://www.answers.com/search?q=",
                    "webcrawler"          =>"https://www.webcrawler.com/serp?q=",
                    _=>&custom[..]
                };
                url_search_querey(target, current_engine, is_verbose);
            }
        }
        else if matches.is_present("use-duckduckgo") {
            current_engine="https://duckduckgo.com/?q=";
            url_search_querey(target, current_engine, is_verbose);
        }
        else if matches.is_present("use-google") {
            current_engine="https://google.com/search?q=";
            url_search_querey(target, current_engine, is_verbose);
        }
        else if matches.is_present("use-wiby") {
            current_engine="https://wiby.me/?q=";
            url_search_querey(target, current_engine, is_verbose);
        }
        else if matches.is_present("use-yandex") {
            current_engine="https://yandex.com/search/?text=";
            url_search_querey(target, current_engine, is_verbose);
        }
        else if matches.is_present("use-amazon") {
            current_engine="https://www.amazon.com/s?k=";
            url_search_querey(target, current_engine, is_verbose);
        }
        else if matches.is_present("random") { 
            random_search_querey(is_verbose);
        }
        else if is_direct {
            if target.contains("https://") || matches.is_present("unsafe") {
                //use current link without editing
                open_target_link(target, is_verbose);
            }
            else {
                //edit url to have https://
                let unsafe_url: String = "https://".to_owned() + target;
                let safe_url: &str = &unsafe_url[..];
                open_target_link(safe_url, is_verbose);
            }
        }
        else {
            //use DuckDuckGo as default engine
            url_search_querey(target, current_engine, is_verbose);
        }
    } 
    else if matches.is_present("random") {    
        random_search_querey(is_verbose);
    }
    else {
        if is_verbose {
            print!("No querey...");
        }
    }

    if is_verbose {
        print!("\n");
    }
}