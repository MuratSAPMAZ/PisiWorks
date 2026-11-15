# PisiWorks ve PisiSahne: lisans ve telif bildirimi

Bu belge, PisiWorks'ü ve onunla aynı pakette gelen animasyon programı PisiSahne'yi Pardus depolarına ya da okullara dağıtmadan önce inceleyecek kişiler için yazıldı. Programın hangi lisansla yayımlandığını, kodunun nereden geldiğini, içine hangi kitaplıkların derlendiğini ve Adobe ile ilişkisini tek yerde toplar. Sonunda, bizim karar veremediğimiz ve inceleyenin görüşüne bıraktığımız noktalar listelenir.

Bu bir hukuki görüş değildir; bir hukukçu tarafından gözden geçirilmedi. Buradaki her bilgi depodaki dosyalardan doğrulanabilir; nasıl doğrulanacağı ilgili bölümde yazıyor.

Sürüm: PisiWorks ve PisiSahne 0.1.0, Kasım 2026. İki program aynı kaynak koddan derlenir, aynı lisansı taşır ve tek bir `.deb` paketiyle kurulur; aşağıda "PisiWorks" için söylenen her şey, ayrıca belirtilmedikçe PisiSahne için de geçerlidir. Depo: [github.com/MuratSAPMAZ/PisiWorks](https://github.com/MuratSAPMAZ/PisiWorks)

## 1. Özet

| Soru | Cevap |
|---|---|
| İki programın lisansı | MIT (Debian'daki adıyla Expat). Tam metin: [`LICENSE`](../LICENSE) |
| Telif sahipleri | Med Amine Belbachir (Emberframe, 2026) ve Murat SAPMAZ (PisiWorks için yazılanlar, 2026) |
| Kaynağı | MIT lisanslı [Emberframe](https://github.com/gregstein/emberframe) projesinin çatalı |
| İçine derlenen kitaplıklar | 517 kitaplık, hepsi özgür yazılım; listesi [`THIRD-PARTY.md`](THIRD-PARTY.md) |
| Copyleft (GPL türü) yükümlülük | Yok. Tek zayıf copyleft kitaplık MPL-2.0 lisanslı `option-ext`; değiştirilmeden kullanılıyor (bkz. 4. bölüm) |
| Adobe'ye ait kod, görsel, font ya da dosya | Depoda ve pakette yok |
| Adobe dosya biçimleri | Fireworks PNG'nin katmanları okunur. Flash'ın `.fla` ve `.swf` biçimleri ne okunur ne yazılır |
| Ticari kullanım, değiştirme, yeniden dağıtım | Serbest; tek koşul telif ve lisans bildirimlerinin korunması |
| Pardus, TÜBİTAK ya da Adobe ile kurumsal bağ | Yok; bağımsız bir projedir |

## 2. PisiWorks'ün lisansı

PisiWorks, MIT lisansı altında yayımlanır. MIT lisansı programın kullanılmasına, kopyalanmasına, değiştirilmesine, birleştirilmesine, yayımlanmasına, dağıtılmasına, alt lisanslanmasına ve satılmasına izin verir. Tek koşulu, telif bildiriminin ve lisans metninin programın bütün kopyalarında ya da önemli parçalarında yer almasıdır. Program "olduğu gibi", hiçbir garanti verilmeden sunulur.

MIT lisansı Özgür Yazılım Vakfı'nın özgür yazılım, Açık Kaynak Girişimi'nin açık kaynak, Debian'ın da Debian Özgür Yazılım Yönergeleri'ne uygun saydığı lisanslardandır.

`LICENSE` dosyasındaki telif satırları:

```
Copyright (c) 2026 Med Amine Belbachir (https://medamineweb.com)
Copyright (c) 2026 Murat SAPMAZ (https://github.com/MuratSAPMAZ/PisiWorks)
```

## 3. Kodun kökeni

PisiWorks, Med Amine Belbachir'in yazdığı ve MIT lisansıyla yayımladığı Emberframe 0.1'in çatalıdır. Çatallama GitHub üzerinde açıkça yapıldı; depo sayfasında "forked from gregstein/emberframe" yazar ve Emberframe'in bütün işleme geçmişi korunur.

- Emberframe'den gelen: çatal anında 12.286 satır Rust kodu. Bunun yaklaşık %88'i hâlâ yerindedir.
- PisiWorks ve PisiSahne için yazılan: yaklaşık 26.400 satır. Bugünkü 37.312 satırın yüzde yetmişinden fazlası.
- PisiSahne'nin tamamı bu proje için yazıldı; Emberframe'de bir animasyon programı ya da zaman çizelgesi yoktur. PisiSahne, PisiWorks'ün çizim kodunu bir kitaplık olarak kullanır.
- MIT lisansının gereği olan telif bildirimi `LICENSE` dosyasında, README'lerde, Yardım > Hakkında penceresinde ve `.deb` paketinin `copyright` dosyasında korunur.

Doğrulamak için: `git log` çatal noktasını (`64dcd71`) ve sonrasındaki işlemeleri gösterir; `git diff 64dcd71 HEAD --stat -- src` eklenen ve silinen satırları verir.

Emberframe'in yazarı, çatalın adının "Emberframe" olarak kalmasını rica etti. MIT lisansı adın değiştirilmesine engel değildir ve PisiWorks kendi adıyla yayımlanır; bu rica 8. bölümde açık nokta olarak yer alır.

Depoda Emberframe'den kalan ve onunla aynı lisansı taşıyan dosyalar: `docs/logo.svg`, `docs/ef-logo.png`, `docs/screenshots/` altındaki eski ekran görüntüleri ve `examples/solstice-landing.pisi` örnek belgesi.

### Depoya kopyalanmış tek üçüncü taraf kaynak

`vendor/xim-ctext/` klasörü, Riey'nin MIT lisanslı [xim-rs](https://github.com/Riey/xim-rs) projesindeki `xim-ctext` 0.5.0 kitaplığının kopyasıdır. Arayüz kitaplığı GPUI'nin bağlı olduğu eski sürüm, X giriş yöntemiyle "Ç" ya da "ş" yazıldığında programı çökertiyordu; yeni sürüm bu klasörden kullanılır. Klasördeki `Cargo.toml` kaynağı, yazarı ve nedeni belirtir.

## 4. İçine derlenen kitaplıklar

PisiWorks Rust ile yazılmıştır ve kitaplıkları Cargo ile alır. Linux programına 517 kitaplık derlenir; bunların çoğu arayüz kitaplığı GPUI'nin (Zed Industries, Apache-2.0) dolaylı bağımlılıklarıdır. Tam liste, sürümleri, lisansları ve kaynak adresleriyle [`THIRD-PARTY.md`](THIRD-PARTY.md) dosyasındadır ve `packaging/third-party.py` betiğiyle Cargo'nun kendi verisinden üretilir.

Doğrudan kullanılan başlıca kitaplıklar:

| Kitaplık | Ne için | Lisans |
|---|---|---|
| gpui | Arayüz | Apache-2.0 |
| tiny-skia | Çizim ve dışa aktarma | BSD-3-Clause |
| ab_glyph, fontdb | Yazı tipleri | Apache-2.0, MIT |
| image, png, gif | Görüntü dosyaları | MIT ya da Apache-2.0 |
| flate2 | Sıkıştırma | MIT ya da Apache-2.0 |
| i_overlay | Yol işlemleri | MIT ya da Apache-2.0 |
| serde, serde_json | Belge dosyaları | MIT ya da Apache-2.0 |

Lisanslara göre dağılım:

| Lisans türü | Kitaplık sayısı | Yükümlülük |
|---|---|---|
| MIT, Apache-2.0 ya da ikisinden biri | 440 civarı | Telif ve lisans bildirimini korumak |
| Unicode-3.0 | 18 | Bildirimi korumak |
| BSD-2-Clause, BSD-3-Clause, ISC, Zlib | 20 civarı | Bildirimi korumak |
| Unlicense, CC0-1.0, 0BSD | 10 civarı | Yok (kamu malına bırakılmış) |
| MPL-2.0 | 1 (`option-ext`) | Aşağıda |

Dikkat isteyen üç durum:

- **`option-ext` (MPL-2.0).** Ayar klasörünü bulan `dirs` kitaplığının bağımlılığıdır. MPL-2.0 dosya düzeyinde zayıf copyleft'tir: yalnızca o kitaplığın kendi dosyaları değiştirilirse değişikliklerin kaynağı yayımlanmalıdır. PisiWorks onu değiştirmeden kullanır; kaynağı crates.io'da herkese açıktır. Programın geri kalanına bir yükümlülük getirmez.
- **Seçmeli lisanslar.** `self_cell` (Apache-2.0 ya da GPL-2.0), `r-efi` (MIT, Apache-2.0 ya da LGPL-2.1) gibi birkaç kitaplık lisans seçimi sunar. PisiWorks bunları her zaman izin verici olan seçenekle (MIT ya da Apache-2.0) alır; GPL ya da LGPL koşulları devreye girmez.
- **Apache-2.0 kitaplıkları.** Apache-2.0, lisans metninin ve varsa `NOTICE` dosyasının dağıtımla birlikte verilmesini ister. `.deb` paketi her kitaplığın kendi lisans dosyalarını `/usr/share/doc/pisiworks/third-party-licenses.txt.gz` içinde taşır (bkz. 6. bölüm).

517 kitaplığın 34'ü kaynak paketinde ayrı bir lisans dosyası taşımaz; lisansları yalnızca paket bilgisinde (`Cargo.toml`) yazılıdır. Bunlar listede lisans adlarıyla yer alır.

### Sistemden kullanılanlar

Bunlar PisiWorks'ün içine derlenmez; Pardus'un kendi paketlerinden gelir.

- Çalışma zamanında bağlanan sistem kitaplıkları: `libc6`, `libgcc-s1`, `libxcb1`, `libxkbcommon0`, `libxkbcommon-x11-0`, `libvulkan1`.
- Ayrı program olarak çağrılanlar: yazdırma için CUPS'un `lp` ve `lpstat` komutları, yazım denetimi için `enchant-2` ya da `hunspell` (Türkçe sözlük `hunspell-tr` paketinden), dosya seçme pencereleri için masaüstünün kendi penceresi ya da `zenity`.
- Yazı tipleri: programa font gömülmez; sistemde kurulu fontlar kullanılır.

## 5. Adobe ile ilişki

PisiWorks, Adobe Fireworks'ün; PisiSahne de Adobe Flash'ın çalışma biçimini bilerek örnek alan bağımsız programlardır. İnceleyenin en çok soracağı konu bu olduğu için ne yapıldığını ve ne yapılmadığını ayrı ayrı yazıyoruz.

**Yapılmayanlar:**

- Adobe'nin kaynak kodu, derlenmiş kodu ya da herhangi bir dosyası depoda ve pakette yoktur. Fireworks tersine derlenmedi ve kodu incelenmedi.
- Adobe'nin simgeleri, logoları, desen, doku, degrade, stil ve şablon kitaplıkları, yardım metinleri ve fontları kullanılmadı. PisiWorks'teki desenler, degrade hazır ayarları, şablonlar ve örnek belge ya Emberframe'den gelir ya da PisiWorks için yapılmıştır.
- Programlar "Fireworks" ya da "Flash" adını kendi adları olarak kullanmaz. Ad yalnızca neyin benzeri olduğunu ve hangi dosyaları açabildiğini anlatmak için geçer; her geçtiği yerde Adobe'nin markası olduğu belirtilir.

**Yapılanlar:**

- **Benzer çalışma alanı.** Pencere düzeni, araç grupları, paneller ve menü yapısı Fireworks kullanıcısının tanıyacağı biçimdedir. Bu düzen Emberframe'den gelir.
- **Türkçe terimler.** Menü ve komut adları ("Seçimi Canlandır", "Sembole Dönüştür", "Geçiş Yumuşatma" gibi), öğretmenlerin ders notlarıyla uyuşsun diye Türkçe Fireworks'ün arayüzündeki terimlerle aynı tutuldu. Bunun için geliştirme sırasında Türkçe Fireworks kurulumundaki arayüz metinlerine bakıldı. Alınan şey kısa, işlevsel komut adlarıdır; yardım metni, açıklama ya da belge kopyalanmadı. Çeviri tablosu `src/i18n/tr.rs` dosyasında açıkça görülebilir.
- **Fireworks PNG dosyalarını okuma.** Fireworks'ün kendi PNG dosyalarındaki katmanlar okunabilir. Biçim Adobe tarafından yayımlanmadığı için okuyucu, Fireworks 8 ile yazılmış sekiz dosyanın yapısı incelenerek, yalnızca birlikte çalışabilirlik amacıyla yazıldı. O dosyalar depoya ya da pakete alınmadı; testlerde kullanılan örnek dosya program tarafından sıfırdan üretilir. Okuyucunun tamamı `src/fwpng.rs` dosyasındadır ve biçim hakkında öğrenilenler dosyanın başında yazılıdır. PisiWorks Fireworks PNG yazmaz.
- **PisiSahne ve Flash.** PisiSahne'nin zaman çizelgesi, anahtar kareleri, hareket ve şekil ara doldurması, kılavuz ve maske katmanları, film klibi ve düğme sembolleri Flash'taki karşılıklarının davranışını örnek alır; kısayolları (F5, F6, F7, Enter, Ctrl+Enter) aynıdır. Bunların tamamı bu proje için yazıldı. Geliştirme sırasında elimizde bir Flash kurulumu yoktu: Flash'ın hiçbir dosyasına, arayüz metnine ya da örnek dosyasına bakılmadı. Türkçe terimler ("Zaman Çizelgesi", "Anahtar Kare", "Hareket Ara Doldurması") bizim çevirimizdir. PisiSahne `.fla` ve `.swf` dosyalarını açmaz ve yazmaz; ActionScript çalıştırmaz. Animasyonları kendi `.sahne` biçiminde saklar ve yalnızca HTML5 sayfası olarak yayımlar.
- **Yayımlanan HTML5 sayfaları.** PisiSahne'nin ürettiği sayfanın içindeki oynatıcı betik bu proje için yazıldı ve sayfayla birlikte gider. Kullanıcının yayımladığı animasyon kullanıcının kendi eseridir; betik için ayrı bir bildirim ya da lisans koşulu istenmez.
- **Photoshop Canlı Efektleri.** Fireworks'teki bu özelliğin karşılığı vardır; efektlerin adları ve ayarları aynıdır, hesaplamaları PisiWorks için yazılmıştır.

Adobe, Fireworks, Flash, Photoshop ve ActionScript, Adobe Inc.'in ticari markalarıdır.

## 6. Dağıtımda verilen bildirimler

`packaging/build-deb.sh` ile üretilen `.deb` paketi şunları `/usr/share/doc/pisiworks/` altına koyar:

| Dosya | İçeriği |
|---|---|
| `copyright` | Debian biçiminde telif ve lisans: iki telif sahibi, MIT (Expat) metni, Emberframe'e atıf |
| `THIRD-PARTY.md` | 517 kitaplığın adı, sürümü, lisansı ve kaynak adresi |
| `third-party-licenses.txt.gz` | Her kitaplığın kendi `LICENSE`, `COPYING` ve `NOTICE` dosyaları, yazarlarının yayımladığı haliyle |
| `lisans-ve-telif.md` | Bu belge |

Paket iki programı da kurar: `/usr/bin/pisiworks` ve `/usr/bin/pisisahne`, her birinin kendi menü girdisi, simgesi ve dosya türü (`.pisi`, `.sahne`) ile.

Program içinde Yardım > Hakkında penceresi geliştireni, depo adresini, Emberframe'e teşekkürü ve lisansı gösterir.

## 7. Nasıl doğrulanır

```bash
# Kitaplık listesini Cargo'nun verisinden yeniden üretir
python3 packaging/third-party.py

# Depoda Adobe'ye ait dosya olmadığını gösterir: izlenen bütün görsel ve font dosyaları
git ls-files | grep -iE '\.(png|jpg|svg|gif|ttf|otf|dll)$'

# Çataldan bu yana yazılanları gösterir
git diff 64dcd71 HEAD --stat -- src
```

Kaynak koddan derleme ve testler README'de anlatılır; 318 otomatik testin 317'si her derlemede çalışır, biri yalnızca elde gerçek bir Fireworks dosyası varsa çalıştırılır.

## 8. İnceleyenin görüşüne bırakılan noktalar

Bunlar bizim tek başımıza karar veremediğimiz ya da bir hukukçunun bakmasını doğru bulduğumuz konulardır. Gizlemek yerine buraya yazıyoruz.

1. **Görünüm benzerliği.** PisiWorks'ün çalışma alanı Fireworks'e, PisiSahne'ninki Flash'ın klasik çalışma alanına (zaman çizelgesi üstte, araçlar solda, Özellikler altta) bilerek benzer. PisiSahne, PisiWorks'ün renklerini ve simgelerini kullanır; Flash'ın görsellerinden hiçbiri alınmadı. Kod ve görseller Adobe'ye ait değildir, ancak bir programın genel görünümünün ne ölçüde korunduğu ülkeden ülkeye değişen bir hukuk konusudur. Fireworks 2013'ten beri geliştirilmiyor ve satılmıyor; bu durumu değiştirmez ama riski değerlendirirken bilinmelidir.
2. **Sol üstteki "Pw" ve "Sh" işaretleri.** Uygulama çubuğunun başındaki iki harfli işaretler Emberframe'in "Ef" işaretinden uyarlandı ve Adobe ürünlerinin iki harfli işaretlerini andırır. PisiSahne'nin işareti, Adobe Photoshop'un kullandığı "Ps" ile karışmasın diye "Sh" seçildi. Uygulama simgesi bu yüzden daha önce değiştirildi (kedi başı); pencere içindeki işaret duruyor. Değiştirilmesi istenirse küçük bir iştir.
3. **Türkçe terimlerin kaynağı.** Komut adları Türkçe Fireworks'tekilerle aynıdır. Tek tek komut adlarının telifle korunmadığını düşünüyoruz, ancak bu bir hukukçunun teyit etmesi gereken bir değerlendirmedir.
4. **Fireworks PNG biçiminin incelenmesi.** Okuyucu, yasal olarak edinilmiş bir Fireworks kurulumunun örnek dosyaları incelenerek, birlikte çalışabilirlik için yazıldı. 5846 sayılı Kanun'un bilgisayar programlarına ilişkin hükümlerinin bu tür incelemeye ne ölçüde izin verdiği teyit edilmelidir. Gerekirse okuyucu tek dosya olduğu için kolayca çıkarılabilir.
5. **"Pisi" ve "PisiSahne" adları.** PisiSahne adı bu projede konuldu; başka bir ürünle çakışıp çakışmadığı araştırılmadı.  Ad, Pardus geleneğine bir göndermedir. "PiSi" Pardus'un eski paket yöneticisinin adıdır ve "Pisi Linux" adında ayrı bir dağıtım vardır. Karışıklık ya da bir marka hakkı doğurup doğurmadığına Pardus ekibi karar vermelidir.
6. **Emberframe yazarının ad ricası.** Yazar, çatalın "Emberframe" adını korumasını rica etti. Lisans buna zorlamaz; PisiWorks her yerde kaynağını ve yazarını belirtir. Kendisine henüz cevap verilmedi.
7. **Telif satırındaki kişi.** PisiWorks için yazılanların telifi şimdilik Murat SAPMAZ adınadır. Proje bir kurum çatısı altına alınacaksa telif sahipliği ve katkı kuralları buna göre düzenlenmelidir.
8. **Kitaplık sayısı.** 517 kitaplığın lisansları Cargo'nun bildirdiği bilgilere dayanır; her birinin kaynak kodu tek tek gözle denetlenmedi. Pardus depoları için gerekiyorsa bağımlılıkların ayrıca denetlenmesi ya da Debian'da paketlenmiş sürümlerine geçilmesi ayrı bir çalışmadır.
9. **Garanti ve sorumluluk.** MIT lisansı programı garantisiz sunar. Okullarda kullanım için ayrı bir destek ya da sorumluluk taahhüdü yoktur.

## 9. İletişim

Sorular, düzeltmeler ve lisansla ilgili talepler için: [github.com/MuratSAPMAZ/PisiWorks/issues](https://github.com/MuratSAPMAZ/PisiWorks/issues)
