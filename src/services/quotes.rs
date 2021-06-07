use serde::Serialize;

#[derive(Serialize)]
pub struct Quote {
    saying: String,
    author: String,
}

impl Quote {
    pub fn new(saying:String, author:String) -> Quote {
        Quote {
            saying,
            author,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} - {}",self.saying,self.author)
    }

}

pub fn themes() -> [String;1] {
    [String::from("reading")]
}

pub fn whatsapp(theme:Option<String>) -> String {
    get(theme).to_string()
}

pub fn get(theme:Option<String>) -> Quote {
    match theme.unwrap_or(format!("_")).as_str() {
        "reading" => reading(),
        _ => reading()
    }

}

fn reading() -> Quote {
    Quote::new(
        String::from("Today a reader, tomorrow a leader."),
        String::from("Margaret Fuller")
    )
}
