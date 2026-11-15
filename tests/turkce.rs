//! The written language in Turkish. The language of the interface is one for the whole
//! program, so what is tried in Turkish is tried here, in a run of its own.

use pisiworks::i18n::{set_language, Language};
use pisiworks::script::{read, strangers, write};

#[test]
fn every_game_reads_back_from_its_own_turkish_text() {
    set_language(Language::Turkish);
    let mut games = 0;
    for example in pisiworks::sahne_examples::EXAMPLES {
        let doc = (example.build)();
        if doc.rules.is_empty() {
            continue;
        }
        games += 1;
        let written = write(&doc.variables, &doc.rules);
        let found = read(&written).unwrap_or_else(|mistakes| panic!("{}: {mistakes:?}\n{written}", example.name));
        assert_eq!(found.rules, doc.rules, "{}\n{written}", example.name);
        assert_eq!(found.variables, doc.variables, "{}", example.name);
        let sounds: Vec<String> = doc.sounds.iter().map(|s| s.name.clone()).collect();
        assert!(strangers(&doc.rules, &doc.object_names(), &sounds).is_empty(), "{}", example.name);
    }
    assert_eq!(games, 6, "the showcase and the five small games");
}

#[test]
fn a_game_written_in_turkish_is_read() {
    set_language(Language::Turkish);
    let text = "\
Sayılar:
    Puan = 0
    Can = 3

Elma nesnesi Sepet nesnesine değince:
    Puan sayısını 1 artır
    Elma nesnesini üstte rastgele bir yere koy

Kahraman nesnesi Kapı nesnesine değince, Puan sayısı 2 değerinden büyükse ve Can sayısı 0 değerinden büyükse:
    zafer sesini çal
    1 saniye bekle
    2. sahneye git
değilse:
    Kahraman nesnesini Kahraman.x - 40, 262 konumuna koy

Boşluk tuşuna basılınca:
    Mermi nesnesinin kopyasını Gemi nesnesinin yerinde oluştur
    Puan sayısını Puan * 2 + rastgele(1, 6) yap
";
    let found = read(text).unwrap();
    assert_eq!((found.variables.len(), found.rules.len()), (2, 3));
    let door = &found.rules[1];
    assert_eq!(door.conditions().count(), 2);
    assert_eq!((door.steps.len(), door.otherwise.len(), door.otherwise[0].ex.as_str()), (3, 1, "Kahraman.x - 40"));
    assert_eq!(found.rules[2].steps[1].ex, "Puan * 2 + rastgele(1, 6)");
    // Written out again it is the same text.
    assert_eq!(write(&found.variables, &found.rules).trim(), text.trim());
    // What is wrong is told in Turkish, with its line.
    let told = read("Elma dans edince:\n    Elma nesnesini gizle\n").unwrap_err();
    assert_eq!((told[0].line, told[0].what.as_str()), (1, "bu, bildiğim bir olay değil"));
    let told = read("Film başlayınca:\n    Puan sayısını 1 artır\n").unwrap_err();
    assert_eq!((told[0].line, told[0].what.as_str()), (2, "Puan adında bir sayı yok"));
}
