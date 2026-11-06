# Dinamik simülasyon paneli: geliştirme yol haritası

Tarih: 6 Kasım 2026  
Durum: kısmen uygulanıyor; ayrıntılar aşağıdaki uygulama durumunda.
Kapsam: LiNC'nin simülasyon profillerindeki gömülü paneli. Gerçek makinede yazılımsal kontrol bu planla etkinleştirilmez.

## Uygulama durumu

İlk geliştirme diliminde panel alt kenara taşındı, yüksekliği sınırlandırıldı ve
panel gizlenince tutulan girişlerin bırakılması eklendi. Sabit eksen lambaları
ve jog tuşları aktif INI'deki koordinatlara göre filtreleniyor. Kontrol, jog,
işlev ve spindle sekmeleri, sayfa bazlı görünürlük, seçili sekme ve panel
yüksekliği tercihleri eklendi. Sekme değişimi tutulan girişleri bırakır. Mevcut
LED ve güç/koruma geri bildirimleriyle seçili buton görünümleri başlatıldı.
Lisanslı Tabler ikonlarının küçük bir alt kümesi, mevcut simülasyon panel
tuşları için TOML üzerinden etiket/ikon/gizleme seçimi ve panelde ICONS
sekmesinden ikon seçimi eklendi. Beş eksen simülasyonunda A/B homing LED'leri
joint durumuna bağlandı. Bu, aşağıdaki aşamaların tamamlandığı anlamına gelmez:
tüm AXIS fonksiyonları, kullanıcı SVG yükleme, kullanıcı tanımlı sanal HAL,
harici panel uyumluluğu henüz uygulanmadı.
Ayrı `[DISPLAY] ANGULAR_INCREMENTS` listesi ve açısal hız değerleri artık INI
modelinde okunur; geçersiz adımlar ayrıca raporlanır. Bu veriler jog
arayüzü ve simülasyon servisinde ayarlanabilir. Yerel AXIS, ayrı
`ANGULAR_INCREMENTS` anahtarını değil `INCREMENTS` içindeki birimleri kullanır;
bu anahtar beş eksen profilindeki diğer GUI uyumluluğu için ayrıca okunur.
Model artık açık ve tekil trivkins axis–joint eşlemesini, axis/joint türlerini,
hız sınırlarını ve açısal adım birimlerini denetleyen hareket başlatmayan bir
plan da hazırlar. Belirsiz kinematik ve eksik/geçersiz sınırlar reddedilir.
Plana sahip olmak tek başına HAL zincirinin etkin olduğu anlamına gelmez.
Simülasyon panel servisi doğrulanmış açısal hız ve adım seçimini tutar; değerler
ini sınırları dışında veya bir jog tuşu basılıyken değiştirilemez. Lineer ve
açısal jog birlikte istenirse simülasyon servisi iki grubu da bütün jog tuşları
bırakılıncaya dek kilitler. Yalnız sim-5axis profilinde A/B için ayrı HAL
zinciri, açısal hız/artım ve lineer-açısal geçiş beklemesi eklendi. İzole
LinuxCNC simülasyonunda FREE ve TELEOP yönleri, artımlı hareket, bırakma,
lineer/açısal hız geçişi, süreli tuşun istemci bağlantısı kesilince bırakılması
ve eşzamanlı lineer/açısal kilit sınandı. Hızlı yön değişiminde halui'nin yeni
hareketi eski yönün dur komutuyla kesebildiği tekrar üretildi; simülasyon
arbitratörü artık her eksende yön değişiminden önce çıkışları kısa süre bırakır.
Aynı eksenin iki zıt yönü aynı anda istenirse bütün jog tuşları bırakılana kadar
hareket engellenir. PROTECT açıkken yapılan izole tekrarda hızlı yön değişimi
geçti; zıt yön kilidinin panel durumu ve bütün tuşlar bırakılana kadar devamı
hem soket testinde hem HAL üzerinde doğrulandı. Bu yalnızca söz konusu
koşullardaki simülasyon doğrulamasıdır.

Önceki farklı bir TELEOP başarısızlığında PROTECT kapalıydı ve uygulamanın
arka plandaki G54 eşitlemesiyle aynı anda `halui.mode.is-manual` ve teleop
durumları düştü. Bunun tam mekanizması ve güvenli düzeltmesi ayrıca
araştırılmalıdır; ilişki zamanlama gözlemidir, kesin kök neden kanıtı değildir.
Kodda ofset eşitlemesi MDI kipine geçmeyi ister ve daha sonra önceki kipi geri
ister; kip isteğinin LinuxCNC tarafından gerçekten kabul edildiği ayrıca
doğrulanmaz. Sadece anlık hareket durumu kontrolü yeni bir jog isteğiyle kip
geçişi arasındaki yarışı kapatmaz. Yeni bir çözüm ofset doğruluğunu, jog
isteklerini ve kip geçişini birlikte ele almalı; fiziksel makine davranışı için
ayrı inceleme gerektirir.
Limitler, panel sürecinin kaybı ve görsel pencere de ayrıca sınanmalıdır.

## Hedef

Panel makinenin gerçek yapılandırmasını ve doğrulanmış kabiliyetlerini yansıtsın; LinuxCNC AXIS'in ilgili standart işlevleri destek durumuyla takip edilsin; kullanıcı kendi simülasyon paneli öğelerini tanımlayabilsin. Panel, uygulamanın altında sayfaya duyarlı, açılır/kapanır ve ekran boyutuna uyumlu olsun. Butonlar endüstriyel HMI görünümünde; işlev gerektirdiğinde iki durumlu ve gerçek geri bildirime dayalı olsun.

Kaynak incelemesi: [SIM PANEL ve AXIS karşılaştırması](../sim-panel-axis-karsilastirma.md). Mevcut gömülü panelin süreç ve güvenlik modeli: [embedded-simulator-panel.md](embedded-simulator-panel.md).

## Başlangıç durumu

- Panel halen `crates/linc-ui/src/shell/mod.rs` içinde sola yerleşir; içeriği `panel_preview.rs` içinde büyük ölçüde sabit listelerle çizilir.
- Aktif INI eksenleri `linc-core::ini` içinde okunur; panel tuşlarının görünümü bu listeyi henüz kullanmaz. HAL bağlantısının varlığı, gerçek eylemin tamamlandığını veya donanımın varlığını kanıtlamaz.
- `linc-panel` panel servisidir ve simülasyon durumuna sahiptir; UI bir istemcidir. Simülasyon kontrolü fiziksel panel kontrolüne genellenmemelidir.
- Mevcut tuşlar sabit bit maskesiyle sınırlıdır; TOML'de yeni isim göstermek yeni bir sanal HAL pini oluşturmaz.
- SVG raster işleme için `resvg` zaten mevcut. İkon paketi veya yeni SVG yükleme yeteneği bu planda henüz eklenmiş değildir.

## Temel tasarım ilkeleri

1. **Durum = gerçek geri bildirim:** Kullanıcı bir isteğe bastığında düğmeyi hemen tamamlanmış sayma; komut gönderiliyor, onaylandı, reddedildi, bağlantı koptu ve bilinmiyor durumlarını ayır. Komut ve geri bildirim farklı kaynaklar olabilir.
2. **İki durum her zaman toggle değildir:** bir işlev kalıcı aç/kapat olabilir; jog ve cycle-start gibi anlık girişler basma/bırakmadır; spindle CW/STOP/CCW çok durumlu bir seçimdir. İki görsel hal eklemek bunların kontrol semantiğini değiştirmez.
3. **Tek kaynak:** eksen ve joint verileri aktif INI ve LinuxCNC durumundan, canlı bağlantı bilgisi HAL'den, özel öğenin amacı kullanıcı panel tanımından gelir. Bunlar TOML'de ikinci kez makine gerçeği olarak tutulmaz.
4. **Bilinmeyen = bilinmeyen:** HAL veya geri bildirim yoksa “kapalı”, “bağlı” ya da “başarılı” varsayma. Kullanılamayan eylemin sebebini göster.
5. **Sınırlı panel, genişletilebilir model:** hazır kontroller ve tipli kullanıcı tanımları ayrı katmanlardır; kullanıcı dosyası ile yeni kontrol adı eklemek tek başına yeni sürücü yeteneği anlamına gelmez.
6. **Klavye ve dokunma:** 1280×720 minimum pencere; en az 44 px etkileşim hedefi; QMK klavyesinde Tab veya Shift olduğu varsayılmaz. Odak, görünür uyarılar ve erişilebilir etiketler korunur.
7. **Süreç ve kapsam:** simülasyon panelinin basılı kontrollerinde servis tarafındaki zaman aşımı korunur; fiziksel profil ve mevcut `linc.*` kapalı yazma yüzeyi etkilenmez. Görsel E-stop gerçek acil durdurmanın yerine geçmez.

## Aşama 0 — Envanter ve kabul ölçütleri

**İşler**
- Aktif LiNC panelindeki tuş, LED, grup, override, mod ve E-stop için `ad → semantik → pin/komut → durum geri bildirimi → profil → AXIS karşılığı → mevcut test` tablosu çıkar.
- Yerel AXIS kaynak ağacındaki eksen/joint jog, homing, spindle, flood/mist, program, override, limit ve özel panel seçeneklerini bu tabloya ekle. Her satırı `mevcut`, `kısmi`, `planlandı`, `destek dışı`, `araştırılacak` olarak işaretle.
- AXIS çekirdek işlevi, PyVCP/GladeVCP uzantısı ve makineye özel HAL özelliğini ayır. “AXIS'teki her şey” için kapsamı ve her fonksiyonun simülasyon panelinde mi, LiNC'nin başka ekranında mı sunulacağını kaydet.
- XYZ freze, XZ torna, XYZAB ve XYYZ gantry için fixture seç; aktif INI ve HAL dosyaları ile gerçekte bağlı yolları belgele.

**Bittiğinde:** Kod tabanına referanslı eşleştirme ve kapsam dışı özellikler açıkça görülebilir; hiçbir eksik özellik varmış gibi sunulmaz.

## Aşama 1 — Alt panel kabuğu ve sayfa davranışı

**İşler**
- Sol alanı mevcut durum çubuğunun üstünde, `egui` alt paneline taşı; sayfa kullanılabilirliğini koruyan minimum/maksimum yükseklik uygula. Panel kapalı, kompakt ve genişletilmiş olabilir; kullanıcı yüksekliği ayarlayabilir.
- İçeriği genişliğe göre satıra geçir; yükseklik sınırında panel içinde kaydır. Grup sekmeleri ve düzenlenmiş hedefler 44 px sınırını korusun.
- Varsayılanlar: Machine → çalıştırma, Manual → jog, Homing → referans, Probe → jog/prob; Offsets kompakt veya kapalı; Program, Tools, Files, Diagnostics, Help kapalı. Bu varsayımlar kullanıcı tarafından açılıp değiştirilebilir.
- Sayfa ve grup seçimi, kullanıcının manuel tercihi, panel yüksekliği ve görünürlüğünü profil bazlı tercihlerde sakla. Makine konfigürasyonunu tercihlerde çoğaltma.
- Sayfa/grup değişimi, panel kapanması, modal, pencere odak kaybı ve bağlantı kesilmesinde aktif tutuşu bırak; eski basış yeniden bağlanınca tekrarlanmasın.
- Fare, dokunma ve mevcut klavye akışında açma/kapatma ve grup değiştirme erişilebilir olsun; editör odağına çakışan kısayol seçme.

**Doğrulama:** 1280×720, 1920×1080 ve farklı ölçeklerde sayfa alanı, kaydırma, 44 px hedef, durum çubuğu, odak ve basılı tuş bırakma; bağımsız sim ve LinuxCNC sim smoke testi. Fiziksel profilde panel görünmez.

## Aşama 2 — HMI buton sistemi ve iki durum

**İşler**
- Temada ortak bir HMI buton bileşeni oluştur: açık/dark yüzey, belirgin çerçeve, okunabilir ikon + metin, duruma göre ışık/renk ve dokunmatik boyut. Siyah tek tip kare görünümü zorunlu olmasın.
- Görsel durumları tanımla: `idle`, `pressed`, `active`, `pending`, `disabled`, `unknown/error`. Renkle birlikte metin/ikon/LED kullan; renk körlüğü ve düşük parlaklıkta okunabilirlik değerlendir.
- Davranış türlerini ayrı tut:
  - **Momentary:** jog, rapid jog, cycle-start gibi basılı tutulan veya darbe üreten kontrol; basılı/bırakılmış görselleri. Görünüm kalıcı ON/OFF yazmaz.
  - **Toggle:** gerçekten kalıcı aç/kapat isteği ve doğrulanabilir geri bildirimi olan özellik; `ON` / `OFF`, ayrıca pending ve unknown.
  - **Çok durumlu:** spindle yönü ve mod seçimi gibi birbirini dışlayan seçimler; doğru durum geri bildiriminden seçilir.
  - **Salt okunan lamba:** kullanıcıya basılabilir buton gibi görünmez.
- İstek üzerine iyimser kalıcı durum yazma. Geciken/gelmeyen veya çelişen geri bildirimi göster; tekrar bağlanmada servis durumunu yeniden oku.
- CNC'ye özgü kritik eylemlerde ikonun ve etiketin anlamı açık olsun; simülasyon E-stop görseli sürekli “SIM” olarak etiketlensin.

**Doğrulama:** her davranış türünde basma/bırakma, gecikme, hata, bilinmeyen değer, kesinti ve dışarıdan değişen geri bildirim; gerçek LinuxCNC simülasyonunda hareketin bırakmayla durması. İki görünümün aynı eylemi yanlışlıkla toggle'a çevirmediği doğrulanır.

## Aşama 3 — İkon paketi, özel SVG ve ikon seçici

**İşler**
- Sabit sürümlü [Tabler Icons](https://github.com/tabler/tabler-icons) veya [Lucide](https://github.com/lucide-icons/lucide) paketini seç; lisans ve telif bildirimlerini kontrol et. Seçilmiş varlıkları uygulama ile birlikte taşı; çalışma zamanında ağ gerekmesin. HMI görünümünü ikon değil buton bileşeni belirler.
- Başlangıçta genel işlevleri kapsayan küratörlü bir ikon kataloğu, ad/etiket ve kategori ile sun; CNC'ye özgü simgeler için kendi SVG'lerimize izin ver. Paket sürümü ve ikon kimlikleri yapılandırmada kararlı olmalı.
- Panel düzenleyicide arama, önizleme, seçme, ikonsuz seçenek ve yerel SVG içe aktarma tasarla. Kullanıcı etiketini de düzenler; ikon tek başına işlev anlamı taşımaz.
- SVG boyut ve karmaşıklık sınırları, dış kaynak/referansların ele alınması, hatalı dosya ve yerel dosya erişimi doğrulaması tanımla. `resvg` ile hedef boyutta rasterleştirip çözünürlük/ölçek/renge göre önbellekle; her karede yeniden işleme.
- Mevcut elle çizilen özel ikonlar için bire bir eşleştirme gerekiyorsa koru; uygun hazır ikon varsa kullan. Lisanssız Google görsellerini taşıma.

**Doğrulama:** farklı DPI/ölçek ve iki durum, offline çalışma, eksik/geçersiz SVG, yedek ikon, lisans dosyası, uzun etiket ve simge seçiminin kalıcılığı.

## Aşama 4 — Makine yetenekleri ve dinamik görünürlük

**İşler**
- Etkin INI, LinuxCNC canlı axis mask / joint durumu, spindle sayısı ve HAL envanterini kaynakları belli, tazelik bilgili bir modelde birleştir. Çakışmayı tanıda göster; yazılı eylemi uyumsuzken kapat.
- Axis ve joint'i ayır. `XYYZ` gantry, XZ torna ve özel kinematikte index eşitliği varsayma; `XYZABCUVW` axis desteğini modelle ve gerçekte desteklenmeyen panel tuşunu açıkça işaretle.
- Makinede bulunmayan eksen/spindle öğeleri çizilmesin. Mevcut ama eylem yolu bağlanmamış öğe açıklamayla devre dışı gösterilebilsin. Geri bildirim bağlantısız ve bilinmiyor durumları ayrı olsun.
- HAL pininin sadece herhangi bir sinyale bağlı olmasını uçtan uca işlevsel kabul etme. Tip/yön/kaynak, writer/reader ve hedef eşleşmesi için doğrulama ve tanı geliştir; post-GUI geç bağlantıları ve tekrar bağlanmayı ele al.
- `[TRAJ] SPINDLES` varsayılanlarını hedef LinuxCNC sürümüne göre doğrula; mevcut `max(1)` davranışını ve sıfır spindle geçerliliğini uçtan uca araştır. Spindle fonksiyonlarını CW/CCW/fren/geri bildirim olarak ayrı değerlendir.
- 5 eksen sim profilindeki A/B zincirinin G54 eşitlemesiyle çakışan aralıklı TELEOP başarısızlığını araştır; limit ve panel süreci kaybı testleriyle doğrulamayı tamamla. Görsel destek ile hareket desteği ayrı doğrulanır.

**Doğrulama:** XYZ, XZ, XYZAB, XYZC, XYZUVW, XYYZ, özel kinematik ve çoklu spindle fixture'ları; eksik HAL, geç yükleme, INI/canlı uyuşmazlığı. Hareket testleri yalnız izole simülasyonda.

## Aşama 5 — Standart AXIS işlevlerinin aşamalı tamamlanması

Aşama 0'daki matristen hareket et; AXIS özelliği ile LiNC'nin panel mimarisini birbirine zorla eşitleme.

1. Jog: axis/joint seçimi, sürekli/artımlı, lineer/açısal hız ve artım, gerektiğinde analog/MPG; birim ve yön.
2. Referans ve limit: joint home/home-all uygunluğu, homed/fault, pozitif/negatif limit ve durumlar.
3. Program/mod: uygun çalışma modu, start/pause/resume/stop/step, optional stop ve block delete.
4. Spindle/coolant: her desteklenen spindle için yön, durdurma, fren, override ve gerçek geri bildirim; flood/mist.
5. Feed/rapid/spindle override ve maksimum hız: aralıklar, ölçek ve geri bildirim. RAPID jog tuşu ile rapid override ayrı kalır.
6. İhtiyaca göre MDI, tool/probe, takım değişimi ve diğer ilgili AXIS işlevleri. LiNC'nin mevcut ekranlarında zaten bulunan bir işlevi panelde çoğaltma kararı matriste gerekçelendir.

**Doğrulama:** her özellik için kaynak → komut → geri bildirim → uçtan uca sim testi; AXIS karşılaştırma matrisinde destek düzeyi güncel. Mevcut HAL sinyallerine ikinci writer bağlanmaz.

## Aşama 6 — Kullanıcı panel tanımı ve düzenleyici: mevcut kabiliyetler

**İşler**
- Sürümlü panel tanım formatı: grup/sekme, sıralama, widget kimliği, türü, metin, ikon, görünür sayfalar, durum kaynağı ve açık eylem eşlemesi. Örnek TOML şeması önce tasarımdır, mevcut uygulamanın desteklediği özellikmiş gibi sunulmaz.
- İlk sürümde yalnız panel servisinin mevcut bilinen eylemlerine ve izinli salt okunur veri kaynaklarına bağlan. Widget türü ile eylemin semantiği uyuşmuyorsa tanımı reddet.
- Düzenleyiciyle ekle, sırala, düzenle, gizle, sil, önizle ve kaydet. Tanım hatası satır/alan ve anlaşılır nedenle gösterilir; önceki geçerli tanım korunur.
- Kullanıcının sayfa bazında aynı grubu isteğe bağlı gösterebilmesi ve her sayfadaki varsayılan panel davranışını ayarlaması.

**Doğrulama:** tanımın yeniden yüklenmesi, geçersiz/tekrarlı kimlik, bilinmeyen ikon/eylem, eksik veri, yükseltme ve eski ayar uyumu; özel öğelerin iki durumunun yalnız onaylı durumdan çizilmesi.

## Aşama 7 — Kullanıcı tanımlı sanal I/O ve gelişmiş paneller

**İşler**
- Mevcut `u64` sabit panel protokolünden ayrı kullanıcı tanımlı sanal HAL bileşeni veya başka uygun süreç tasarla. Tip/yön/izin, pin adları, durum erişimi ve her widget için yaşam döngüsü açık olsun.
- Bir başlangıç tür kümesi belirle: bool LED, momentary/toggle bool çıkışı, sayısal gösterim ve sınırlı sayısal giriş. Gerçek hedef LinuxCNC sürümünün HAL türlerini doğrula; `PORT` gibi akış türlerini normal sayısal widget sanma.
- Kullanıcı HAL bağlantılarını açık tanımlar; pin sahibini, tek writer kuralını, POSTGUI zamanlamasını, tekrar bağlanmayı ve bağlantı kopmasında varsayılan değerleri doğrula. Güvenli basma süresi ve odak kaybında bırakma servis tarafında da korunur.
- Sadece simülasyon modunda etkinleştir. Fiziksel sürücüye genel yazma veya mevcut kapalı `linc.*` allowlist'ini sessizce genişletme.
- Ardından grafik, çok durumlu gösterge, analog slider, görüntü ve özel etiket gibi türleri ihtiyaç ve testlerle genişlet.

**Doğrulama:** panel servisinin kesilmesi/yeniden başlaması, pin oluşturma sırası, tür/yön uyuşmazlığı, ikinci writer, bağlantı yokluğu, held giriş zaman aşımı ve fiziksel profilde erişim yokluğu. HAL entegrasyonu canlı LinuxCNC simülasyonunda doğrulanır.

## Aşama 8 — PyVCP / GladeVCP / QtVCP uyumluluğu

- Kullanıcıların gerçek panel örneklerini incele; **çalıştırma/gömme**, **belirli widget alt kümesini içe aktarma** veya **yalnız LiNC formatı** seçeneklerini karşılaştır.
- Her seçenek için X11/Wayland, harici süreç ömrü, klavye odağı, HAL başlatma sırası, paketleme ve kullanıcı tanımlı Python handler'larının yetkilerini test et.
- Kısmi içe aktarmada desteklenmeyen widget/handler'ları raporla, sessizce atlama. LinuxCNC uzantıları çalıştırılabilir kod içerebilir; veri dosyası gibi ele alma.
- Uyumluluğu yalnız doğrulanmış dosya ve davranışlar için belirt; tüm LinuxCNC panel ekosistemiyle tam uyum sözü verme.

**Doğrulama:** seçilen uyum seviyesine ait örnek paneller ve dokümante edilmiş eksikler; kurulu olmayan bağımlılık ve harici süreç hataları.

## Çapraz kabul kriterleri

- Gösterilen her kontrolün kaynağı, anlamı, bağlılığı ve beklenen davranışı tanımlıdır. Tanımlı olmayan özel işlev kendiliğinden ortaya çıkmaz.
- Komut isteği ile fiili durum ayrıdır; iki durumlu kontroller gerçek geri bildirim, hata ve bilinmeyen durumları gösterir.
- `--sim --no-hal` ve LinuxCNC HAL sim ayrı test edilir; no-HAL simüle durumları gerçek bağlantı olarak sunulmaz.
- Sayfa, sekme, modal ve odak değişimi ile istemci durması hareket/tutma eylemlerini bırakır; yeniden bağlanmada eski basış tekrarlanmaz.
- 1280×720 ve daha büyük boyutlar, fare/dokunma/klavye, okunabilir etiket, 44 px hedef ve panelin kapladığı alan kontrol edilir.
- Fiziksel profilin panel kontrol yetkisi değişmez; mevcut `linc.*` allowlist ve HAL tek-yazar kuralı korunur.
- Anlamlı otomatik testlere ek olarak ilgili profiller `--check-config` ile doğrulanır; UI işi `cargo test --workspace` ve simülasyonda gerçek pencere incelemesiyle doğrulanır. HAL değişiklikleri statik incelenir ve uygun sim testleriyle sınanır.

## Önerilen teslim sırası

- **Teslim A:** Aşama 0–2 — kaynak envanteri, alt panel, sayfa ilişkileri ve HMI buton/state sistemi.
- **Teslim B:** Aşama 3–4 — lisanslı ikonlar, dinamik makine kabiliyeti ve eksenlere göre görünürlük.
- **Teslim C:** Aşama 5–6 — standart AXIS kapsamını matristen tamamlama ve mevcut yetenekler için panel düzenleyici.
- **Teslim D:** Aşama 7–8 — gerçek kullanıcı tanımlı sanal I/O ve kararlaştırılan harici panel uyumluluğu.

Aşamalar bağımlılıklarına göre yeniden sıralanabilir; A/B eksen HAL işi donanım eşlemesi bilinmeden görsel panelde tamamlanmış sayılmaz.
