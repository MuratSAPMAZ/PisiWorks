# PisiWorks kullanım kılavuzu

Şekilleri ve çizgileri kısıtlamak için çizerken Shift'i basılı tutun. Tuvali kaydırmak için Boşluk tuşunu basılı tutun. Ctrl+kaydırma yakınlaştırır. Kalem yolunu bitirmek için Enter'a basın, kapatmak için ilk noktasına tıklayın.

## Eğriler

Kalem (P) ile köşe noktası koymak için tıklayın. Yumuşak nokta koymak için basıp sürükleyin: çektiğiniz iki tutamaç, yolu noktanın iki yanında büker.

Alt Seçim (A) ile bir yolu seçtiğinizde noktaları görünür. Bir noktayı seçmek için tıklayın; tutamaçları görünür. Shift+tıklama seçime nokta ekler; boş tuvalde kutu çizmek içindeki bütün noktaları seçer. Seçili noktayı, seçtiğiniz diğer noktalarla birlikte taşımak için sürükleyin; eğriyi yeniden biçimlendirmek için tutamacı sürükleyin. Karşı tutamaç da onunla birlikte döner ve nokta yumuşak kalır; tutamaçlardan yalnızca birini oynatıp sivri bir köşe yapmak için Alt'ı basılı tutun. Bir köşeden tutamaç çekmek için Alt+sürükleyin, bir noktayı yeniden köşeye çevirmek için Alt+tıklayın. Delete, seçili noktaları siler.

Kalem de seçili bir yolu düzenler. Nokta eklemek için yolun bir parçasına tıklayın. Eğri noktasını köşeye çevirmek için ona, köşe noktasını silmek için köşeye tıklayın. Bir noktadan tutamaç çekmek için noktadan sürükleyin. Açık bir yolun serbest ucuna tıklayarak çizmeye oradan devam edin; başka bir açık yolun serbest ucunda bitirerek ikisini birleştirin.

Elipsler, noktaları düzenlendiğinde birkaç eğri noktasından oluşan yollara dönüşür. Dikdörtgen önce onay ister, çünkü yola dönüşünce yuvarlaklık ayarı kalmaz. Serbest Biçim, Bıçak ve yol işlemleri sonucu yine eğri yol olarak verir: başka bir daireyle birleştirilen daire yine eğrilerden oluşur.

Arayüz dili masaüstünüzün diline göre seçilir. Değiştirmek için Düzenle > Dil menüsünü kullanın.

## Görüntüleri düzenleme

Bir görüntüyü düzenlemek için Dosya > Aç (Ctrl+O), geçerli belgeye bir görüntü eklemek için Dosya > İçe Aktar (Ctrl+R) komutunu kullanın.

Fırça, Kurşun Kalem ve Silgi, işaretçinin altındaki bitmap'in piksellerini değiştirir. Bitmap dışında Fırça ve Kurşun Kalem vektör konturları çizer. Fırça kontur rengiyle boyar, Boya Kovası dolgu rengiyle doldurur. Fırça veya Silgi etkinken fırça boyutunu Özellikler panelinden ayarlayın.

Kırp aracıyla sürükleyin, ardından uygulamak için Enter'a basın veya çift tıklayın. Esc iptal eder.

Kaydet, açılan görüntünün üzerine hiçbir zaman yazmaz. Nesnelerinizi koruyan bir `.pisi` belgesi oluşturur; Dosya > Dışa Aktar ise düz bir görüntü yazar.

Seçim Çerçevesi (M), Kement (L) ve Sihirli Değnek (W) bir bitmap üzerinde piksel seçer. Seçime eklemek için Shift'i, seçimden çıkarmak için Alt'ı basılı tutun. Boyama, doldurma ve filtreler seçimin içinde kalır. Kes, Kopyala ve Sil seçili piksellere uygulanır; Yapıştır bunları yeni bir bitmap olarak ekler. Seçili pikselleri taşımak için İşaretçi ile seçimin içinden sürükleyin veya ok tuşlarına basın; geride bir kopya bırakmak için Alt'ı basılı tutun. Seçim Çerçevesi veya Kement ile seçimin içinden sürüklemek yalnızca seçim sınırını taşır; Ctrl basılıyken pikselleri taşır. Ctrl+D veya Esc seçimi kaldırır.

Lastik Damgası (S): kaynağı belirlemek için Alt+tıklayın, ardından oradan klonlamak için sürükleyin.

## Vektörleri yeniden biçimlendirme

Ölçek (Q) aracı, seçili her şeyi tutamaçlarından yeniden boyutlandırır. Shift'i basılı tutmadığınız sürece köşeler oranları korur.

Eğriltme ve Deformasyon araçları, Ölçek düğmesini paylaşır: listelemek için düğmeye basılı tutun veya üçü arasında sırayla geçmek için Q'ya yeniden basın. Üçünde de döndürmek için seçim kutusunun dışını sürükleyin; 15 derecelik adımlarla döndürmek için Shift'i basılı tutun. Döndürme, kutunun ortasındaki küçük dairenin çevresinde yapılır; bu daireyi önce başka bir yere sürükleyebilirsiniz.

- **Eğriltme**: seçimi yatırmak için bir kenar tutamacını, perspektif için bir köşe tutamacını sürükleyin; perspektifte köşe ile komşusu birbirinden uzaklaşır veya birbirine yaklaşır.
- **Deformasyon**: yalnızca bir köşeyi oynatmak için köşe tutamacını, bütün bir kenarı taşımak için kenar tutamacını sürükleyin.

Sürüklerken çizim dönüşümü yalnızca önizler; başka bir tutamaçla veya üç araçtan bir diğeriyle devam edebilirsiniz. Uygulamak için Enter'a basın veya kutunun içine çift tıklayın, her şeyi eski haline getirmek için Esc'ye basın. Kutunun içini sürüklemek seçimi taşır.

Değiştir > Dönüştür menüsünde aynı araçlar ile 180° Döndür, 90° döndürmeler ve iki çevirme bulunur. Nümerik Dönüştürme (Ctrl+Shift+T) yüzdeyle ölçekler, piksel cinsinden boyut verir veya bir açıyla döndürür. Merkez noktasını sürüklediyseniz o noktayı, sürüklemediyseniz seçimin ortasını sabit tutar.

Metin dönüştürüldüğünde metin olarak kalır; düzenlemeye ve boyutunu değiştirmeye devam edebilirsiniz. Sembol kitaplığa bağlı kalır; dikdörtgen yuvarlaklığını, otomatik şekil ayarlarını korur. Değiştir > Dönüştür > Dönüştürmeleri Kaldır bunları yeniden dik hale getirir. Yatay Çevir ve Dikey Çevir metni, sembolleri ve grupları da aynalar. Bitmap'ler yeniden çizilir. Web alanları dönüştürülmez. Metin > Yollara Dönüştür, metni harflerinin ana hatlarına çevirir.

Serbest Biçim (O), işaretçinin yakınındaki bir yolu veya şekli çeker. Bıçak (Y, Kurşun Kalem seçiliyken yeniden basın) seçili vektörleri sürüklenen çizgi boyunca keser. Hiçbir şey seçili değilken Bıçak, kestiği her vektörü böler. İkisi de dikdörtgenleri ve elipsleri yola dönüştürür.

## Web alanları

Sıcak Nokta (J) ve Dilim (K), çizimin üzerine alanlar çizer. Bunları İşaretçi ile taşıyıp yeniden boyutlandırın; sıcak noktanın bağlantısını Özellikler panelinden ayarlayın. Gizlemek veya göstermek için 2'ye basın.

Bu alanlar düzleştirilmiş görüntüde hiçbir zaman görünmez. `ad.png` dışa aktarılırken dilimler için `ad_slice1.png`, `ad_slice2.png` ve devamı, sıcak noktalar için de görüntü haritası içeren `ad_map.html` yazılır.

## Paneller

Pencere menüsü her paneli açıp kapatır; F4 hepsini gizler veya geri getirir. Bir sekmeyi öne getirmek için tıklayın, grubunu daraltmak için öndeki sekmeye çift tıklayın.

- **Katmanlar**: üstteki liste seçili nesnelerin karışım modunu, yani renklerinin alttaki çizimle nasıl karışacağını belirler; Opaklık ise alttakinin ne kadar görüneceğini. Aynı liste Özellikler panelinde de vardır. Karışım modu kullanılan sayfa, dışa aktarılacağı gibi piksel piksel gösterilir.
- **Durumlar**: her durumun kendi nesneleri vardır. Animasyonu oynatın, soğan zarını açın ve Katmanlar panelinden bir katmanı durumlar arasında paylaşın. Yeniden adlandırmak için bir sayfaya, duruma, katmana veya nesneye çift tıklayın.
- **En İyileştir**: GIF, Animasyonlu GIF, JPEG, PNG 8, PNG 24 veya PNG 32 seçin. Panel dosya boyutunu tahmin eder; dışa aktarma bu ayarları kullanır. JPEG için Düzgünleştirme (0-8), dosyayı küçültmek üzere görüntüyü biraz bulanıklaştırır. Seçmeli kalite, sayfanın bir bölümünü kendi kalitesinde tutar: pikselleri seçin, Değiştir > Seçmeli JPEG > Seçimi JPEG Maskesi Olarak Kaydet komutunu verin ve Seçmeli kalite kutusunu işaretleyin. Maske pembe bir renklendirmeyle görünür. Ayarlar penceresinden tüm metnin ve tüm düğmelerin seçmeli kalitede tutulması istenebilir, renklendirmenin rengi de değiştirilebilir. Bir sembol, Sembol Özellikleri'ndeki Tür alanı Düğme ise düğmedir; Ortak Kitaplık'taki düğmeler böyle gelir.
- **Stiller**: uygulamak veya sonraki şeklin renklerini ayarlamak için bir stile tıklayın. Stiller dolgu, kontur ve opaklık bilgisini tutar.
- **Yol**: Topla, Kesiştir, Zımbala, Kırp, Birleştir, Böl, Basitleştir, Konturu Genişlet, Yolu Daralt ve Yolu Genişlet.
- **Otomatik Şekiller**: bir kutucuğu tuvale sürükleyin veya şekli sayfanın ortasına yerleştirmek için tıklayın; ardından Otomatik Şekil Özellikleri panelinde ayarlayın.
- **Belge Kitaplığı**: Yeni, seçimden bir sembol oluşturur. Ekle sembolü yerleştirir, Güncelle onu seçimden yeniden tanımlar, Sil kaldırır.
- **Ortak Kitaplık**: bir öğeyi belgeye sembol olarak eklemek için tuvale sürükleyin veya tıklayın. Belge Kitaplığı'ndaki semboller de tuvale sürüklenebilir.
- **Bul**: bu sayfada veya tüm sayfalarda metin ya da renk değiştirin.
- **Renk Paleti**: bir renkle doldurmak için tıklayın, kontur için Shift+tıklayın. Geçerli dolgu için tamamlayıcı, benzeşik ve üçlü renkler de önerir.
- **Sembol Özellikleri**: seçili sembolü yeniden adlandırın, kaç kopyası olduğunu görün ve sembolü ayırın.

Her sekme satırının sağındaki panel menüsü o grubu daraltır veya kapatır. Üst çubuktaki Genişletilmiş Mod, açık paneller ile yalnızca sekmeler arasında geçiş yapar.

## Dışa aktarmaları karşılaştırma

Önizleme, sayfayı dışa aktarılacağı haliyle gösterir. 2 Yukarı görünümü orijinal sayfayı yanına koyar; 4 Yukarı görünümü orijinal sayfayı üç dışa aktarma ayarı ve dosya boyutlarıyla karşılaştırır. Bu görünümlerde kaydırmak için sürükleyin.
