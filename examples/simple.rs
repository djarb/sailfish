#[macro_use]
extern crate sailfish_macros;

use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "simple.stpl")]
struct Simple {
    messages: Vec<String>,
}

fn main() {
    let messages = vec![String::from("Message 1"), String::from("<Message 2>")];
    println!("{}", Simple { messages }.render_once().unwrap());
}
