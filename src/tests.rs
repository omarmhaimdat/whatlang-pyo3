#[cfg(test)]
mod tests {
    use crate::extensions::detect_multiple_languages;
    use serde_json::to_string_pretty;
    use whatlang::{detect, Lang, Script};
    #[test]
    fn test_detect() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.lang(), Lang::Eng);
    }

    #[test]
    fn test_script() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.script(), Script::Latin);
    }

    #[test]
    fn test_is_reliable() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.is_reliable(), false);
    }

    #[test]
    fn test_detect_multiple_languages() {
        let text = "Parlez-vous français? Ich spreche Französisch nur ein bisschen. A little bit is better than nothing.";
        let languages = detect_multiple_languages(text).unwrap();
        // println!("{:?}", languages);
        let langs = languages.iter().map(|l| l.to_hashmap()).collect::<Vec<_>>();
        // Save languages to a json file
        println!("{:?}", langs);
        let json = to_string_pretty(&langs).unwrap();
        std::fs::write("languages.json", json).unwrap();
        assert_eq!(languages.len(), 3);
        assert_eq!(languages[0].text, "Parlez-vous français?");
        assert_eq!(languages[0].languages[0], Lang::Fra);
        assert_eq!(
            languages[1].text,
            "Ich spreche Französisch nur ein bisschen."
        );
        assert_eq!(languages[1].languages[0], Lang::Deu);
        assert_eq!(languages[2].text, "A little bit is better than nothing.");
        assert_eq!(languages[2].languages[0], Lang::Eng);
    }
}
