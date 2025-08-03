mod json;
mod lex;

fn main() {
    let json = String::from("{\"name\":\"Amin\",\"age\":25,\"good\":true,\"job\":null,\"arr\":[1,2,null,\"str\",{\"name\":\"John\"}]}");
    // let json2 = String::from("[{\"name\":\"Amin\",\"age\":25},{\"name\":\"John\",\"age\":25}]");
    match lex::tokenize(json) {
        Ok(tokens) => {
            match lex::parse(tokens.clone()) {
                Ok(jsvalue) => {
                    dbg!(jsvalue);
                }
                Err(_) => {}
            }
            println!("tokens are {:?}", &tokens);
        }
        Err(err) => {
            println!("error extracting tokens: {}", err);
        }
    }
    println!("Hello, world!");
}
