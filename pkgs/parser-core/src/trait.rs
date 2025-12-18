use volumen_types::ParseResult;

pub trait VolumenParser {
    fn parse(source: &str, filename: &str) -> ParseResult;
}
