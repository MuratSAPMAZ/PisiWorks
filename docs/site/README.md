# PisiAnimasyon

**Okullar için animasyon ve oyun yapma programı.** Pardus'ta çalışır, Türkçedir, ücretsizdir ve özgür yazılımdır.

- **Site ve kitap:** <https://muratsapmaz.github.io/PisiAnimasyon/>
- **Kaynak kodu:** <https://github.com/MuratSAPMAZ/PisiWorks>

Bu depo PisiAnimasyon'un **tanıtım sitesini ve kitabını** barındırır. Programın kaynak kodu burada değildir: PisiAnimasyon, çizim programı PisiWorks ile aynı kaynaktan derlenir ve kodun tamamı [PisiWorks deposundadır](https://github.com/MuratSAPMAZ/PisiWorks). Bu depodaki sayfalar o depodaki `docs/kitap/` ve `docs/site/` klasörlerinden `packaging/site.py` ile üretilir; düzeltmeler orada yapılır.

## Ne yapar?

- **Zaman çizelgesi:** kare, anahtar kare, hareket ve şekil ara doldurma, yavaşlama, kılavuz yolu, maske. Flash dersindeki adlarla.
- **Davranışlar:** bir şekle tek tıkla verilen on dokuz hazır hareket.
- **Kurallar:** "Elma sepete değince: puanı 1 artır." Kod yazmadan oyun.
- **Türkçe yazı dili:** aynı kurallar Türkçe cümlelerle; sayılar, işlemler, eğer, yinele, komutlar.
- **Tek dosyalık film:** yapılan film her tarayıcıda açılan tek bir HTML dosyasıdır.

## Kitap

Öğrenci ve öğretmen için, ders ders ilerleyen 31 bölümlük resimli bir kitap:

- [Canlı örnekli sürüm](https://muratsapmaz.github.io/PisiAnimasyon/kitap/index-canli.html): örnekler sayfanın içinde oynar.
- [Resimli sürüm](https://muratsapmaz.github.io/PisiAnimasyon/kitap/index.html): yazdırmaya uygun.
- [Öğretmen için dönemlik plan](https://muratsapmaz.github.io/PisiAnimasyon/kitap/31-ogretmen-icin-canli.html)

## Kurulum

[Paketi indirin](https://github.com/MuratSAPMAZ/PisiWorks/releases/download/pisiworks-0.1.0/pisiworks_0.1.0_amd64.deb) (0.1.0, amd64; Pardus 25'te denenmiştir) ve indirdiğiniz klasörde:

```bash
sudo apt install ./pisiworks_0.1.0_amd64.deb
```

Sürüm notları: <https://github.com/MuratSAPMAZ/PisiWorks/releases>

## Amaç

Okulların müfredatında Flash hâlâ anlatılıyor; ama Flash Pardus'ta çalışmıyor ve artık geliştirilmiyor. PisiAnimasyon o dersin Pardus'ta sürebilmesi için yazıldı. **Ticari bir amacı yoktur:** satılmaz, reklam göstermez, veri toplamaz.

## Lisans

PisiAnimasyon özgür yazılımdır: [GNU Genel Kamu Lisansı, sürüm 3 ya da sonrası](COPYING). Ek koşullar ve programla yapılan filmlere ilişkin izin [NOTICE](NOTICE) dosyasındadır: geliştirici bilgisi korunur, değiştirilmiş bir sürüm kendi adını taşır, **yaptığınız filmler sizindir**. Bu depodaki kitap ve sayfalar da aynı lisansın altındadır.

## Geliştiren

**Murat SAPMAZ** · [muratsapmaz.web.app](https://muratsapmaz.web.app/)

PisiWorks, Med Amine Belbachir'in [Emberframe](https://github.com/gregstein/emberframe)'i üzerine kuruludur.

Adobe, Flash, Fireworks ve Animate, Adobe Inc.'in ticari markalarıdır. PisiAnimasyon'un Adobe ile hiçbir bağı yoktur. Pardus ve TÜBİTAK ile kurumsal bir bağı yoktur; bağımsız bir projedir.

---

**In English.** PisiAnimasyon is a free animation and game-making program for schools, with a Turkish interface, for the Pardus operating system. It follows the way Adobe Flash is taught (timeline, keyframes, tweens, symbols) and adds ready-made behaviors and rules written as Turkish sentences. This repository holds its site and its book (in Turkish); the source code is in [PisiWorks](https://github.com/MuratSAPMAZ/PisiWorks). GPL-3.0-or-later.
