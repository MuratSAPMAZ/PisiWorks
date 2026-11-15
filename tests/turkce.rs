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
        let written = write(&doc.variables, &doc.commands, &doc.rules);
        let found = read(&written).unwrap_or_else(|mistakes| panic!("{}: {mistakes:?}\n{written}", example.name));
        assert_eq!(found.rules, doc.rules, "{}\n{written}", example.name);
        assert_eq!(found.variables, doc.variables, "{}", example.name);
        let sounds: Vec<String> = doc.sounds.iter().map(|s| s.name.clone()).collect();
        assert!(strangers(&doc.rules, &doc.commands, &doc.object_names(), &sounds).is_empty(), "{}", example.name);
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
    assert_eq!(write(&found.variables, &found.commands, &found.rules).trim(), text.trim());
    // What is wrong is told in Turkish, with its line.
    let told = read("Elma dans edince:\n    Elma nesnesini gizle\n").unwrap_err();
    assert_eq!((told[0].line, told[0].what.as_str()), (1, "bu, bildiğim bir olay değil"));
    let told = read("Film başlayınca:\n    Puan sayısını 1 artır\n").unwrap_err();
    assert_eq!((told[0].line, told[0].what.as_str()), (2, "Puan adında bir sayı yok"));
}

#[test]
fn loops_and_commands_read_in_turkish() {
    set_language(Language::Turkish);
    let text = "\
Sayılar:
    Puan = 0
    Atış = 3

Patlat komutu:
    Patlama nesnesinin kopyasını Düşman nesnesinin yerinde oluştur
    bum sesini çal

Boşluk tuşuna basılınca:
    Atış kez yinele:
        Mermi nesnesinin kopyasını Gemi nesnesinin yerinde oluştur
        0.2 saniye bekle
    eğer Puan sayısı 10 değerinden büyükse:
        Patlat komutunu yap
        Puan sayısı 0 değerinden büyükse yinele:
            Puan sayısını 3 azalt
    değilse:
        vız sesini çal
";
    let found = read(text).unwrap();
    assert_eq!((found.commands[0].name.as_str(), found.rules[0].steps.len()), ("Patlat", 2));
    let choice = &found.rules[0].steps[1];
    assert_eq!((choice.body.len(), choice.otherwise.len(), choice.body[1].body.len()), (2, 1, 1));
    assert_eq!(write(&found.variables, &found.commands, &found.rules).trim(), text.trim());
    let told = read("Film başlayınca:\n    Zıpla komutunu yap\n").unwrap_err();
    assert_eq!((told[0].line, told[0].what.as_str()), (2, "Zıpla adında bir komut yok"));
}

#[test]
fn what_is_typed_first_decides_what_is_offered() {
    use pisiworks::script_edit::{Names, ScriptEdit};
    set_language(Language::Turkish);
    let names = Names { objects: vec!["Elma".into(), "Sepet".into()], numbers: vec!["Puan".into()], commands: vec!["Patlat".into()], sounds: vec!["bum".into()] };
    let offered = |line: &str| {
        let mut edit = ScriptEdit::new("Film başlayınca:");
        edit.end();
        edit.newline();
        edit.insert(line);
        edit.offers(&names).iter().map(|offer| offer.shown().to_string()).collect::<Vec<_>>()
    };
    // For an object, the steps about objects, in its own capitals however it was typed.
    let apple = offered("elma");
    assert!(apple.iter().filter(|s| s.contains(' ')).all(|sentence| sentence.starts_with("Elma nesnesi")), "{apple:?}");
    // For a number, those about numbers and those that open with an amount; none about objects.
    let score = offered("puan");
    assert!(score.contains(&"Puan sayısını ? artır".to_string()) && score.contains(&"Puan kez yinele:".to_string()) && !score.iter().any(|s| s.contains("nesnesini")), "{score:?}");
    // For a command, the one sentence that does it; for a figure, what opens with one; for a sound, playing it.
    assert_eq!(offered("patlat")[0], "Patlat komutunu yap");
    assert!(offered("3").contains(&"3 kez yinele:".to_string()) && offered("3").contains(&"3 saniye bekle".to_string()));
    assert_eq!(offered("bum"), vec!["bum sesini çal"]);
    // Further into a sentence, what still fits it; and the beginning of a name brings the name.
    assert!(offered("Elma nesnesini g").contains(&"Elma nesnesini gizle".to_string()));
    assert_eq!(offered("Puan sayısını Pu"), vec!["Puan"]);
    assert!(!offered("Puan sayısını Puan").contains(&"Puan".to_string()), "a name that is whole is not offered again");
    assert_eq!(offered("Elma nesnesi Se")[0], "Sepet");
    // At the edge, the events: a key's name brings the event of that key.
    let mut edit = ScriptEdit::new("");
    edit.insert("boşluk");
    assert_eq!(edit.offers(&names)[0].shown(), "Boşluk tuşuna basılınca:");
}
