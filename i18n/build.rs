use rspolib::{pofile, Merge, Save};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

type Translations = HashMap<String, HashMap<String, String>>;

fn generate_lazy_static_translations_hashmap(
    translations: Translations,
) -> String {
    let mut code = concat!(
        "::lazy_static::lazy_static! {\n",
        "    static ref TRANSLATIONS: HashMap<&'static str, HashMap<String, String>> = {\n",
        "        let mut translations = HashMap::new();\n"
    ).to_string();

    for (language_code, translations) in translations {
        let language_code_variable =
            language_code.replace('-', "_").to_lowercase();
        code.push_str(&format!(
            "        let mut {} = HashMap::new();\n",
            language_code_variable
        ));
        for (msgid, msgstr) in translations {
            code.push_str(&format!(
                "        {}.insert(\"{}\".to_string(), \"{}\".to_string());\n",
                language_code_variable,
                msgid,
                match msgstr.is_empty() {
                    true => msgid.clone(),
                    false => msgstr,
                }
            ));
        }
        code.push_str(&format!(
            "        translations.insert(\"{}\", {});\n",
            language_code, language_code_variable
        ));
    }
    code.push_str("        translations\n");
    code.push_str("    };\n");
    code.push('}');
    code
}

fn main() {
    let mut translations: Translations = HashMap::new();

    let pot_ref = pofile("messages.pot").unwrap();
    for locale_file in Path::new("locales").read_dir().unwrap() {
        // Merge PO files
        let path = locale_file.unwrap().path().to_string_lossy().to_string();
        let mut po = pofile(path.as_str()).unwrap();
        po.merge(pot_ref.clone());
        po.save(path.as_str());

        // Generate translations
        let language_code = path
            .split('.')
            .next()
            .unwrap()
            .to_string()
            .split('/')
            .last()
            .unwrap()
            .to_string();

        let mut translation = HashMap::new();

        for entry in po.entries {
            if entry.msgstr.is_none() {
                continue;
            }
            translation.insert(entry.msgid, entry.msgstr.unwrap());
        }
        translations.insert(language_code, translation);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("translations.rs");

    let lazy_static_translations_code =
        generate_lazy_static_translations_hashmap(translations);
    fs::write(dest_path, lazy_static_translations_code).unwrap();
}
