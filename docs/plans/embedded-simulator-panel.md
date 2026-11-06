# LiNC içine gömülü simülasyon paneli

## Kapsam

Mevcut `linc-panel` simülatörünün denetimlerini LiNC içindeki açılıp kapanan sol alanda sunmak. İlk uygulama yalnızca simülasyon profilleriyle sınırlıdır; fiziksel panelin seri sürücüsü ve gerçek makinenin HAL bağlantıları değiştirilmez. Ekran üzerindeki simüle edilmiş E-stop fiziksel acil durdurma devresi yerine geçmez.

## Mevcut yapı

- `crates/linc-panel/src/sim.rs`: `SimPanelState`, sürücü, geniş panel yüzü, çizim ve işaretçi/touch basma-bırakma takibi.
- `crates/linc-panel/src/main.rs`: simülasyon panelinin durumunu ve `cnc_panel.*` HAL bileşenini ayrı süreçte sahiplenen döngü.
- `crates/linc-diag/src/panel.rs`: isteğe bağlı simülasyon kontrol yüzeyi; gerçek panel bu yüzeyi sunmaz.
- `crates/linc-core/src/sim_ipc.rs`: HAL bulunmayan bağımsız simülasyonda panel snapshot ve gösterge değerlerini taşır; düğme girişleri için bir protokol değildir.
- `crates/linc-ui/src/shell/mod.rs`: LiNC pencere yerleşimi; panel servisiyle doğrudan aynı belleği paylaşmaz.

Panelin mevcut 1680×1000 mantıksal tuvalini dar bir şeride ölçeklemek 44 piksel etkileşim hedefini korumaz. Sol alan için kontroller yeniden akışlı gruplar, kaydırma veya açılır bölümlerle düzenlenmeli; tüm mevcut işlevler ve freze/torna yüzleri ayrıca eşleştirilmelidir.

## Tasarım kararları

1. Panel servisi ayrı süreç ve `cnc_panel.*` sahibi olarak kalır. Gömülü arayüz yalnızca simülasyon servisine bağlanan istemci olur; arayüz çökmesi fiziksel panel servisini kapatmaz.
2. İşlemler arayüz thread'ini bloklamayan ayrı bir istemci üzerinden taşınır; bağlantı kesilirse denetimler devre dışı ve bağlantı durumu görünür olur. Panel durumu servisten okunur, arayüz ikinci bir durum kaynağı oluşturmaz.
3. Sürekli jog gibi basılı kontroller sınırlı süreli ve yenilenen basışlarla yürütülür; bırakma, odak kaybı, sayfa kapanması, iletişim kesintisi veya istemci durması sonrasında panel servisinde süre aşımıyla bırakılır. Bir defalık kontrollerin darbe süresi ve mod/override kontrollerinin mevcut anlamı korunur.
4. Mevcut `diag.sim_*` yüzeyi yalnızca açıkça etkinleştirilen simülatörde vardır. Otomatik etkinleştirme veya MCP yazma yetkisini genişletmek yerine, gömülü panel istemcisi için erişim ve sahiplik modeli ayrıca belirlenip doğrulanmalıdır. Süresiz `press` bu iş için kullanılmamalıdır.
5. Fiziksel ve gömülü kontrolün eşzamanlı kullanımı, sahiplik, kilitler, güç ve E-stop semantiği ayrı tasarlanıp test edilmeden gerçek makine modunda ekran kontrolü açılmaz.

## Doğrulama

- Tüm mevcut panel butonları, düğmeler, LED'ler ve göstergeler için kaynak-hedef eşleştirme tablosu; mevcut durumda bağlı olmayanlar açıkça işaretlenir.
- 1280×720 ve büyük pencere boyutlarında 44 piksel hedefler, klavye/fare/dokunma, erişilebilir kaydırma ve kullanılabilir ana ekran.
- Testlerde basma, bırakma, uzun basış, çoklu dokunma, odak kaybı, istemci ölümü, bağlantı kopması ve geri bağlanma; gerçek zaman sınırları panel süreci tarafında doğrulanır.
- Bağımsız `--sim --no-hal` ve LinuxCNC simülasyonunda ayrı ayrı uçtan uca doğrulama; fiziksel panel profillerinde simülasyon kontrollerinin bulunmadığı doğrulanır.
- Mevcut bağımsız panel penceresi devre dışı bırakılmadan önce gömülü görünümün aynı durum ve işlemleri ürettiği karşılaştırılır.

## Çalıştırma ve doğrulama durumu

`./scripts/start-sim.sh` gömülü paneli varsayılan olarak açar. Ayrı panel
penceresiyle karşılaştırmak için `./scripts/start-sim.sh --external-panel`
kullanılır. Panel servisi iki durumda da ayrı süreçte çalışır. Gerçek makine
profilinde gömülü simülasyon kumandaları etkinleşmez.

Arayüz testleri freze ve torna düğmelerinin basma ve bırakmasını, acil
durdurma, güç, mod, koruma, override ve panel genişliğinin değiştirilmesini
simülasyon servisine kadar sınar. Bağlantı kesilip döndüğünde eski basılı
komutun yeniden gönderilmediği soket testinde kontrol edilir.
LinuxCNC simülasyonunda 56 düğmenin HAL çıkışı, sekiz mod, üç override,
koruma ve prob doğrulandı. Güç açıkken X+ hareketinin bırakılınca durduğu ve
simüle acil durdurmanın gücü kapattığı ayrıca kontrol edildi. Bu testler,
gerçek pencere üzerinden her düğmenin LinuxCNC'deki son etkisinin ayrı ayrı
doğrulandığı anlamına gelmez; bağlı olmayan HAL pinleri işlev sağlamaz.

`embedded_ui_jog_moves_and_stops_real_linuxcnc_simulation` testi ayrıca egui
işaretçi olayından gerçek simülasyon soketine ve HAL konumuna kadar X, Y ve Z'nin
artı ve eksi yönlerindeki hareket ve durmayı; sekiz modu, üç override'ı,
çalışma lambası, soğutma, yağlama, S.B.K., B.D.T. ve OPT düğmelerini;
program koruması ile acil durdurmanın LinuxCNC'deki etkisini
doğrular. Test özel olarak çalıştırılmalıdır: ayrı bir terminalde
`LINC_DIAG_DIR=/tmp/linc-ui-hal-integration ./scripts/start-sim.sh --home`
ile simülasyon başlatılır; ardından aynı `LINC_DIAG_DIR` ile
`cargo test --release -p linc-ui --lib embedded_ui_jog_moves_and_stops_real_linuxcnc_simulation -- --ignored`
çalıştırılır. Test yalnızca simülasyon profilini kabul eder ve X, Y, Z eksenlerini hareket ettirir.
Sonrasında E-stop etkin ve güç kapalıyken freze simülasyonunda bağlı 18 matris
tuşunun ve PROBE CONTACT'ın arayüzden basma/bırakma HAL sinyallerini kontrol eder.
`embedded_cycle_keys_control_real_linuxcnc_program` testi ise bekleme içeren
bir simülasyon programında CYCLE START ile çalıştırma ve devam ettirmeyi,
CYCLE STOP ile duraklatmayı ve K4 ile durdurmayı arayüzden doğrular.
`embedded_spindle_keys_control_real_linuxcnc_simulation` testi, önceden hız
ayarlanmış freze simülasyonunda SPD.CW, SPD.SP ve SPD.CCW düğmelerinin
iş mili yönü ve durma durumuna etkisini arayüzden kontrol eder.

Torna için `embedded_lathe_wired_keys_reach_real_linuxcnc_hal` testi güç
açıkken X ve Z'nin iki yönde hareket ve durmasını, ekrandan acil durdurmayı;
güç kapalıyken bağlı 11 matris tuşunun basma/bırakma HAL sinyallerini doğrular;
ikinci iş mili tuşları dahildir. Bu bilgisayarda normal torna simülasyonunun
beklediği `probe_basic_dev` checkout'u yoktu. Doğrulama için yerel Smartlathe
simülasyonundaki HAL dosyalarının geçici kopyaları ve geçici bir INI kullanıldı;
bu, normal `start-lathe-sim.sh` kurulumunun eksik bağımlılığını gidermez.
