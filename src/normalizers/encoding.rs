use chardetng::EncodingDetector;
use tracing::trace;

pub fn to_utf8(raw_text: &[u8]) -> String {
    let mut detector = EncodingDetector::new();

    if !detector.feed(&raw_text, true) {
        return std::str::from_utf8(raw_text)
            .expect("Malformed ASCII")
            .to_string();
    }

    let encoding = detector.guess(None, true);
    trace!("Encoding chosen is {}", encoding.name());

    let (string, malformed) = encoding.decode_with_bom_removal(raw_text);
    if malformed {
        panic!("There were malformed sequences on a given text")
    }

    string.to_string()
}
