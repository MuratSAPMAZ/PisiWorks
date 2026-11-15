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

Seç menüsü piksel seçimini değiştirir. Benzerini Seç, bitmap'in her yerindeki benzer renkli pikselleri seçime katar; Tersini Seç (Ctrl+Shift+I) geri kalan her şeyi seçer. Seçim Çerçevesini Genişlet ve Daralt, seçimi verilen piksel kadar büyütür veya küçültür; Seçim Çerçevesine Kenarlık Ekle onu kenarı boyunca bir şeride çevirir; Seçim Çerçevesini Düzgünleştir pürüzlü kenarı düzeltir. Geçiş Yumuşatma, seçimin kenarını verilen yarıçap boyunca yumuşatır; silme, kopyalama, taşıma, boyama ve filtreler de kenarda yumuşayarak uygulanır. Piksel seçili değilken Tersini Seç, seçili olmayan nesneleri seçer.

Lastik Damgası (S): kaynağı belirlemek için Alt+tıklayın, ardından oradan klonlamak için sürükleyin.

Değiştir > Seçimi Düzleştir (Ctrl+Alt+Shift+Z) seçili nesneleri tek bir bitmap yapar. Aşağıda Birleştir (Ctrl+E) onları hemen altlarındaki bitmap ile birleştirir. Katmanları Düzleştir, görünen katmanlardaki her şeyi tek katmanda tek bir bitmap yapar; gizli katmanlar silinir, dilimler ve sıcak noktalar korunur. Değiştir > Tuval > Tuvali Kes, tuvali çizime kadar kırpar.

Düzenle > Ekle > Boş Bitmap, üzerine boyamak için tuval boyutunda saydam bir bitmap ekler. Düzenle > Klonla (Ctrl+Shift+D) seçimi yerinde kopyalar; Nitelikleri Yapıştır (Ctrl+Alt+Shift+V) seçime, kopyalanan nesnenin dolgusunu, konturunu, opaklığını, karışım modunu ve filtrelerini verir. Filtreler > Filtreyi Tekrarla (Ctrl+Alt+Shift+F) son filtreyi yeniden uygular. Dosya > Geri Dön, dosyanın son kaydedilen haline döner. Görünüm > Seçimi Sığdır (Ctrl+Alt+0) seçime yakınlaşır.

## Vektörleri yeniden biçimlendirme

Ölçek (Q) aracı, seçili her şeyi tutamaçlarından yeniden boyutlandırır. Shift'i basılı tutmadığınız sürece köşeler oranları korur.

Eğriltme ve Deformasyon araçları, Ölçek düğmesini paylaşır: listelemek için düğmeye basılı tutun veya üçü arasında sırayla geçmek için Q'ya yeniden basın. Üçünde de döndürmek için seçim kutusunun dışını sürükleyin; 15 derecelik adımlarla döndürmek için Shift'i basılı tutun. Döndürme, kutunun ortasındaki küçük dairenin çevresinde yapılır; bu daireyi önce başka bir yere sürükleyebilirsiniz.

- **Eğriltme**: seçimi yatırmak için bir kenar tutamacını, perspektif için bir köşe tutamacını sürükleyin; perspektifte köşe ile komşusu birbirinden uzaklaşır veya birbirine yaklaşır.
- **Deformasyon**: yalnızca bir köşeyi oynatmak için köşe tutamacını, bütün bir kenarı taşımak için kenar tutamacını sürükleyin.

Sürüklerken çizim dönüşümü yalnızca önizler; başka bir tutamaçla veya üç araçtan bir diğeriyle devam edebilirsiniz. Uygulamak için Enter'a basın veya kutunun içine çift tıklayın, her şeyi eski haline getirmek için Esc'ye basın. Kutunun içini sürüklemek seçimi taşır.

Değiştir > Dönüştür menüsünde aynı araçlar ile 180° Döndür, 90° döndürmeler ve iki çevirme bulunur. Nümerik Dönüştürme (Ctrl+Shift+T) yüzdeyle ölçekler, piksel cinsinden boyut verir veya bir açıyla döndürür. Merkez noktasını sürüklediyseniz o noktayı, sürüklemediyseniz seçimin ortasını sabit tutar.

Metin dönüştürüldüğünde metin olarak kalır; düzenlemeye ve boyutunu değiştirmeye devam edebilirsiniz. Sembol kitaplığa bağlı kalır; dikdörtgen yuvarlaklığını, otomatik şekil ayarlarını korur. Değiştir > Dönüştür > Dönüştürmeleri Kaldır bunları yeniden dik hale getirir. Yatay Çevir ve Dikey Çevir metni, sembolleri ve grupları da aynalar. Bitmap'ler yeniden çizilir. Web alanları dönüştürülmez. Metin > Yollara Dönüştür, metni harflerinin ana hatlarına çevirir.

Serbest Biçim (O), işaretçinin yakınındaki bir yolu veya şekli çeker. Bıçak (Y, Kurşun Kalem seçiliyken yeniden basın) seçili vektörleri sürüklenen çizgi boyunca keser. Hiçbir şey seçili değilken Bıçak, kestiği her vektörü böler. İkisi de dikdörtgenleri ve elipsleri yola dönüştürür.

## Canlı filtreler

Özellikler panelindeki Filtreler, seçili nesnelere efekt ekler: renk ayarları, eğim ve kabartmalar, bulanıklaştırmalar, parazit, gölge ve ışımalar, keskinleştirme. Eklemek ve ayarlamak için listeden birini seçin. Her filtre; onu kapatan bir kutu, ayarlarını yeniden açan adı ve onu kaldıran bir eksi ile listelenir. Filtreler listenin başından sonuna doğru, nesne her çizildiğinde uygulanır; bu yüzden nesnenin kendisi düzenlenebilir kalır. Bir filtrenin yerini değiştirmek için adını yukarı veya aşağı sürükleyin. Bir nesneden oluşturulan stil, nesnenin filtrelerini de saklar.

Düzeyler, her düzeyde kaç piksel olduğunu ve girişte siyah, orta gri ve beyaz, çıkışta siyah ve beyaz sayılacak düzeyleri gösterir. Eğriler, girdi düzeyinden çıktı düzeyine bir eğri gösterir: nokta eklemek için eğriye tıklayın, bükmek için bir noktayı sürükleyin, kaldırmak için noktayı kutunun dışına sürükleyin. İkisinde de Kanal listesi vardır; kırmızı, yeşil ve mavinin, üçünün ortak ayarlarından ayrı kendi ayarları olur. Otomatik düğmesi her rengi en koyu düzeyinden en açık düzeyine yayar. Düzeyler'de sayıları yazmak yerine histogramın ve renk şeridinin altındaki üçgenleri sürükleyebilirsiniz. Üç damlalık siyah, gri veya beyaz noktayı belirler: birini seçin, sonra nesnenin üzerinde bir renge tıklayın.

Filtre pencerelerinde Önizleme kutusu vardır. Kapalıyken nesne, kutu yeniden işaretlenene veya pencere Tamam ile kapatılana kadar pencere açılmadan önceki haliyle görünür.

Photoshop Canlı Efektleri; Alt Gölge, İç Gölge, Dış Işıma, İç Işıma, Eğim Ver ve Kabart, Saten, Renk Kaplama, Degrade Kaplama, Desen Kaplama ve Kontur efektlerini bir arada tutan tek bir filtredir. İstediklerinizi işaretleyin ve ayarlarını görmek için efektin adına tıklayın: gölge ve ışımalarda karışım modu, opaklık, renk, açı, mesafe, yayılma veya boğma, boyut ve parazit; eğimde stil, teknik, derinlik, yön, yumuşatma, yükseklik, açıkton ve gölge; degradede stil, açı ve ölçek; konturda konum ve dolgu türü. Desenler kısa bir yerleşik listeden seçilir.

Degrade, altında duraklarıyla gösterilir. Durak eklemek için degradenin altına tıklayın, taşımak için durağı sürükleyin, kaldırmak için degradeden uzağa çekin. Seçili durağın rengi, yüzde olarak konumu ve opaklığı görünür. Hazır Ayarlar, degradeyi hazır bir degradeyle değiştirir. Kaplamanın, degrade konturun ve degrade ışımanın ayrı degradeleri vardır; degrade ışıma nesnenin kenarından başlar ve Değişim renklerini dağıtır.

Alt gölge ve dış ışıma, nesnenin altında kalanlarla kendi karışım modlarıyla karıştırılır. Listede Photoshop Canlı Efektleri'nden sonra gelen bir filtre, nesneye olduğu gibi onlara da uygulanır.

## Maskeler

Maske, bir nesnenin bir bölümünü gizler. Komutlar Değiştir > Maske menüsündedir:

- **Maske olarak Grupla**: iki veya daha fazla nesne seçin; en üstteki, diğerlerinin maskesi olur. Maske ne kadar açık renkliyse o kadar çok şey görünür; temiz bir parça kesmek için beyaz bir şekil kullanın.
- **Maske Olarak Yapıştır**: bir nesneyi kopyalayın, başka birini seçip bu komutu verin; kopyalanan nesne onu anahattıyla maskeler. **İçine Yapıştır** (Ctrl+Shift+V) tersini yapar: kopyaladığınız şey seçili nesnenin içinde görünür, seçili nesne çevresinde görünmeye devam eder.
- **Tümünü Göster** ve **Tümünü Gizle**, üzerine boyamak için her şeyi gösteren veya hiçbir şeyi göstermeyen bir bitmap maskesi ekler. Bir bitmap üzerinde piksel seçiliyken **Seçimi Göster** ve **Seçimi Gizle** maskeyi seçimden oluşturur.
- **Maskeyi Devre Dışı Bırak** maskeyi kaldırmadan kapatır; **Maskeyi Sil**, önce nesneye uygulanıp uygulanmayacağını sorduktan sonra kaldırır.

Maskeli nesnenin Özellikler panelinde Maske alanı görünür: vektör maskesi için Yol anahattı veya Gri tonlama görünümü ile Dolgu ve konturu göster; bitmap maskesi için Gri tonlama görünümü veya Alfa kanalı. Oradaki Maskeyi Düzenle düğmesi ya da Katmanlar panelinde maskenin küçük resmine tıklamak maskenin kendisini seçer; maskeyi herhangi bir araçla taşıyabilir, yeniden biçimlendirebilir veya boyayabilirsiniz. Beyaz boya gösterir, siyah gizler. Bitirmek için başka bir şey seçin.

Katmanlar panelinde nesne ile maskesi arasındaki zincir, ikisinin bağlı olduğunu ve birlikte taşındığını gösterir. Maskeyi serbest bırakıp nesneyi altında taşımak için zincire tıklayın, yeniden bağlamak için bir daha tıklayın. Maske Olarak Yapıştır, vektörün bitmap maskesine mi dönüşeceğini yoksa vektör mü kalacağını ve var olan maskenin değiştirilip değiştirilmeyeceğini ya da ona eklenip eklenmeyeceğini sorar.

## Araç grupları

Köşesinde küçük üçgen olan düğme birkaç aracı temsil eder. Listelemek için düğmeyi basılı tutun veya aralarında gezinmek için tuşuna yeniden basın; düğme en son kullanılanı gösterir.

- İşaretçi (V): Arkasındakini Seç, her tıklamada farenin altındaki bir alttaki nesneyi seçer.
- Ölçek (Q): Eğriltme ve Deformasyon'dan sonra 9 dilimli ölçekleme gelir. Dört mavi çizgi seçimi dokuz parçaya böler; bir çizgiyi taşımak için sürükleyin, sonra seçimin bir tutamacını sürükleyin: köşeler boyutunu korur, yalnızca orta kısım uzar; böylece yuvarlak köşeler yuvarlak kalır.
- Kırpma (C): Dışa Aktarma Alanı, sayfanın yalnızca üzerinden sürüklediğiniz kısmını dışa aktarır. Enter'a basın veya çift tıklayın; o kısım için Görüntü Önizleme açılır.
- Seçim Çerçevesi (M): Oval Seçim Çerçevesi oval bir alan seçer. Kement (L): Çokgen Kement düz kenarlarla seçer; her köşeye tıklayın, kapatmak için çift tıklayın veya ilk köşeye tıklayın.
- Bulanıklaştırma (R): Keskinleştir, Soldurma (açar), Yakma (koyulaştırır) ve Leke (boyayı fırça boyunca sürükler) Bulanıklaştırma gibi fırçanın altında çalışır. Soldurma ve Yakma'nın Özellikler'de Pozlama ve ton Aralığı; Leke'nin Basınç ayarı vardır ve Leke, kontur rengiyle yüklü başlayabilir.
- Lastik Damgası (S): Renk Değiştir, Kimden rengini Tolerans içinde Kime rengine çevirir; Kimden rengi yoksa fırçanın başladığı yerdeki rengi değiştirir. Kırmızı Göz Kaldırma, üzerinden sürüklediğiniz alandaki gözlerin kırmızısını Tolerans ve Güç ayarına göre alır.
- Kalem (P): Vektör Yol serbest elle yol çizer. Yolu Yeniden Çiz bir yolun bir kısmını değiştirir: çizgiye yolun üzerinde başlayın ve yine yolun üzerinde bitirin.
- Serbest Biçim (O): Alanı Yeniden Şekillendir, dairesinin içindeki bütün noktaları fareyle birlikte iter. İki Yol Süpürgesi, üzerinden geçtiğiniz yerde konturu Özellikler'deki Hız kadar kalınlaştırır veya inceltir; bu işlemden geçen yol çok sayıda kısa düz parçaya dönüşür.
- Sıcak Nokta (J): Daire Sıcak Nokta ortasından dışa doğru, Çokgen Sıcak Nokta ise Çokgen Kement gibi köşe köşe çizilir. Dilim (K): Çokgen Dilim de aynı biçimde çizilir. Dışa aktarılan görüntü eşleminde daire ve çokgen olarak yazılırlar.
- Dikdörtgen (U): Elips ve ortasından dışa doğru çizilen Çokgen; kenar sayısı ve yıldız seçeneği Özellikler'dedir.
- Boya Kovası (G): Degrade, seçili nesneyi sürüklemeye başladığınız yerden bıraktığınız yere uzanan bir degradeyle doldurur.

## Degrade ve desen dolguları

Bir vektör nesnesi seçiliyken Özellikler'de dolgunun yanındaki liste, nesnenin neyle doldurulacağını belirler: tek renk için Düz, on iki degradeden biri (Doğrusal, Radyal, Elips, Dikdörtgen, Koni, Kontur, Saten, Yıldız Patlaması, Kıvrımlar, Çubuklar, Dalgacıklar, Dalgalar) veya Desen. Yeni degrade, nesnenin dolgu renginden beyaza uzanır.

İşaretçi ile degradenin tuvalde iki tutamacı görünür: başladığı yerde yuvarlak, bittiği yerde kare. Degradeyi taşımak, döndürmek, uzatmak veya kısaltmak için bunları sürükleyin; nesnenin dışında da durabilirler. Kontur, nesnenin dış hattını izler ve tutamacı yoktur.

Elips ve Dikdörtgen degradesinin, genişliğini belirleyen üçüncü, daha küçük bir tutamacı vardır. Web Renk Taklidi iki rengi piksel piksel sırayla doldurur. Metin de her şekil gibi degrade veya desenle doldurulabilir.

Dolgu listesinin altındaki Kenar, dolgunun kenarının nasıl çizileceğini belirler: Sert, Kenar Yumuşatma veya kaç piksel boyunca söneceği yazılan Geçiş Yumuşatma. Doku, desenlerden birini yüzdesi kadar güçlü biçimde dolgunun içine işler.

Düzenle, degradenin renklerini açar: renk durağı eklemek için degradenin altına tıklayın, taşımak için durağı sürükleyin, kaldırmak için uzağa çekin; seçili durağın rengini ve konumunu ayarlayın. Opaklık durakları degradenin üstünde aynı biçimde çalışır: her biri degradenin bulunduğu yerde ne kadar opak olacağını belirler. Hazır Ayarlar, degradeyi hazır bir degradeyle değiştirir. Desende Düzenle, deseni ve ölçeğini seçtirir. Stiller, Nitelikleri Yapıştır ve dosyalar dolguyu korur.

## Görüntü Önizleme

Dosya > Görüntü Önizleme (Ctrl+Shift+X), dışa aktarma ayarlarını verdikleri görüntünün yanında gösterir; biçimi, dosya boyutu ve ölçüleri altında yazar, ayarları değiştirdikçe görüntü de değişir. Dışa Aktar görüntüyü kaydeder; Tamam dışa aktarmadan ayarları korur, İptal eski haline döndürür.

## Yazdırma

Dosya > Sayfa Yapısı kâğıdı, yönünü, kenar boşluklarını ve sayfanın kâğıda sığacak şekilde ölçeklenip ölçeklenmeyeceğini belirler. Dosya > Yazdır (Ctrl+P) sayfayı bilgisayarda kurulu bir yazıcıya gönderir veya PDF dosyası olarak kaydeder. Sayfa, dışa aktarıldığı gibi, dilimler ve sıcak noktalar olmadan, inç başına 72 pikselle yazdırılır; kâğıttan büyük sayfa, Sayfaya sığdır açık değilse birkaç kâğıda yazdırılır. Yazdırma aralığı görünen sayfayı, belgenin bütün sayfalarını veya bir sayfa aralığını seçer. Yazdırma için sistemin yazdırma hizmeti (CUPS) gerekir.

## Gruplar

Değiştir > Grupla (Ctrl+G) seçili nesnelerin tek nesne gibi taşınıp ölçeklenmesini sağlar. Grubu çözmeden bir üyeyi değiştirmek için Alt Seçim aracıyla üzerine tıklayın veya seçili grubun bütün üyelerini seçmek için Seç > Alt Seç'i kullanın. Üye artık tek başına taşınabilir, renklendirilebilir, yeniden biçimlendirilebilir veya silinebilir. Üyelerinden hiçbiri seçili kalmadığında grup yeniden bütün olur; Seç > Üst Seç ise bir üyeden bütün gruba geçer.

## Metin

Bir metin nesnesi seçiliyken Özellikler'de fontu ve boyutu, kalın, italik ve altı çizili düğmeleri ile satırları sola, ortaya veya sağa hizalayan düğmeler görünür. Aynıları Metin menüsünde de vardır: Font bilgisayarda kurulu fontları listeler; Boyut'ta Daha Küçük (Ctrl+Shift+,), Daha Büyük (Ctrl+Shift+.) ve bir boyut listesi; Stil'de Düz, Kalın (Ctrl+B), İtalik (Ctrl+I) ve Altı Çizili (Ctrl+U); Hizala'da Sol, Yatay Ortalanmış, Sağ, Yaslanmış (son satır dışındaki her satır boşluklarıyla kenardan kenara yayılır) ve Uzatılmış (her satır kutuyu dolduracak kadar genişletilir) bulunur. Hiçbir şey seçili değilken ayarlanırsa bir sonraki yazacağınız metne uygulanır. Kendi kalın veya italik yüzü olmayan fontta kalın, harfler kalınlaştırılarak; italik, harfler yatırılarak yapılır.

Metin > Yola Ekle (Ctrl+Shift+Y), seçili metni seçili yol veya şekil boyunca dizer; yol artık metne aittir ve onunla birlikte taşınır. Kapalı bir şeklin çevresinde metin soldan başlar ve üstten dolaşır. Özellikler'de harflerin nasıl duracağı (Metin > Metin Yönlendirmesi: Yolun Etrafında Döndür, Dikey, Dikey Eğrilt, Yatay Eğrilt) ve metni yol boyunca ileriden başlatan Metin kaydırma görünür. Yönü Ters Çevir metni öbür yöne çevirir; Metni Yoldan Ayır metni ve yolu geri verir.

Metin > Yazım Denetimi (Shift+F7), seçili metinleri veya sayfadaki bütün metinleri gözden geçirir ve sözlükte olmayan her sözcükte durur. Değiştir, alandaki sözcüğü onun yerine koyar; Tümünü Değiştir bunu her yerde yapar. Yoksay yalnızca bu yeri, Tümünü Yoksay sözcüğün tamamını geçer; Kişisel Sözlüğe Ekle sözcüğü bundan sonra doğru sayar. Denetim arayüzün dilinde, sistemde kurulu yazım sözlükleriyle yapılır (`enchant-2` veya `hunspell` programı ve `hunspell-tr` gibi bir sözlük).

## Kılavuzlar

Sayfaya kılavuz koymak için üstteki veya yandaki cetvelden sürükleyin. İşaretçi ile bir kılavuzu sürükleyerek taşıyın; kaldırmak için cetvelin üzerine geri sürükleyin. Taşınan, çizilen veya yeniden boyutlandırılan nesneler kılavuzlara bitişir. Konumunu yazmak için kılavuza çift tıklayın. Kalem ve Alt Seçim aracıyla sürüklenen noktalar da kılavuzlara bitişir. Görünüm > Kılavuzlar onları gösterir veya gizler (Ctrl+;), kilitler, bitiştirmeyi kapatır ve hepsini temizler; oradaki Kılavuzları Düzenle renklerini de belirler.

Akıllı kılavuzlar bir nesneyi taşırken belirir: nesnenin bir kenarı veya ortası başka bir nesneyle ya da sayfayla hizalandığında bir çizgi görünür ve nesne oraya bitişir. Görünüm > Akıllı Kılavuzlar çizgileri veya bitiştirmeyi kapatır. Görünüm > Dilim Kılavuzları, dilimlerin kenarları boyunca uzanan kırmızı çizgileri gösterir veya gizler.

## Web alanları

Sıcak Nokta (J) ve Dilim (K), çizimin üzerine alanlar çizer. Düzenle > Ekle > Sıcak Nokta (Ctrl+Shift+U) ve Dikdörtgen Dilim, seçili nesnenin üzerine bir tane koyar; birden fazla nesne seçiliyse hepsi için bir tane mi yoksa her biri için ayrı mı oluşturulacağı sorulur. Bunları İşaretçi ile taşıyıp yeniden boyutlandırın; sıcak noktanın bağlantısını Özellikler panelinden ayarlayın. Gizlemek veya göstermek için 2'ye basın.

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
