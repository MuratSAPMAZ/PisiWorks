# PisiWorks kullanım kılavuzu

Şekilleri ve çizgileri kısıtlamak için çizerken Shift'i basılı tutun. Tuvali kaydırmak için Boşluk tuşunu basılı tutun. Ctrl+kaydırma yakınlaştırır. Kalem yolunu bitirmek için Enter'a basın, kapatmak için ilk noktasına tıklayın.

## Eğriler

Kalem (P) ile köşe noktası koymak için tıklayın. Yumuşak nokta koymak için basıp sürükleyin: çektiğiniz iki tutamaç, yolu noktanın iki yanında büker.

Alt Seçim (A) ile bir yolu seçtiğinizde noktaları ve tutamaçları görünür. Noktayı taşımak için noktayı, eğriyi yeniden biçimlendirmek için tutamacı sürükleyin. Karşı tutamaç da onunla birlikte döner ve nokta yumuşak kalır; tutamaçlardan yalnızca birini oynatıp sivri bir köşe yapmak için Alt'ı basılı tutun. Bir köşeden tutamaç çekmek için Alt+sürükleyin, bir noktayı yeniden köşeye çevirmek için Alt+tıklayın.

Serbest Biçim, Bıçak ve Yol paneli düz parçalar üzerinde çalışır; bu yüzden eğriyi çok sayıda kısa parçaya dönüştürürler.

Arayüz dili masaüstünüzün diline göre seçilir. Değiştirmek için Düzen > Dil menüsünü kullanın.

## Görüntüleri düzenleme

Bir görüntüyü düzenlemek için Dosya > Aç (Ctrl+O), geçerli belgeye bir görüntü eklemek için Dosya > İçe Aktar (Ctrl+R) komutunu kullanın.

Fırça, Kurşun Kalem ve Silgi, işaretçinin altındaki bitmap'in piksellerini değiştirir. Bitmap dışında Fırça ve Kurşun Kalem vektör konturları çizer. Fırça kontur rengiyle boyar, Boya Kovası dolgu rengiyle doldurur. Fırça veya Silgi etkinken fırça boyutunu Özellikler panelinden ayarlayın.

Kırp aracıyla sürükleyin, ardından uygulamak için Enter'a basın veya çift tıklayın. Esc iptal eder.

Kaydet, açılan görüntünün üzerine hiçbir zaman yazmaz. Nesnelerinizi koruyan bir `.ember` belgesi oluşturur; Dosya > Dışa Aktar ise düz bir görüntü yazar.

Seçim Çerçevesi (M), Kement (L) ve Sihirli Değnek (W) bir bitmap üzerinde piksel seçer. Seçime eklemek için Shift'i, seçimden çıkarmak için Alt'ı basılı tutun. Boyama, doldurma ve filtreler seçimin içinde kalır. Kes, Kopyala ve Sil seçili piksellere uygulanır; Yapıştır bunları yeni bir bitmap olarak ekler. Ctrl+D veya Esc seçimi kaldırır.

Klonlama Damgası (S): kaynağı belirlemek için Alt+tıklayın, ardından oradan klonlamak için sürükleyin.

## Vektörleri yeniden biçimlendirme

Ölçekle (Q), seçili her şeyi tutamaçlarından yeniden boyutlandırır. Shift'i basılı tutmadığınız sürece köşeler oranları korur.

Aynı araç döndürür, eğer ve çarpıtır. Döndürmek için seçim kutusunun dışını sürükleyin; 15 derecelik adımlarla döndürmek için Shift'i basılı tutun. Eğmek için bir kenar tutamacını, tek bir köşeyi oynatmak için bir köşe tutamacını Ctrl ile sürükleyin. Değiştir > Dönüştür > Serbest Dönüştürme (Ctrl+T) bu aracı seçer.

Dikdörtgenler ve elipsler bu şekilde dönüştürüldüğünde yola çevrilir, bitmap'ler yeniden çizilir. Metin harflerinin yollarına dönüşür; bu yüzden önce metni düzenlemeyi bitirin. Metin > Yollara Dönüştür aynı işi tek başına yapar. Semboller ve web alanları olduğu gibi kalır: sembol için önce Değiştir > Sembol > Ayır komutunu kullanın.

Serbest Biçim (O), işaretçinin yakınındaki bir yolu veya şekli çeker. Bıçak (Y, Kurşun Kalem seçiliyken yeniden basın) seçili vektörleri sürüklenen çizgi boyunca keser. Hiçbir şey seçili değilken Bıçak, kestiği her vektörü böler. İkisi de dikdörtgenleri ve elipsleri yola dönüştürür.

## Web alanları

Etkin Nokta (J) ve Dilim (K), çizimin üzerine alanlar çizer. Bunları İşaretçi ile taşıyıp yeniden boyutlandırın; etkin noktanın bağlantısını Özellikler panelinden ayarlayın. Gizlemek veya göstermek için 2'ye basın.

Bu alanlar düzleştirilmiş görüntüde hiçbir zaman görünmez. `ad.png` dışa aktarılırken dilimler için `ad_slice1.png`, `ad_slice2.png` ve devamı, etkin noktalar için de görüntü haritası içeren `ad_map.html` yazılır.

## Paneller

Pencere menüsü her paneli açıp kapatır; F4 hepsini gizler veya geri getirir. Bir sekmeyi öne getirmek için tıklayın, grubunu daraltmak için öndeki sekmeye çift tıklayın.

- **Durumlar**: her durumun kendi nesneleri vardır. Animasyonu oynatın, soğan zarını açın ve Katmanlar panelinden bir katmanı durumlar arasında paylaşın. Yeniden adlandırmak için bir sayfaya, duruma, katmana veya nesneye çift tıklayın.
- **En İyileştir**: GIF, Animasyonlu GIF, JPEG, PNG 8, PNG 24 veya PNG 32 seçin. Panel dosya boyutunu tahmin eder; dışa aktarma bu ayarları kullanır.
- **Stiller**: uygulamak veya sonraki şeklin renklerini ayarlamak için bir stile tıklayın. Stiller dolgu, kontur ve opaklık bilgisini tutar.
- **Yol**: Birleşim, Kesişim, Del, Kırp, Birleştir, Böl, Basitleştir, Konturu Genişlet, İçe Kaydır ve Dışa Kaydır.
- **Otomatik Şekiller**: eklemek için bir kutucuğa tıklayın, ardından Otomatik Şekil Özellikleri panelinde ayarlayın.
- **Belge Kitaplığı**: Yeni, seçimden bir sembol oluşturur. Ekle sembolü yerleştirir, Güncelle onu seçimden yeniden tanımlar, Sil kaldırır.
- **Ortak Kitaplık**: bir öğeyi belgeye sembol olarak eklemek için tıklayın.
- **Bul**: bu sayfada veya tüm sayfalarda metin ya da renk değiştirin.
- **Renk Paleti**: bir renkle doldurmak için tıklayın, kontur için Shift+tıklayın. Geçerli dolgu için tamamlayıcı, benzeşik ve üçlü renkler de önerir.
- **Sembol Özellikleri**: seçili sembolü yeniden adlandırın, kaç kopyası olduğunu görün ve sembolü ayırın.

Her sekme satırının sağındaki panel menüsü o grubu daraltır veya kapatır. Üst çubuktaki Genişletilmiş Kip, açık paneller ile yalnızca sekmeler arasında geçiş yapar.

## Dışa aktarmaları karşılaştırma

Önizleme, sayfayı dışa aktarılacağı haliyle gösterir. 2'li görünüm özgün sayfayı yanına koyar; 4'lü görünüm özgün sayfayı üç dışa aktarma ayarı ve dosya boyutlarıyla karşılaştırır. Bu görünümlerde kaydırmak için sürükleyin.
