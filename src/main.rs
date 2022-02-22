use clap::{App, Arg};
mod surf;
pub mod useless_web;

fn main() {
    //custom help message
	let help_message = 
    "	Surf 3.0\n            open Urls from the command line

	USAGE: 
		surf [OPTIONS] <search-querey>...

	OPTIONS:
		-d, --duckduckgo                search with duckduckgo [default]
		-g, --google                    search with google
	            --youtube                   search with youtube
		    --amazon                    search with amazon
		    --wiby                      search with wiby
	    	    --yandex                    search with yandex

		-h, --help                      print help information
		-s, --silent                    don't output to console
		-r, --random                    open a random webpage
		-u, --unsafe                    treat querey as file, open with default app
		-l, --link                      open querey as https://<search-querey> 
		-e, --search <search-egine>     enter your own search engine, may or may not work
        ";
   
    let matches = App::new("Surf")
        //handle cli argument parsing
        .override_help(help_message)
        .arg(Arg::new("target-url")
            .multiple_values(true))
        
        //special arguments
        .arg(Arg::new("silent")
            .short('s')
            .long("silent"))
        .arg(Arg::new("unsafe")
             .short('u')
             .long("unsafe"))
        .arg(Arg::new("as-url")
             .short('l')
             .long("link"))
        .arg(Arg::new("random")
             .short('r')
             .long("random"))
        .arg(Arg::new("search-engine")
             .short('e')
             .long("search-egine")
             .takes_value(true))
        
        //easy search engines
        .arg(Arg::new("use-duckduckgo")
             .short('d')
             .long("duckduckgo"))
        .arg(Arg::new("use-google")
             .short('g')
             .long("google"))
        .arg(Arg::new("use-yandex")
             .long("yandex"))
        .arg(Arg::new("use-amazon")
             .long("amazon"))
        .arg(Arg::new("use-youtube")
             .long("youtube"))
        
        .get_matches();

    //init variables
    let mut search_request: String = "".to_string();
    let mut search_engine: &str = "http://duckduckgo.com/?q=";
    let custom: String;

    //init bools
    let is_valid: bool = matches.is_present("target-url");
    let has_selection: bool = matches.is_present("search-engine");
    let is_random: bool = matches.is_present("random");
    let is_silent: bool = matches.is_present("silent");

    let is_unsafe: bool = matches.is_present("unsafe");
    let is_url: bool = matches.is_present("as-url");
    let is_direct: bool = is_unsafe || is_url;

    let mut keep_spaces: bool = false;
    let mut add_to_url: bool = false;
    
    let unformatted_target = matches.values_of("target-url");

    if has_selection {
        if let Some(ov) = matches.value_of("search-engine") {
            let selected_engine = ov;
            custom = surf::url_custom_querey(ov.to_string());

            search_engine = match selected_engine {
				"google" => "https://google.com/search?q=",
				"duckduckgo" => "https://duckduckgo.com/?q=",
				"youtube" | "y" => "https://www.youtube.com/results?search_query=",
				"youtube-music" => "https://music.youtube.com/search?q=",
				"spotify" => "https://open.spotify.com/search/",
				"yahoo" => "https://search.yahoo.com/search?q=",
				"wiby" => "https://wiby.me/?q=",
				"yandex" => "https://yandex.com/search/?text=",
				"openverse" => "https://wordpress.org/openverse/search/?q=",
				"amazon" => "https://www.amazon.com/s?k=",
				"aol" => "https://search.aol.ca/aol/search?q=",
				"wolfram" => "https://www.wolframalpha.com/input/?i=",
				"math" => "https://www.wolframalpha.com/input/?i2d=true&i=",
				"icon" | "iconfinder" => "https://www.iconfinder.com/search?q=",
				"answers" => "https://www.answers.com/search?q=",
				"webcrawler" => "https://www.webcrawler.com/serp?q=",
                "soundcloud" => "https://soundcloud.com/search?q=",
                "1337x" => "https://www.1377x.to/search/",
                "piratebay" | "pirate" | "hidden" => "https://thehiddenbay.com/search/",
                _ => &custom[..],
            };

            if selected_engine == "spotify" || selected_engine == "1337x" || selected_engine == "piratebay" {
                keep_spaces = true;
            }

            if selected_engine == "1337x" { 
               add_to_url = true; 
            }
        }
    }
    else {
        if matches.is_present("use-duckduckgo") {
            search_engine = "https://duckduckgo.com/?q=";
        }
        else if matches.is_present("use-google") {
            search_engine = "https://google.com/search?q=";
        }
        else if matches.is_present("use-yandex") {
            search_engine = "https://yandex.com/search/?text=";
        }
        else if matches.is_present("use-amazon") {
            search_engine = "https://www.amazon.com/s?k=";
        }
        else if matches.is_present("use-youtube") {
            search_engine = "https://www.youtube.com/results?search_query=";
        }
    }

    if is_valid && !is_direct{
        search_request += &search_engine.to_owned();

        if let Some(items) = unformatted_target {
            for item in items {
                if keep_spaces || is_direct {
                    let s = item;
                    search_request += &s.to_owned();
                    search_request += &" ".to_owned();
                    //get rid of the last space in search_request
                }
                else {
                    let s = item 
                        .replace("+","%2B")
                        .replace("/","%2F")
                        .replace("\\","%5C")
                        .replace(" ","+");
                    search_request += &s.to_owned();
                    search_request += &"+".to_owned();
                    //get rid of the last space in search_request
                }
            }
            search_request = (&search_request[..search_request.len()-1]).to_string();

            //add to url
            if add_to_url {
                search_request += &"/1".to_owned();
            }

            //open file 
            surf::open_file(&search_request[..],!is_silent); 
        }
    } 
    else if is_random {
        //open a random web link
        surf::open_random_url(!is_silent);
    }
    else if is_direct {
        if is_url {
            //read unformatted_target
            if let Some(items) = unformatted_target {
                for item in items {
                    let s = item;
                    search_request += &s.to_owned();
                }
            }
            
            //add https:// if not already there
            if !search_request.contains("https://") {
                search_request = "https://".to_owned() + &search_request;
            }

            //add .com if not already there
            if !search_request.contains(".com") {
                search_request += &".com".to_owned();
            }
            surf::open_file(&search_request[..],!is_silent);
        } 
        else if is_unsafe {
            //read unformatted_target
            if let Some(items) = unformatted_target {
                for item in items {
                    let s = item;
                    search_request += &s.to_owned();
                }
            }
            surf::open_file(&search_request[..],!is_silent);
        }
    }
    else {
        //print help message when program does not meet minimum requirements
		println!("        BAD USAGE:\n            -h, --help to display help message");
    }
}
