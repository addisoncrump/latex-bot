#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate groupme_bot;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate regex;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use groupme_bot::Groupme;
use std::string::String;
use rocket_contrib::Json;
use regex::Regex;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[derive(Deserialize)]
pub struct GroupmeCallback {
    text: String,
}

fn main() {
    #[post("/<botid>", format = "application/json", data = "<callback>")]
    fn handle_post(botid: String, callback: Json<GroupmeCallback>) {
        let groupme = Groupme::new(None);
        let bot = groupme.bot(&botid);
        if callback.0.text.starts_with("!latex ") {
            let preamble = concat!("\\usepackage{amsmath}\n",
                                   "\\usepackage{amsfonts}\n",
                                   "\\usepackage{amssymb}\n\\");
            let mut formula = String::new();
            formula += &callback.0.text[7..];
            formula = formula.replace("%", "%25");
            formula = formula.replace("&", "%26");
            let mut body = String::new();
            body = format!("{}{}{}", body, "formula=", formula);
            body += "&fsize=100px";
            body += "&fcolor=00AFF0";
            body += "&mode=0";
            body += "&out=1&remhost=localhost";
            body = format!("{}{}{}", body, "&preamble=", preamble);
            let client = reqwest::Client::new();
            let res = client.post("http://www.quicklatex.com/latex3.f")
                .body(body)
                .send()
                .unwrap()
                .text()
                .unwrap();
            lazy_static! {
                static ref RE: Regex = Regex::new("(http://quicklatex.com/[\\w/\\.]+)").unwrap();
            }
            match RE.captures_iter(res.as_str()).last() {
                Some(link) => {
                    bot.post(&link[0]).unwrap();
                    println!("{}", &link[0])
                }
                _ => bot.post("There was an error interpreting your LaTeX.").unwrap(),
            }
        }
    }

    #[get("/<file..>")]
    fn static_content(file: PathBuf) -> Option<NamedFile> {
        NamedFile::open(Path::new("static/").join(file)).ok()
    }

    #[get("/")]
    fn index() -> Option<NamedFile> {
        NamedFile::open(Path::new("static/index.html")).ok()
    }

    rocket::ignite().mount("/", routes![handle_post, index, static_content]).launch();
}
