# SIM PANEL: LinuxCNC AXIS karşılaştırması ve geliştirme taslağı

Tarih: 6 Kasım 2026. İncelenen LinuxCNC: `/home/mert/Masaüstü/gmtcnc-main/cncgit/linuxcnc`, HEAD `07aa2ba191198003000d2acc52e629f8eb377026` (çalışma ağacı değişiklik içeriyor). LiNC HEAD `ad03f22` (çalışma ağacı değişiklik içeriyor; gömülü panel dahil bazı dosyalar henüz izlenmiyor). Bulgular mevcut dosyaların statik incelemesidir, canlı HAL testi yapılmamıştır. Yerel ağaç yukarı akışla birebir aynı kabul edilmemelidir.

## 1. Sonuç

AXIS hazır kontrollerin bir bölümünü makine konfigürasyonuna ve **canlı HAL bağlantılarına** göre gizler. İsteğe bağlı, makineye özgü kontrolleri ise INI/HAL'den anlam çıkararak otomatik yaratmaz: operatörün hazırladığı PyVCP/GladeVCP panelini yükler. LiNC için de iki katman gerekir:

1. Standart makine kabiliyetleri: INI + canlı LinuxCNC durumu + gerektiğinde HAL bağlantılarından türetilen eksen, joint, spindle ve standart fonksiyonlar.
2. Özel panel tanımı: kullanıcı tarafından seçilen yerleşim, açıklama, kontrol türü ve açıkça eşlenmiş sinyal. Tanımlanmayan özel nesne gösterilmez.

Sadece INI'yi okumak tüm ihtiyacı çözmez: INI özel bir HAL sinyalinin `kapı lambası`, `talaş konveyörü` veya `spindle soğutma pompası` olduğunu standart biçimde belirtmek zorunda değildir. HAL grafiği ad, tür, yön ve bağlantıyı gösterir; niyeti ve kontrol yetkisini genellikle göstermez. Bu nedenle otomatik keşif öneri yapabilir; kullanıcı tanımı anlamsal kaynak olmalıdır.

## 2. AXIS kodunda ne oluyor?

| Konu | Yerel kaynak | Davranış |
|---|---|---|
| Eksenler | LinuxCNC `src/emc/usr_intf/axis/scripts/axis.py:3477`, `:3985` | `[TRAJ] COORDINATES` okunur; LinuxCNC durumunun `axis_mask` bitlerinde bulunmayan eksen seçim widget'ları gizlenir. |
| Joint | `axis.py:3460`, `:3992` | Joint sayısı ve görünür joint kontrolleri eksen kontrollerinden ayrı ele alınır. |
| Jog sırası ve yönü | `axis.py:3945`, `:3951` | `JOG_AXES` / `JOG_INVERT` veya makine türüne göre varsayılan sıralama; tekrarlı eksenler sıralamada tekilleştirilir. |
| Açısal hız | `axis.py:3495`, `:3997` | Axis/joint tipleri değerlendirilir; gerekmediyse açısal jog hızı gizlenir. |
| Jog uygulaması | `axis.py:3350`, `:3366`, `:3446`; `share/axis/tcl/axis.tcl:929` | Seçili axis veya joint için GUI jog komutu gönderilir; manuel durum, çalışma durumu, mod, basma/bırakma ve odak kaybı dikkate alınır. |
| Limit anahtarı | `axis.py:4410` | Joint limit girişlerinde `hal.pin_has_writer` aranır; hiçbiri yoksa limit override gizlenir. Bu, özel bir limit LED'inin otomatik oluştuğu anlamına gelmez. |
| Spindle | `axis.py:4380` | 0 numaralı spindle'ın ilgili HAL pinleri için `hal.pin_has_writer` kullanılarak yön, hız, fren ve spindle alanları ayrı ayrı gizlenir. Bu bölümün açıkça `spindle.0` kullandığı görülür; çoklu spindle için otomatik eşdeğer alan yarattığı varsayılmamalıdır. |
| Soğutma | `axis.py:4425` | `iocontrol.0.coolant-mist/flood` için aynı writer kontrolüyle Mist/Flood gizlenir. |
| Otomatik gizleme istisnası | `axis.py:4380` | `AXIS_NO_AUTOCONFIGURE` varsa `forget` fonksiyonunun otomatik gizlemesi uygulanmaz. Limit override için ayrı döngü vardır. |
| Özel panel | `axis.py:4127`, `:4144`, `:4240`; LinuxCNC `docs/src/gui/pyvcp.adoc` | INI'deki `PYVCP`, `GLADEVCP`, `EMBED_TAB_*` üzerinden harici tasarım yüklenir. PyVCP XML widget'ları HAL pinleri oluşturur ve HAL dosyası bunları bağlar. |
| Kullanıcı kodu | `axis.py:4429` | `USER_COMMAND_FILE` veya `~/.axisrc` Python kodu çalıştırabilir; bu basit bir veri formatından daha geniş yetkilidir. |

`pin_has_writer`, sıradan “pin var” veya “pin bir sinyale bağlı” kontrolüyle aynı değildir. Spindle/coolant örneğinde kontrol edilen pinler çıkış tarafında da olabilir; bu nedenle bu sezgisel kontrol, makine donanımının fiziksel olarak var ve çalışır olduğunu kanıtlamaz. HAL tip, yön ve sinyal ilişkisi ayrıca değerlendirilmelidir. AXIS'in bu hazır kuralları, tüm özel işlevleri otomatik keşfeden bir sistem değildir.

## 3. LiNC bugün

- `configs/sim.ini:89,148`: XYZ / 3 joint; `configs/sim-5axis.ini:95,152`: XYZAB / 5 joint.
- `crates/linc-core/src/ini.rs:210`: koordinatlar INI'den çıkarılıyor; ama `crates/linc-ui/src/shell/panel_preview.rs:145` XYZABC LED'lerini sabit çiziyor. Bunlar jog tuşu değil LED.
- Aynı dosya `:720` freze jog tuşlarını XYZAB, `:768` torna tuşlarını ayrı sabit listeyle çiziyor. U/V/W için karşılık gelen genel jog düğmesi bu sabit panel modelinde bulunmuyor.
- `:218` tuşları panel servisinin `disconnected_buttons` listesiyle gri yapıyor. `crates/linc-panel/src/sim.rs:1802` yalnızca panel pininin bir sinyale bağlı olup olmadığına bakıyor; uçtan uca işlev, sürücü/tüketici ve çalışma şartları doğrulanmıyor. `--no-hal` simülasyonunda bilerek her tuş bağlantılı kabul ediliyor. Durum bilgisi edinilemiyorsa bu durum “bağlı” diye yorumlanmamalı.
- `hal/linc.hal:229` ve devamı sadece X/Y/Z panel jog zincirini, hem axis hem joint ve sürekli/artımlı yollarıyla bağlıyor. Beş eksen INI aynı dosyayı kullanıyor; incelenen `hal/` ve profil dosyalarında panel A/B jog uçlarına eşdeğer bağlantı bulunmadı. INI'ye A/B eklemek bu yolları kendiliğinden oluşturmaz.
- `hal/linc.hal:328` X/Y/Z homed LED'lerini bağlıyor; statik A/B/C LED'lerinin varlığı karşılık gelen eksenin veya homing bağlantısının kanıtı değil.
- `panel_preview.rs:325` feed, rapid ve spindle override aralıklarını sabit çiziyor. Spindle sayısına, gerçek HAL yoluna ya da profil limitlerine göre otomatik üretilmiyor.
- `crates/linc-core/src/ini.rs:358` `SPINDLES` eksikse 1 kullanıyor ve açıkça 0 yazıldığında da `.max(1)` ile 1'e çeviriyor. Bu nedenle “INI anahtarı yoksa spindle yok” veya bu parser'ın 0 değerini koruduğu varsayılamaz. LinuxCNC motion bileşeni sıfır kabul etse de yerel task katmanında spindle sayısı için başka sınır kontrolleri var; sıfır spindle'ın tam uygulama desteği hedef LinuxCNC sürümünde uçtan uca doğrulanmalı.
- `crates/linc-core/src/panel_buttons.rs:1` tuşları `u64` bit alanıyla ve sabit `BUTTON_PINS` listesiyle tanımlıyor; `crates/linc-panel/src/sim.rs:476` bilinmeyen isimleri reddediyor. Yerleşimi TOML'den okumak tek başına keyfi sayıda yeni HAL pinli widget eklemez.
- `crates/linc-hal/src/browser.rs:6` salt okunur HAL envanteri için `halcmd` çıktısını zaten topluyor. Bu mekanizma keşif için adaydır; kullanıcı işlemleri için genel HAL yazma yetkisi sağlamaz.

## 4. RAPID: iki farklı kavram

| Kontrol | LiNC | Yerel AXIS |
|---|---|---|
| RAPID basılı tuşu | Fiziksel panel tuşudur. `crates/linc-ui/src/shell/jog.rs:61` basılıyken profilin `rapid_speed` tavanını seçer; diğer panel modlarında da tuş anlamı olabilir. | Aynı isimli ayrı fiziksel panel tuşu yoktur. `axis.py:3446` Shift ile jog için azami jog hızını seçer. Birebir aynı iş akışı ve her sınırın aynı olduğu iddia edilmemeli. |
| RAPID % | Programın rapid override değerini ayarlayan ayrı kontroldür. HAL yolu `hal/linc.hal:303`. | Rapid Override sürgüsü vardır: `share/axis/tcl/axis.tcl:1558`, `axis.py:2205`, `c.rapidrate(value)`. |

Yani AXIS'te RAPID butonunu görmemen normal; ilgili bazı davranışlar klavye ve sürgü ile ifade ediliyor. RAPID jog hızlandırma ve rapid override tek bir özellik sayılmamalı.

## 5. Bilginin kaynağı ve gösterim politikası

| Nesne | Aday kaynak | Çıkarılabilecek şey | Çıkarılamayan / ayrı doğrulama |
|---|---|---|---|
| Eksen | `[TRAJ] COORDINATES`, canlı axis mask | Geometrik eksen harfleri; canlıysa ikisinin uyuşması | Eksenin bir fiziksel panel tuşuna bağlı olup olmadığı |
| Joint | `[KINS] JOINTS`, kinematik, canlı joint durumu | Joint sayısı ve durumu | Her zaman `axis[i] == joint[i]` değildir. `XYYZ`, XZ torna ve özel kinematik ayrı ele alınır. |
| Spindle | `[TRAJ] SPINDLES`, LinuxCNC varsayılanı, canlı durum ve HAL | Yapılandırılmış mantıksal sayı | CW/CCW/fren/devir geri bildirimi donanım desteği; bunlar tek tek bağlantı/tanım ister. |
| Limit / home | joint pinleri, canlı sinyal bağlantısı, açık kullanıcı eşlemesi | Bilinen limit girişlerinin bağlantısı ve değeri | Fiziksel anahtarın sağlamlığı veya hangi özel lamba ile gösterileceği |
| Flood / mist | bilinen iocontrol / halui pinleri ve HAL grafiği | Standart mantıksal arayüz ve bağlantılar | Pompanın fiziksel varlığı veya başka amaçla kullanılıp kullanılmadığı |
| Özel ışık, kapı, talaş, alarm | HAL envanteri + kullanıcı panel tanımı | Ad, tür, yön, sinyal bağlantısı; tanımlanmış widget | Sinyalin anlamı, etiket, pozisyon, izin, güvenlik ve istenen eylem HAL adından kesin çıkarılamaz. |
| Mod / çalışma / override | LinuxCNC durumu + kullanıcı seçimi + bağlantı | Değer, beklenen standart kontrol | UI özelliğinin her fiziksel panelde bulunduğu |

Gösterim için önerilen ayrı durumlar: **tanımlı ve bağlı**, **tanımlı ama bağlantısız**, **veri/bağlantı bilinmiyor veya eski**, **yapılandırılmamış**. İlki normal; ikincisi görünür fakat devre dışı ve gerekçeli; üçüncüsü bilinmiyor olarak gösterilir ve kontrol kapalı kalır; dördüncüsü gizlenir. Salt okumada sinyal koparsa son değeri geçerli gibi sunma.

`[TRAJ] SPINDLES` anahtarının yokluğu, LinuxCNC varsayılanları nedeniyle “yok” demek değildir. Özel olarak spindle'sız bir yapılandırmanın geçerliliği de sürüme ve tüm task/motion konfigürasyonuna göre test edilmelidir.

## 6. Önerilen mimari

### 6.1 Envanter ve kaynak önceliği

- **MachineCapabilities**: aktif INI yolu, eşsiz axis harfleri, joint sayısı, kinematik, mantıksal spindle sayısı, birimler ve ilgili sınırlar. Ayarlar için varsayılanlar LinuxCNC sürümüyle tutarlı uygulanır.
- **RuntimeCapabilities**: LinuxCNC'nin canlı axis mask, joint/spindle ve mod durumu; HAL pin/sinyal envanteri ve okunma zamanı. Başlangıçta HAL henüz bağlanmamış olabileceğinden `unknown` ile `disconnected` ayrı tutulur.
- **PanelDefinition**: hangi gruplar/widget'lar var, sırası, etiket, tür, kaynak/eylem, görünürlük ve uygunluğu. Makine geometrisi TOML'de yinelenmez.
- **BindingStatus**: tür, yön, ad varlığı, bağlı sinyal, writer/reader bilgisi ve eskilik ayrı alanlardır; tek bir `wired` bool'una sıkıştırılmaz. Sinyalin varlığı, işlevin doğru çalıştığının garantisi değildir.
- INI ile canlı durum çelişirse birini sessizce seçmek yerine panelde anlaşılır tanı ver; hareket kontrollerini uyumsuz durumda kapat.

Bunlar önerilen yeni kavramlardır; şu anda LiNC'de bu şekilde uygulanmış değiller.

### 6.2 Örnek panel tanımı — öneri, mevcut TOML şeması değildir

```toml
[sim_panel]
version = 1
preset = "machine-aware"

[[sim_panel.widgets]]
id = "x_limit"
kind = "indicator"
label = "X limit"
source = { kind = "hal_pin", name = "joint.0.neg-lim-sw-in", type = "bit" }

[[sim_panel.widgets]]
id = "coolant"
kind = "momentary"
label = "Soğutma"
action = { kind = "panel_button", name = "coolant" }
feedback = { kind = "panel_led", name = "led_coolant" }

[[sim_panel.widgets]]
id = "rapid_override"
kind = "override"
label = "Rapid %"
action = { kind = "sim_override", name = "rapid" }
```

Bu sadece şekli anlatır. Gerçek şema, ilgili driver'ın hangi eylemleri gerçekten sağlayabildiğine göre doğrulanmalıdır. Kullanıcı bir widget'ı silerse panelden kalkar; mevcut bir fiziksel tuşu yeniden adlandırmak yeni donanım özelliği üretmez. Anlamsal eksen/joint eşlemesi kullanıcı konfigürasyonu ve kinematikle doğrulanmalıdır; yukarıdaki `joint.0` örneği gantry için genellenemez.

### 6.3 Özel lamba ve düğme: iki farklı genişletme seviyesi

**A. Mevcut panel pinlerinin özelleştirilmesi:** Kapalı `BUTTON_PINS` / `LED_PINS` kapsamındaki öğeleri sıralama, gizleme, etiketleme ve tipli eşleme. Bu mevcut simülasyon servisinin yeteneklerine yakındır; ancak yeni keyfi I/O oluşturmaz.

**B. Genel kullanıcı tanımlı sanal panel:** PyVCP benzeri yeni widget'lar için ayrı, açıkça tasarlanmış sanal HAL bileşeni veya harici panel entegrasyonu gerekir. Kullanıcı `bit`, `float`, `s32`, `u32` türünde pinlerle LED, sayısal gösterge, basılı buton, toggle ve sürgü tanımlar; HAL `net` bağlantılarını kurar. Widget pinlerinin kim tarafından oluşturulacağı, başlatma sırası, tek-yazarlı sinyal kuralı, yeniden bağlanma ve ömür açık tanımlanmalıdır. Bu özellik mevcut kapalı `linc.*` yazma arayüzünü genişletir veya ondan ayrı bir bileşen gerektirir; mevcut mimaride otomatik olarak zaten var değildir. Fiziksel panel protokolünün 64 bitlik haritası keyfi widget sayısını karşılamaz.

Gerçek makinelerde kontrolün yetkisi ve fiziksel panel/HAL güvenlik zinciri ürünün mevcut invariants'larıyla uyumlu tasarlanmalı. Kullanıcı isteği simülasyon panelini kapsamaktadır; bunu tüm canlı sistemlerde sınırsız HAL yazımına dönüştürmek ayrı ürün kararıdır. Tanı ekranının salt okunur HAL tarayıcısını genel yazıcıya çevirmemek gerekir. Her yeni kontrol için davranışın, yetkinin ve hedef bileşenin açık tanımı gerekir.

### 6.4 AXIS paneliyle uyumluluk seçenekleri

1. **LiNC'ye özgü deklaratif şema:** egui ile doğal görünüm ve denetim; PyVCP/GladeVCP dosyaları otomatik çalışmaz.
2. **Harici PyVCP/GladeVCP paneli çalıştırma/gömme:** mevcut LinuxCNC panellerini yeniden kullanabilir; pencere gömme, X11/Wayland, süreç ömrü ve olay odaklanması ayrıca araştırılmalıdır. LiNC'de böyle bir entegrasyon olduğu tespit edilmedi.
3. **İçe aktarma:** PyVCP XML öğelerinin belirli bir alt kümesini LiNC şemasına çevirme; desteklenmeyen öğeleri açıkça raporlama. PyVCP XML'inin bazı bölümleri Python olarak değerlendirilebilir (`docs/src/gui/pyvcp.adoc`, Security); güvenilmeyen dosyaları Python kodu olarak çalıştıran bir içe aktarıcı yapılmamalı.

“LinuxCNC'nin yapabildiği her şeyi” desteklemek; QtVCP, GladeVCP, Python handler'ları, özel HAL bileşenleri ve dış süreçler nedeniyle sadece birkaç yerleşik widget ile sağlanamaz. Kapsam ve uyumluluk seviyesi açıkça seçilmeli.

## 7. İstenilen geliştirme sırası

1. Aktif INI'nin gerçekten hangi dosya olduğunu ve panel servisinin aynı makineyi gördüğünü tanı ekranında göster; INI ve canlı durum farklıysa bildir.
2. Axis ve joint'i ayır; `XYZ`, `XZ`, `XYZAB`, `XYZC`, `XYZUVW`, `XYYZ` ve özel kinematik durumlarını modelle. Tekrarlı koordinatla tekilleştirilmiş axis listesini ve ayrı joint listesini koru. Mevcut `ini.rs:251` koordinatın sırasını joint indeksiyle eşliyor; genel kinematik için bu varsayımı incele.
3. Statik LED/jog listelerinin çizim filtresini kabiliyetlerden üret; kaynağı bilinmeyen veya desteklenmeyen kontrolde açıklama göster. HAL bağlı tuş, aynı zamanda doğru eksene ve doğru moda uygun olmalı.
4. Spindle için sayı/varsayılan ve her eylemin ayrı kabiliyetini çöz; 0, 1, 2 spindle vakalarını LinuxCNC sürümüne karşı doğrula. Yön/freni, sadece spindle sayısı var diye varsayma.
5. Limit ve coolant gibi standart öğelerin gerçek HAL durumunu ayrı, anlaşılır göster. Yalnızca `pin exists` veya `signal exists` üzerinden “tam çalışır” sonucu üretme.
6. Önce bilinen panel tuş/LED'lerini tanımlayan sürümlü yapılandırma ekle; sonra ihtiyaç varsa ayrı sanal I/O katmanıyla özel widget türlerini genişlet.
7. XYZAB sim profilinin A/B sürekli ve artımlı jog bağlantılarını ayrı uygulama konusu olarak ele al: axis/teleop ve joint/FREE modları, doğru joint eşlemesi, hız/birim, limit ve stop dahil. INI veya görsel değişiklik yeterli değildir.

## 8. Kabul ve test matrisi

| Senaryo | Beklenen |
|---|---|
| XYZ freze | Yalnız X/Y/Z axis göstergeleri; A/B/C veya U/V/W tanımlı değilse görünmez. |
| XZ torna | Y görünmez; joint eşlemesi gerçek kinematiğe dayanır, `joint 1 = Y` varsayılmaz. |
| XYZAB, A/B HAL bağlantısız | A/B tanımlı görünür ama jog eylemleri bağlantısız ve kapalı; bağlantı durumu açıklanır. |
| XYZAB, A/B HAL bağlı | Her yön hem sürekli hem artımlı; uygun axis ve joint modunda doğru hızla hareket eder ve bırakma/iletişim kaybında durur. |
| XYYZ gantry | Üç farklı axis, dört joint; eşlenen joint'ler kinematiğe göre; otomatik bire bir eşleştirme yapılmaz. |
| XYZC ve XYZUVW | Sabit 5 eksen panel yüzünün kapsamadığı kontroller doğru tespit edilir; destek varmış gibi sunulmaz. |
| Spindle varsayılanı / 0 / 1 / 2 | Eksik anahtar varsayılanı ve sıfırın geçerliliği hedef LinuxCNC ile test edilir; her spindle ve desteklenen işlev ayrı işlenir. |
| Flood bağlı / değil | Konfigürasyona ve seçilen politikaya göre görünür/gizli veya bağlantısız; gerçek çalışma geri bildirimi bağlantıyla karıştırılmaz. |
| Özel X limit LED | Açık kaynak tanımı varsa doğru değeri ve bilinmeyen/eskimiş durumu gösterir; tanım kaldırılınca görünmez. |
| Özel kontrol | Yetkisiz/yanlış tür/eksik pin reddedilir; mevcut panel kapsamı dışındaki eylem destekleniyor gösterilmez. |
| HAL geç bağlanma / kesilme | Başlangıçta bilinmeyen, bağlanınca güncel, kesilince kapalı/bilinmeyen; gömülü tutma eylemleri bırakılır. |
| `--no-hal` | Simüle edilmiş durumlar gerçek HAL bağlantısı olarak etiketlenmez. |
| Harici panel / PyVCP | Yalnız seçilen uyumluluk kapsamı desteklenir; kullanıcı XML/handler dosyalarının çalıştırma yetkisi açıktır. |

Bu rapor tasarım ve kod incelemesidir; önerilen şema ve genişletilebilir sanal panel uygulanmamıştır.

## 9. Genişletilmiş inceleme: ilk listede ayrıntılandırılmayanlar

Bu bölüm, yalnız AXIS'i değil, aynı yerel LinuxCNC ağacının panel ve HAL olanaklarını da kapsar. Aşağıdaki başlıklar doğrudan SIM PANEL'de mutlaka bulunması gereken hazır düğmeler listesi değildir: kullanıcıya sunulabilecek kabiliyet aileleridir. AXIS, GladeVCP ve QtVCP aynı ürün/arayüz değildir; birinin desteği LiNC'de veya AXIS'in her sürümünde otomatik olarak bulunduğu anlamına gelmez.

### 9.1 Standart makine işlevleri envanteri

Yerel `docs/src/gui/halui.adoc:89-327` HALUI başvurusunda şu aileler yer alır. Her satırın varlığı, o fonksiyonun belirli bir makinede bağlı veya kullanıma hazır olduğunun kanıtı değildir.

| Aile | Kullanıcıya sunulabilecek örnekler | Kaynak ve ayrım |
|---|---|---|
| Makine durumu ve güç | Machine on/off, çalışma durumu, birimler | `halui.machine.*`, `halui.estop.*`; acil durdurma ve reset istekleri gerçek güvenlik zinciriyle karıştırılmamalı. |
| Axis ve joint | Seçim, komutlanan/geri bildirim/göreli konum, fault, homed/unhome | `halui.axis.*`, `halui.joint.*`. Axis ve joint ayrı modeldir. |
| Homing | Tek joint home, home all, home durumu | `halui.home-all`, `halui.joint.N.home/is-homed`; belgede home-all pininin `HOME_SEQUENCE` şartına bağlı olduğu belirtilir. |
| Sınırlar | Pozitif/negatif donanımsal veya yazılımsal limit, geçici limit override durumu | `halui.joint.N.on-hard-*`, `on-soft-*`, `override-limits`; sensör girişi, yorumlanan durum ve override ayrı verilerdir. |
| Jog | Sürekli +/-; artımlı +/-; axis veya joint seçimi; analog joystick; jog deadband, hız | `halui.axis.*`, `halui.joint.*`; artımlı jog girişleri yükselen kenar, sürekli jog seviye mantığındadır. |
| El çarkı / MPG | Sayım, seçili eksen, ölçek/adım ve enable | Motion'ın `axis.*` / `joint.*` jog pinleri ve kullanılan pendant'ın kendi HAL entegrasyonu incelenmeli; hepsinin HALUI üzerinden çalıştığı varsayılmamalı. |
| Mod ve program | MANUAL/MDI/AUTO, joint/teleop; run/pause/resume/stop/step; block delete ve optional stop | `halui.mode.*`, `halui.program.*`, abort. İstek ile onaylanmış durum ayrıdır. |
| Override | Feed, rapid, spindle override; max velocity; sayaç/ölçek/direct-value veya artır/azalt | `halui.*override.*`, `halui.max-velocity.*`. Aralık, birim, ölçek ve bağlantı şekli farklı olabilir. |
| Spindle | Her spindle için yön, start/stop, hız artır/azalt, fren, durum ve override | `halui.spindle.N.*`; gerçek hız/encoder, at-speed, orient ve gerçek donanım geri bildirimi ilgili `spindle.N` / özel HAL grafiğinden ayrıca gelir. |
| Soğutma | Flood ve mist komut ve durumları | `halui.flood.*`, `halui.mist.*`. Bunlar mantıksal kontrol ve durumdur; fiziksel pompa geri bildirimi başka sinyal olabilir. |
| Takım | Seçili takım, çap, her eksendeki takım boy ofseti | `halui.tool.*`. Otomatik takım değiştirici, magazin, sensörler ve onay akışının tamamı bu birkaç pinle temsil edilmez; `iocontrol` ve makineye özel HAL/PLC'ye bakılır. |
| Özel MDI / makro | INI'de listelenen MDI komutlarını tetikleme, çalışma durumu | `halui.mdi-command-NN`; pinler `[HALUI] MDI_COMMAND` sayısına bağlıdır. Bu, bulunan rastgele G-code'u otomatik olarak kullanıcıya sunma yetkisi vermez. |
| Probe / takım ölçüm | Probe kontaktı, ölçüm döngüsü, sonuç ve alarmlar | `motion.probe-input` ve makineye özel akışlar; farklı probların ne anlama geldiği ve çalıştırma koşulları yapılandırılmalıdır. |
| Genel I/O / PLC | Kapı, hava, yağ, çalışma lambası, mengene, konveyör, alarm, servo-ready, özel süreçler | Kullanıcı tanımlı HAL bileşeni, I/O kartı, ClassicLadder veya diğer PLC/komponentler. Sabit isimli kapalı bir katalog ile tamamı kapsanamaz. |

Buradaki ayrım: HALUI'nin komut pinleri LinuxCNC'ye **istek** iletir. Pinlerin belgelenmiş olması, makinenin bunları kabloladığı, izin verdiği veya eylemin başarıyla tamamlandığı anlamına gelmez. `halui.*.is-*` gibi durumları ayrıca izlemek gerekir.

### 9.2 Widget ve yerleşim envanteri

- **Temel gösterimler:** metin/etiket, çok durumlu etiket, LED ve dikdörtgen LED, sayı/LCD, çubuk, progress, dairesel gösterge, birim ve formatlama.
- **Girişler:** anlık basılı buton, toggle/checkbutton, radio/seçim, +/- artırıcı, sayısal giriş, slider, dial/encoder, jog wheel ve joypad. Bunlar birbirine dönüştürülebilir sanılmamalı: tutma, toggle, yükselen kenar, artış ve mutlak değer farklı protokollerdir.
- **Yerleşim:** yatay/dikey kutu, grid, gruplama, sekmeler, kaydırma, koşullu etkinlik/görünürlük, kullanıcı yazısı/renk/ikon ve değiştirilebilir görseller.
- **Gelişmiş:** HAL grafik, DRO, G-code gösterimi, takım/ofset tabloları, dosya seçimi, MDI, probing, kamera hizalama, durumlar, diyaloglar ve 3B Vismach. Bunların çoğu bağımsız kullanıcı panelinin ötesindedir ve yerel GladeVCP/QtVCP widget'larıdır; AXIS'in temel simülasyon panelinde otomatik olarak oluşmaz.
- **Programlanabilir uzantılar:** GladeVCP/QtVCP Python handler, callback, timer, kalıcı ayar, özel HAL pinleri ve özel kod; AXIS'te `.axisrc` / `USER_COMMAND_FILE` ve `user_hal_pins`. Kullanıcının istediği özgürlük, yalnızca birkaç sabit widget seçeneğinden daha geniştir.

Kaynaklar: `docs/src/gui/pyvcp.adoc:326-950`, `docs/src/gui/gladevcp.adoc:447-2734`, `docs/src/gui/qtvcp-widgets.adoc:27-3342`, `docs/src/gui/qtvcp.adoc:139-157`. QtVCP `test_led` ve `test_button` örnekleri HAL test paneli fikri açısından ayrıca incelenebilir: `docs/src/gui/qtvcp-vcp-panels.adoc:110-200`.

### 9.3 HAL veri türleri, yönleri ve sürüm farkı

Önceki §6.3'te verilen `bit`, `float`, `s32`, `u32` ifadeleri **yaygın/eski HAL adlandırmasını açıklayan örneklerdir**, bu yerel checkout'un tek ve güncel tür listesi değildir. İncelenen `include/hal.h:361-376` `HAL_BOOL`, `HAL_REAL`, `HAL_SINT`, `HAL_UINT` ve `HAL_PORT` tanımlar; eski `HAL_S32/U32`, `HAL_S64/U64`, `HAL_FLOAT`, `HAL_BIT` adları için deprecation/alias notları içerir. Gerçek sürümün ABI'si ve genişliği ayrıca doğrulanmadan 32/64-bit uyumu varsayılmamalı. HAL `PORT` veri akışıdır; basit sayı/LED widget'ı gibi ele alınmamalıdır.

Bir widget için asgari bağlantı sözleşmesi: kaynak veya hedef adı, pin/sinyal/parametre türü, HAL veri türü, component yönü (`IN`, `OUT`, `IO`), okuma/yazma yetkisi, değer aralığı ve ölçek, tazelik, başlangıç/bağlantı kaybı değeri ve sinyalin mevcut writer/reader durumu. Bir sinyalin bağlı olması veya output pinin kendisinin writer sayılması downstream cihazın bulunduğunu kanıtlamaz. Yerel `src/hal/halmodule.cc:1152-1172` `pin_has_writer` için bağlı sinyalin `writers > 0` koşulunu uygular; `HAL_IO`/bidirectional durumları da ayrıca değerlendirilmelidir.

`net`, `setp`, `sets`, `unlinkp`, bileşen yükleme ve thread'e fonksiyon ekleme ayrı işlemlerdir (`docs/src/hal/basic-hal.adoc`). Pin ile sinyal ve parametre eş anlamlı değildir. Mevcut bağlı bir input pinine ikinci sinyal bağlamaya çalışmak yerine mevcut grafik doğru modellenmelidir.

### 9.4 Yükleme sırası ve keşfin sınırı

`HALFILE` birden fazla kez belirtilebilir; `.hal` yanında Tcl, `LIB:` yolları, kullanıcı süreçleri ve `TWOPASS` kullanılabilir. `POSTGUI_HALFILE` widget pinleri ortaya çıktıktan sonra bağlanabilir. `docs/src/config/ini-config.adoc:712-763`, `docs/src/hal/twopass.adoc` ve `docs/src/gui/pyvcp.adoc:138` örnekleri buna bakar. Bu yüzden:

- Yalnız INI ve HAL metnini regex ile taramak gerçek çalışma grafiğinin güvenilir eşdeğeri değildir. Değişkenler, Tcl, yüklenen bileşenler ve sonradan bağlantılar vardır.
- Canlı HAL grafiği de kullanıcının niyetini her durumda ifade etmez; ilk okuma post-GUI bağlantılarından önce gelebilir.
- Panel hazır, HAL hazır, bağlantılar güncel, süreç kapandı/yeniden başladı durumları takip edilmeli; pin var ama bağlantısız ile okuma hatası ayırt edilmeli.
- Kapsamlı destek için hem statik konfigürasyon doğrulaması hem de canlı grafik tanıları gereklidir; canlı sistem yoksa “tespit edilemedi” denmelidir.

### 9.5 Kaynağa özgü sınırlar ve doğrulanması gerekenler

Bu yerel LinuxCNC checkout'unda `include/emcmotcfg.h:25-33` **16 joint, 9 axis, 8 spindle** sabitleri tanımlar. Bunlar bu kaynak ağacının derleme sınırlarıdır; başka sürüm, derleme ve kullanılan kinematik için değişebileceğinden evrensel ürün garantisi değildir. Axis adları X/Y/Z/A/B/C/U/V/W'dir; joint sayısının axis sayısından büyük olabilmesi ayrıca önemlidir. Maksimum spindle sabiti, her GUI'nin sekiz spindle'ı eksiksiz yönettiğini veya sıfır spindle konfigürasyonunun her katmanda geçerli olduğunu göstermez.

Ek uyumluluk testleri: spindle sayısı >2; aynı sinyale birkaç reader; bağlantısız veya writer'sız sinyal; HAL_IO; kaldırılıp yeniden yaratılan pinler; geç oluşturulan post-GUI pinleri; farklı ad/tip ve Unicode etiketler; analog ölçek/birim dönüşümü; momentary release/odak kaybı; kalıcı toggle başlangıcı; RPM komutu ile gerçek RPM farkı; home/limit/probe bağlantıları; kullanılan LinuxCNC sürümünün tip/ABI özellikleri; özel kinematik ve tek axis üzerinde çok joint; farklı panel driver'ları; XML/handler içe aktarma sınırı.

### 9.6 Kapsam kararı

Bu tarama yukarıdaki standart aileleri ve belgelenmiş uzantı mekanizmalarını yakalar; **üçüncü taraf veya makineye özel her HAL bileşenini ve gelecekte eklenebilecek işlevi sonlu bir listeyle garanti edemez**. Gerçek genişletilebilirlik için sabit özellik kataloğunun yanında tipli kullanıcı tanımı, uyum/bağlantı tanısı ve seçilen güven sınırına göre harici panel/bileşen desteği gereklidir. Tam QtVCP/GladeVCP/PyVCP davranış uyumu ancak her widget ve handler sözleşmesi tek tek uygulandığında veya ilgili ortam doğrudan desteklendiğinde iddia edilmelidir.
