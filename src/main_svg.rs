use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
fn main() {
    let captcha: String = thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    let text = svg::node::element::Text::new(captcha.clone())
        .set("x", 10)
        .set("y", 30)
        .set("font-size", 20);

    let document = Document::new().add(text);

    svg::save("image.svg", &document).unwrap();
}
