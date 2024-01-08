use celery_rust::frontend::celery_parser::CeleryParser;
fn main() {
    CeleryParser::from_file("example/number.clr")
}
