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
use std::env;

#[derive(Deserialize)]
pub struct GroupmeCallback {
    text: String
}

fn main() {
//    let token = &env::var("GROUPME_TOKEN").unwrap();
//    let group_id = &env::var("GROUPME_GROUP").unwrap();

//    let groupme = Groupme::new(Some(token));
//    let bot = groupme
//            .create_bot("LaTeX", group_id)
//            .unwrap()
//            .with_avatar_url("https://i.groupme.com/2400x2400.png.d26f7326928f4f35ba1af10a9228417b")
//            .create()
//            .unwrap();

    #[post("/<botid>", format = "application/json", data = "<callback>")]
    fn index(botid: std::string::String, callback: rocket_contrib::Json<GroupmeCallback>) {let groupme = Groupme::new(None);
        let bot = groupme.bot(&botid);
        if callback.0.text.starts_with("!latex ") {
            let preamble = "\\usepackage{amsmath}\n\\usepackage{amsfonts}\n\\usepackage{amssymb}\n\\";
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
                .send().unwrap()
                .text().unwrap();
            lazy_static! {
                static ref RE: regex::Regex = regex::Regex::new("(http://quicklatex.com/[\\w/\\.]+)").unwrap();
            }
            match RE.captures_iter(res.as_str()).last() {
                Some(link) => {
                    bot.post(&link[0]).unwrap();
                    println!("{}", &link[0])
                }
                _ => bot.post(&res).unwrap()
            }
        }
    }

    rocket::ignite().mount("/", routes![index]).launch();
}