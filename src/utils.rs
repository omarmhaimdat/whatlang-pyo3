use indicatif::{ProgressBar, ProgressStyle};

/// Get a progress bar with a custom style.
/// # Examples
/// ```
/// use utils::get_progress_bar;
/// let progress_bar = get_progress_bar(100);
/// ```
pub(crate) fn get_progress_bar(total: u64, message: String) -> ProgressBar {
    let progress_bar = ProgressBar::new(total);

    progress_bar.set_style(ProgressStyle::with_template("{msg} | {spinner:.green} [{elapsed_precise}] [{wide_bar:.green/blue}] {human_pos}/{human_len} ({eta})")
        .unwrap()
        .progress_chars("##-"));
    progress_bar.set_message(message);
    progress_bar
}

/// convert String to colored String with ANSI escape codes
pub(crate) enum TermColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

/// Convert String to colored String with ANSI escape codes
/// # Examples
/// ```
/// use utils::colorize;
/// let colored_string = colorize("Hello World", TermColor::Red);
/// ```
pub(crate) fn colorize(text: &str, color: TermColor) -> String {
    let color_code = match color {
        TermColor::Red => 31,
        TermColor::Green => 32,
        TermColor::Yellow => 33,
        TermColor::Blue => 34,
        TermColor::Magenta => 35,
        TermColor::Cyan => 36,
        TermColor::White => 37,
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}

/// Convert whatlang::Lang code to iso639-1 code
/// # Examples
/// ```
/// use utils::lang_to_iso639_1;
/// let iso639_1_code = lang_to_iso639_1("eng");
/// assert_eq!(iso639_1_code, "en");
/// ```
pub(crate) fn lang_to_iso639_1(lang: &str) -> String {
    // convert whatlang::Lang code to iso639-1 code
    match lang {
        "afr" => "af",
        "ara" => "ar",
        "bul" => "bg",
        "bos" => "bs",
        "cat" => "ca",
        "ces" => "cs",
        "dan" => "da",
        "deu" => "de",
        "ell" => "el",
        "eng" => "en",
        "epo" => "eo",
        "est" => "et",
        "eus" => "eu",
        "fin" => "fi",
        "fra" => "fr",
        "glg" => "gl",
        "heb" => "he",
        "hin" => "hi",
        "hrv" => "hr",
        "hun" => "hu",
        "ind" => "id",
        "ile" => "is",
        "ita" => "it",
        "jpn" => "ja",
        "kor" => "ko",
        "lat" => "la",
        "lit" => "lt",
        "lav" => "lv",
        "mkd" => "mk",
        "mlt" => "mt",
        "nld" => "nl",
        "nob" => "nb",
        "nno" => "nn",
        "pol" => "pl",
        "por" => "pt",
        "rum" => "ro",
        "rus" => "ru",
        "slk" => "sk",
        "slv" => "sl",
        "spa" => "es",
        "swe" => "sv",
        "tel" => "te",
        "tur" => "tr",
        "ukr" => "uk",
        "vie" => "vi",
        _ => "unknown",
    }
    .to_string()
}
