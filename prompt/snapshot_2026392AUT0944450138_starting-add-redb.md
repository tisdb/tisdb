# CODE SNAPSHOT v:2026392AUT0944450138 (starting add redb)

## Metadata

```text
├─ Katalog roboczy (CWD): A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT_tisdb/tisdb
├─ Lokalizacje (scan_at): ["./"]
└─ Wzorce (match_pattern): ["./{examples|tests|docs|src}/**", "./{Cargo.toml|README.md|MODEL.md|ARCHITECTURE.md}"]
📦 Zeskanowano fizycznie: 2004 plików, 285 katalogów
```

## Structure

```plaintext
  ▣─┬ tisdb                             [499.7 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                    _tisdb/tisdb/                       
 1  ├──• ARCHITECTURE.md                [  5.2 KiB] [2026-39-2 09:37:04] ./ARCHITECTURE.md                   
 2  ├──• Cargo.toml                     [  1.7 KiB] [2026-39-2 09:41:00] ./Cargo.toml                        
 3  ├──• MODEL.md                       [ 15.4 KiB] [2026-39-2 09:37:16] ./MODEL.md                          
 4  ├──• README.md                      [  1.4 KiB] [2026-39-2 09:36:23] ./README.md                         
    ├──┬ docs                           [417.7 KiB] [2026-39-2 07:45:33] ./docs/                             
    │  └──┬ images                      [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/                      
    │     └──• GRAFY_CISOWSKIEGO.png    [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/GRAFY_CISOWSKIEGO.png 
    ├──┬ src                            [ 44.2 KiB] [2026-39-2 07:47:10] ./src/                              
 5  │  ├──• engine.rs                   [ 12.8 KiB] [2026-39-1 23:08:51] ./src/engine.rs                     
 6  │  ├──• error.rs                    [  1.1 KiB] [2026-39-1 22:33:50] ./src/error.rs                      
 7  │  ├──• lib.rs                      [    858 B] [2026-39-2 08:59:18] ./src/lib.rs                        
    │  ├──┬ domain                      [ 11.6 KiB] [2026-39-2 07:47:10] ./src/domain/                       
 8  │  │  ├──• flow.rs                  [  1.8 KiB] [2026-39-1 21:54:01] ./src/domain/flow.rs                
 9  │  │  ├──• hyper.rs                 [  2.0 KiB] [2026-39-1 21:52:34] ./src/domain/hyper.rs               
10  │  │  ├──• id.rs                    [  1.9 KiB] [2026-37-3 13:41:25] ./src/domain/id.rs                  
11  │  │  ├──• metadata.rs              [  3.6 KiB] [2026-39-1 22:48:17] ./src/domain/metadata.rs            
12  │  │  ├──• node.rs                  [    701 B] [2026-39-1 21:52:19] ./src/domain/node.rs                
13  │  │  └──• zone.rs                  [  1.5 KiB] [2026-39-1 21:53:46] ./src/domain/zone.rs                
    │  └──┬ storage                     [ 17.9 KiB] [2026-39-2 08:58:04] ./src/storage/                      
14  │     ├──• backend.rs               [    610 B] [2026-39-1 23:07:07] ./src/storage/backend.rs            
15  │     ├──• file.rs                  [  3.7 KiB] [2026-39-1 23:07:44] ./src/storage/file.rs               
16  │     ├──• memory.rs                [  1.1 KiB] [2026-39-1 23:08:08] ./src/storage/memory.rs             
17  │     └──• redb_backend.rs          [ 12.4 KiB] [2026-39-2 09:02:45] ./src/storage/redb_backend.rs       
    └──┬ tests                          [ 14.0 KiB] [2026-39-2 08:53:26] ./tests/                            
18     ├──• axiom_tests.rs              [  1.9 KiB] [2026-39-1 21:55:27] ./tests/axiom_tests.rs              
19     ├──• metadata_tests.rs           [  1.1 KiB] [2026-39-1 21:56:23] ./tests/metadata_tests.rs           
20     ├──• persistence_tests.rs        [  4.6 KiB] [2026-39-1 22:49:39] ./tests/persistence_tests.rs        
21     ├──• redb_poc_tests.rs           [  4.8 KiB] [2026-39-2 08:53:14] ./tests/redb_poc_tests.rs           
22     └──• storage_tests.rs            [  1.6 KiB] [2026-39-1 22:50:01] ./tests/storage_tests.rs            
```

## Source Code Content

### [1] `./ARCHITECTURE.md`

```markdown
# Architektura TISDB (Architecture & Decision Log)

[![Crates.io](https://img.shields.io/crates/v/tisdb.svg)](https://crates.io/crates/tisdb)
[![Documentation](https://docs.rs/tisdb/badge.svg)](https://docs.rs/tisdb)
[![GitHub](https://img.shields.io/badge/GitHub-tisdb-181717?logo=github)](https://github.com/tisdb/tisdb)

Niniejszy dokument opisuje kluczowe decyzje architektoniczne podjęte podczas tworzenia silnika bazy danych TISDB, ze szczególnym uwzględnieniem ewolucji warstwy fizycznego zapisu (Storage Layer). Dokument służy jako przewodnik dla kontrybutorów, wyjaśniający "dlaczego" system został zbudowany w taki, a nie inny sposób.

## 1. Podział Odpowiedzialności (Separation of Concerns)

Architektura TISDB opiera się na twardym oddzieleniu logiki domenowej od fizycznego zapisu danych.

*   **`CisowskiEngine` (Warstwa Domenowa):** Silnik ontologiczno-logiczny grafu. Odpowiada wyłącznie za walidację relacji, spójność encji (N, H, Z, F) i przestrzeganie Aksjomatów TISOS (tzw. *Guardrails*).
*   **`RedbStorage` (Warstwa Fizyczna):** Silnik K-V (Klucz-Wartość) zarządzający operacjami dyskowymi, transakcjami ACID, izolacją współbieżności i integralnością bajtów.

## 2. Ewolucja Warstwy Storage (Log Decyzji)

### Faza 1: Naiwne Snapshoty i problem RAM (Odrzucone)
Początkowy prototyp `CisowskiEngine` opierał się na przetrzymywaniu całego grafu w pamięci operacyjnej (`HashMap<NodeId, Node>`) i zrzucaniu jej w całości do pliku binarnego na dysku.
*   **Dlaczego z tego zrezygnowaliśmy?** Przy setkach tysięcy węzłów ładowanie całego grafu do RAM (deserializacja) oraz zrzucanie gigabajtowych snapshotów przy każdej drobnej zmianie całkowicie zabijało wydajność i zasoby I/O. System nie mógłby skalować się do rozmiarów produkcyjnych.

### Faza 2: Eksperyment z Mmap, `rkyv` i pułapka "Unaligned" (Odrzucone)
Aby rozwiązać problem pamięci, zdecydowaliśmy się na wykorzystanie bezpośredniego mapowania plików do pamięci (Mmap) z pomocą formatu zero-copy `rkyv`. Chcieliśmy nałożyć strukturę Rusta bezpośrednio na bajty na dysku.
Podczas budowy Proof-of-Concept napotkaliśmy jednak krytyczny błąd sprzętowy procesorów: `UnalignedPointer`. 
*   **Problem:** Systemy K-V zwracają wskaźniki na bajty z dysku. Domyślny format `rkyv` (oraz architektura CPU) wymaga, aby 8-bajtowe liczby (np. `u64`) były umieszczone pod adresami wielokrotności liczby 8. Bezpośredni zrzut powodował naruszenie tego wyrównania (Alignment Error), co w językach C/C++ skończyłoby się awarią systemu (Segfault), a w Rust skutkowało bezpiecznym, ale twardym odrzuceniem danych przez walidator.

### Faza 3: Docelowy Model - `redb` + `rkyv (unaligned)` (Wdrożone)
Stanęliśmy przed wyborem: kopiować dane z dysku do wyrównanych buforów (`AlignedVec` - utrata zalet Zero-Copy) lub zmienić definicję układu binarnego. Wybraliśmy to drugie rozwiązanie.

#### Decyzja: Wybór silnika `redb`
Do zarządzania plikiem użyto `redb`. Wybrano go ze względu na:
1.  **Odporność na awarie (Crash-safe):** Struktura B-Tree z mechanizmem *Copy-on-Write* (CoW) gwarantuje, że aktywne dane nigdy nie są nadpisywane, dopóki transakcja nie zostanie potwierdzona (fsync).
2.  **Jednoplikowość:** Cała baza mieści się w jednym przenośnym pliku `.cdb`.
3.  **Transakcje ACID:** Baza wspiera złożone modyfikacje wielotabelowe. Testowaliśmy przerywanie takich transakcji (rollback/abort) i wykazaliśmy, że `redb` doskonale izoluje brudne dane i zapobiega uszkodzeniom strukturalnym.

#### Decyzja: `rkyv` z flagą `unaligned`
Aby pogodzić mapowanie pamięci `redb` z bezpośrednim rzutowaniem zmiennych, skonfigurowaliśmy `rkyv` z flagami `["unaligned", "pointer_width_32"]`.
*   **Dlaczego?** Zmusza to Rusta i rkyv do odczytywania liczb bajt po bajcie (bez polegania na wbudowanych instrukcjach CPU zakładających ścisłe wyrównanie adresów). Zyskaliśmy w ten sposób prawdziwe **Zero-Copy z mmap**, eliminując koszty stertowych alokacji (alloc/copy) przy każdym odczycie encji, przy jednoczesnym zachowaniu pełnego bezpieczeństwa pamięci (memory-safe). Format ten gwarantuje też zgodność między maszynami 32 i 64-bitowymi.

## 3. Organizacja Danych w B-Tree

Fizyczny plik `redb` dzieli się na następujące tabele:
*   `metadata` (Klucz: `&str`, Wartość: `u32`) - przechowuje metadane bazy i wersję formatu.
*   `nodes` (Klucz: `[u8; 16]`, Wartość: Zserializowane w unaligned rkyv bajty encji)
*   `hypers` (Klucz: `[u8; 16]`, Wartość: Zserializowane bajty)
*   `zones` (Klucz: `[u8; 16]`, Wartość: Zserializowane bajty)
*   `flows` (Klucz: `[u8; 16]`, Wartość: Zserializowane bajty)

Użycie `[u8; 16]` (odpowiadające wygenerowanym identyfikatorom typu ULID) jako klucza gwarantuje ekstremalnie szybkie czasy przeszukiwania B-Drzewa przy zachowaniu chronologicznej (monotonicznej) lokalności danych.

## 4. Przyszłość (Roadmap)

Kolejnym kluczowym krokiem rozwojowym, dla którego fundamentem jest obecna architektura fizyczna, będzie wdrożenie **Silnika Kwerend (Query Engine)**. Zbudujemy warstwę zapytań polimorficznych, pozwalającą wyciągać podgrafy bezpośrednio na podstawie ścieżek klas ontologicznych (`class_path`) zapisanych w metadanych encji, korzystając z wydajności indeksów `redb`.

---
---

🫟

```

### [2] `./Cargo.toml`

```toml
[package]
name = "tisdb"
version = "0.0.1-dev.1"
authors = ["Jan Roman Cisowski „j-Cis” <code@cisowscy.com>"]
license = "MIT OR Apache-2.0"
edition = "2024"
rust-version = "1.98.0"
repository = "https://github.com/tisdb/tisdb"
resolver = "3"
description = "TISDB is a hypergraph database engine tailored for TISOS (Topologically Integrated System of Semantic Overlays), featuring strict axiom enforcement, atomic I/O, and Redb-based storage."
keywords = ["database", "graph", "hypergraph", "redb"]
categories = ["database", "database-implementations", "data-structures"]
include = [
    "src/**/*",
    "docs/**/*",
    "Cargo.toml",
    "README.md",
    "ARCHITECTURE.md",
    "MODEL.rs",
    "LICENSE*",
]

[package.metadata.cargo]
edition = "2024"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
targets = [
    "x86_64-pc-windows-msvc",
    "i686-pc-windows-msvc",
    "x86_64-unknown-linux-gnu",
    "x86_64-unknown-linux-musl",
    "i686-unknown-linux-gnu",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin"
]

[lib]
crate-type = ["cdylib", "rlib"]

[badges]
travis-ci = { repository = "tisdb/tisdb" }
appveyor = { repository = "tisdb/tisdb" }

[dependencies]
fslock = "0.2.1"
serde = { version = "1.0.229", features = ["derive"] }
ulid = "3.0.0"
thiserror = "2.0.20"
wasm-bindgen = { version = "0.2.128", optional = true }

redb = "4.3.0"
# Specyfikacja gwarantująca pełne Zero-Copy z redb bez AlignmentError 
# oraz spójność między architekturami (32/64 bit).
rkyv = { version = "0.8.18", default-features = false, features = [
    "alloc",
    "std",
    "bytecheck",
    "unaligned",
    "little_endian",
    "pointer_width_32",
] }

[features]
wasm-bindgen = ["dep:wasm-bindgen"]


```

### [3] `./MODEL.md`

```markdown
# Cisowski's Hyperconnector Model (Hiperkonektory Cisowskiego / Grafy Cisowskiego)

[![Crates.io](https://img.shields.io/crates/v/tisdb.svg)](https://crates.io/crates/tisdb)
[![Documentation](https://docs.rs/tisdb/badge.svg)](https://docs.rs/tisdb)
[![GitHub](https://img.shields.io/badge/GitHub-tisdb-181717?logo=github)](https://github.com/tisdb/tisdb)

Wprowadzam koncepcje "Grafów Cisowskiego" - są nadrzędnym meta-modelem (nadzbiorem), klasyczne teorie grafów i hipergrafów to jedynie jego zubożone podstruktury.

![Diagram](docs/images/GRAFY_CISOWSKIEGO.png)

1. Klasyczny graf skierowany $\Big(\Phi\Big)$

   - **Tradycyjne ujęcie:** Bezpośrednia relacja binarna $1:1$ łącząca dwa wierzchołki $\Big(u \to v\Big)$. Krawędź jest jedynie „płaskim” połączeniem pozbawionym własnej struktury wewnętrznej.

   - **Redukcja w Meta-Modelu:**  Hiperkonektor zredukowany do dwóch jednoelementowych stref portowych $\Big(z_e^{(1)}$ zawiera $u$, $z_e^{(2)}$ zawiera $v\Big)$ powiązanych pojedynczą instancją przepływu skierowanego $\Big(\pi_F(f_e) = (z_e^{(1)}, z_e^{(2)}, \mathrm{dir})\Big)$.

2. Klasyczny hipergraf Berge’a $\Big(\Phi_{\mathrm{hyper}}\Big)$

   - **Tradycyjne ujęcie:** Płaski podzbiór wierzchołków $\Big(e \subseteq V\Big)$. Wszystkie elementy w krawędzi są równorzędne — brak tam jakichkolwiek wyróżnionych ról, portów czy struktury wewnętrznej.

   - **Redukcja w Meta-Modelu:** Skrajnie uproszczony hiperkonektor całkowicie pozbawiony wewnętrznej dynamiki relacyjnej $\Big(\vert{}F_{h_e}\vert{} = 0\Big)$, posiadający zaledwie jedną strefę portową $\Big(z_e\Big)$, której funkcja zawartości przechowuje pełny zbiór wierzchołków $\Big(\mu_Z(z_e) = \Phi_{\mathrm{hyper}}(e)\Big)$.

3. Klasyczny skierowany hipergraf $\Big(\Phi_{\mathrm{dir\_hyper}}\Big)$

   - **Tradycyjne ujęcie:** Relacja wieloargumentowa łącząca podzbiór wejściowy (Tail) z podzbiorem wyjściowym (Head) w formule $T \to H$.

   - **Redukcja w Meta-Modelu:** Hiperkonektor posiadający dokładnie dwie strefy interfejsowe $\Big(z_e^{\mathrm{tail}}$ oraz $z_e^{\mathrm{head}}\Big)$, z których każda agreguje odpowiedni podzbiór wierzchołków, spięte jednym wewnętrznym przepływem skierowanym między tymi portami.

4. Metagrafy / Grafy hierarchiczne

   - **Tradycyjne ujęcie:** Umożliwiają łączenie całych podgrafów lub krawędzi z innymi krawędziami, jednak często cierpią na brak twardej izolacji (połączenia skrośne przeskakują poziomy zagnieżdżenia).

   - **Redukcja w Meta-Modelu:** Pełna, inżynieryjna enkapsulacja. Dowolna strefa może zawierać inne hiperkonektory $\Big(\mu_Z(z) \subseteq U\Big)$, tworząc ufundowaną strukturę zagnieżdżoną $\Big(\mathcal{R}_{\mathrm{contain}}\Big)$, w której jakikolwiek ruch skrośny jest ściśle kontrolowany przez dedykowane porty $\Big(Z_h\Big)$ i lokalne przepływy $\Big(F_h\Big)$.

## Słownik Ontologiczny

### ♦️ Węzły ($N$ – Nodes / Atoms)

- Podstawowe, niepodzielne obiekty lub jednostki danych, które nie posiadają żadnej struktury wewnętrznej ani portów.

- Atomy ontologiczne. Byty pozbawione wewnętrznej struktury topologicznej ($\vert{}Z_n\vert{}$ nie istnieje). Stanowią podstawowe jednostki danych lub obiekty niepodzielne w uniwersum.

### ♣️ Hiperkonektory / Hiperkrawędzie ($H$ – Hyperconnectors / Relational Complexes)

- Autonomiczne kontenery relacji, które posiadają własną tożsamość, własne porty (strefy) oraz wewnętrzne połączenia (przepływy).

- Autonomiczne byty relacyjne posiadające własną tożsamość oraz przypisaną strukturę wewnętrzną $(Z_h, F_h)$. Użycie terminu hiperkrawędź w niniejszym formalizmie ma charakter rozszerzający i należy je rozumieć w sensie hiperkonektora (kompleksu portowego). W przeciwieństwie do klasycznej hiperkrawędzi Berge’a (będącej jedynie płaskim podbiorem wierzchołków), $H$ definiuje lokalny kontekst topologiczny posiadający własne porty (strefy) oraz wewnętrzną dynamikę (przepływy).

### ♥️ Strefy ($Z$ – Zones / Ports)

- Dyskretne punkty styku wewnątrz hiperkonektora. Pełnią podwójną rolę: są portami dla wewnętrznych przepływów oraz kontenerami na obiekty ($N$ i $H$). Ten sam obiekt z uniwersum (zarówno węzeł, jak i inny zagnieżdżony hiperkonektor) może znajdować się w zawartości wielu stref naraz.

- Dyskretne elementy strukturalne stanowiące punkty styku (porty / role / interfejsy) danej hiperkrawędzi. Strefa $z \in Z_h$ nie jest zbiorem, lecz instancją portu o unikalnej tożsamości globalnej ($z_1 \neq z_2$). Pełni podwójną rolę: interfejsu relacyjnego (dla funkcji $\pi_F$) oraz kontenera zawartości (dla funkcji $\mu_Z(z) \subseteq U$). Zawartość stref ($\mu_Z$), obejmująca węzły $N$ oraz hiperkonektory $H$, może być współdzielona.

### ♠️ Przepływy ($F$ – Flows / Relational Instances)

- Autonomiczne połączenia relacyjne zachodzące wyłącznie pomiędzy strefami należącymi do tego samego hiperkonektora.

- Autonomiczne instancje relacji zachodzące wyłącznie pomiędzy strefami należącymi do tej samej hiperkrawędzi ($Z_h \times Z_h$). Przepływ $f \in F_h$ nie jest zbiorem par, lecz unikalnym obiektem wskazywanym przez funkcję przydziału $\pi_F$, niosącym atomowy typ orientacji $T = \{\mathrm{sym}, \mathrm{dir}, \mathrm{bidir}\}$.

## Aksjomaty Systemu

### ♦️ Aksjomat 1 (Ontologia Węzłów)

- W systemie istnieją węzły ($N$), które są prostymi, niepodzielnymi elementami (atomami) bez portów czy struktury wewnętrznej.

- Istnieje zbiór węzłów $N$, których elementy są ontologicznymi atomami pozbawionymi struktury portowej.

### ♣️ Aksjomat 2 (Ontologia Hiperkonektorów)

- W systemie istnieją hiperkonektory ($H$), które są całkowicie odrębne od węzłów ($N$). Razem z węzłami tworzą pełne uniwersum obiektów ($U$).

- Istnieje zbiór hiperkonektorów $H$. Zbiory $N$ oraz $H$ są rozłączne i tworzą uniwersum obiektów:
   $U = N \sqcup H \quad \mathrm{gdzie} \quad N \cap H = \emptyset$ .

### ♥️ Aksjomat 3 (Lokalność i Rozłączność Stref)

- Każdy hiperkonektor ma swój własny, unikalny zestaw stref (portów). Jedna strefa nie może należeć do dwóch hiperkonektorów naraz.

- Każdemu hiperkonektorowi $h \in H$ przypisana jest lokalna rodzina stref $Z_h$. Strefy należące do różnych hiperkonektorów są ściśle rozłączne:
   $\forall h_1, h_2 \in H \quad (h_1 \neq h_2 \implies Z_{h_1} \cap Z_{h_2} = \emptyset)$ .

### ♥️ Aksjomat 4 (Globalne Uniwersum Stref)

- Wszystkie strefy w całym systemie tworzą łączną przestrzeń $Z$. Strefa nie może istnieć „luzem” poza swoim hiperkonektorem.

- Globalną przestrzeń stref $Z$ definiuje się jako sumę rozłączną rodzin lokalnych:
   $$Z = \bigsqcup_{h \in H} Z_h$$
   Strefy nie istnieją jako samodzielne byty poza swoim hiperkonektorem macierzystym.

### ♠️ Aksjomat 5 (Lokalność i Rozłączność Przepływów)

- Przepływy są w 100% lokalne – dany przepływ należy ściśle do swojego hiperkonektora i nie może bezpośrednio wychodzić poza niego.

- Każdemu hiperkonektorowi $h \in H$ przypisana jest lokalna rodzina instancji przepływów $F_h$. Przepływy należące do różnych hiperkonektorów są ściśle rozłączne:
   $$\forall h_1, h_2 \in H \quad (h_1 \neq h_2 \implies F_{h_1} \cap F_{h_2} = \emptyset)$$

### ♠️ Aksjomat 6 (Globalne Uniwersum Przepływów)

- Cała przestrzeń przepływów w systemie ($F$) składa się wyłącznie z połączeń zdefiniowanych w poszczególnych hiperkonektorach.

- Globalną przestrzeń przepływów $F$ definiuje się jako sumę rozłączną rodzin lokalnych:
   $$F = \bigsqcup_{h \in H} F_h$$
   Przepływy nie istnieją jako samodzielne byty poza swoim hiperkonektorem macierzystym.

### ♣️ Aksjomat 7 (Odwzorowanie Struktury Hiperkonektora)

- Każdy hiperkonektor ma przypisaną strukturę złożoną ze stref i przepływów. Może posiadać zero stref (byt bezportowy) lub zero przepływów.

- Struktura dowolnego hiperkonektora $h \in H$ jest określona przez funkcję przydziału:
   $$\sigma: H \to \mathcal{P}(Z) \times \mathcal{P}(F) \quad \mathrm{gdzie} \quad \sigma(h) = (Z_h, F_h)$$
   Dopuszczalne są liczności $\vert{}Z_h\vert{} \ge 0$ oraz $\vert{}F_h\vert{} \ge 0$. Stan $\vert{}Z_h\vert{} = 0$ definiuje byt bezportowy (placeholder).

### ♥️ Aksjomat 8 (Zawartość Stref i Współdzielenie Zawartości)

- Strefa może być pusta lub zawierać w sobie dowolne węzły ($N$) oraz zagnieżdżone hiperkonektory ($H$). Ten sam obiekt z uniwersum $U$ (dowolny węzeł lub inny hiperkonektor) może znajdować się jednocześnie w zawartości wielu różnych stref..

- Zawartość każdej strefy $z \in Z$ opisuje funkcja:
   $$\mu_Z: Z \to \mathcal{P}(U)$$
   Strefa może być pusta ($\mu_Z(z) = \emptyset$) lub zawierać dowolny podzbiór węzłów $N$ i/lub hiperkonektorów $H$. Obrazy funkcji $\mu_Z$ dla różnych stref nie muszą być rozłączne — dozwolone jest współdzielenie obiektów z uniwersum $U = N \sqcup H$ przez odrębne strefy.

### ♥️ Aksjomat 9 (Relacja Zagnieżdżenia i Aksjomat Ufundowania)

- Hiperkonektory mogą zawierać w swoich strefach inne hiperkonektory, tworząc strukturę zagnieżdżoną. Zagnieżdżanie nie może jednak tworzyć cykli (samozawierania) ani nieskończonej głębokości.

- Relację bezpośredniego zagnieżdżenia $\mathcal{R}_{\mathrm{contain}} \subseteq U \times U$ definiuje się jako:
   $$(h, u) \in \mathcal{R}_{\mathrm{contain}} \iff h \in H \land \exists z \in Z_h \bigl(u \in \mu_Z(z)\bigr)$$
   Relacja $\mathcal{R}_{\mathrm{contain}}$ jest dobrze ufundowana (well-founded), co wyklucza istnienie nieskończonych łańcuchów zagnieżdżenia $u_0 \ni u_1 \ni u_2 \dots$. Ponadto jej domknięcie przechodnie $\mathcal{R}_{\mathrm{contain}}^+$ jest ściśle irrefleksywne:
   $$\forall u \in U \quad (u, u) \notin \mathcal{R}_{\mathrm{contain}}^+$$

### ♠️ Aksjomat 10 (Instancja i Typowanie Przepływu)

- Każdy przepływ wewnątrz hiperkonektora łączy dwie jego strefy i posiada jeden z trzech typów połączenia: nieskierowane (—), skierowane (⟶) lub dwukierunkowe (⟷).

- Każda instancja przepływu $f \in F_h$ jest odwzorowywana przez funkcję przydziału:
   $$\pi_F: F_h \to Z_h \times Z_h \times T \quad \mathrm{gdzie} \quad T = \{\mathrm{sym}, \mathrm{dir}, \mathrm{bidir}\}$$

### ♠️ Aksjomat 11 (Symetria Reprezentacji i Auto-Przepływy)

- Przy połączeniu nieskierowanym (—) lub dwukierunkowym (⟷) kolejność podania stref nie ma znaczenia. Mimo tej samej symetrii portowej, (—) oznacza powiązanie statyczne (np. styk), a (⟷) aktywny kanał dwukierunkowy (f⟷). Przepływ może również łączyć strefę z samą sobą (auto-przepływ).

- Dla typów orientacji $t \in \{\mathrm{sym}, \mathrm{bidir}\}$ zachodzi symetria reprezentacji krotki modulo relacja równoważności $\equiv$:
   $$(z_1, z_2, t) \equiv (z_2, z_1, t)$$
   Dla $t = \mathrm{dir}$ kolejność argumentów w parze jest asymetryczna. Dozwolone są auto-przepływy, gdzie $z_1 = z_2$.

> ℹ️ Uwaga dotycząca semantyki typów: Relacja $\equiv$ określa wyłącznie symetrię podłączenia do portów $Z_h \times Z_h$. Typy sym i bidir stanowią jednak rozłączne elementy zbioru $T$: typ sym reprezentuje statyczną relację nieskierowaną (np. styk/potencjał), natomiast bidir reprezentuje atomowy, aktywny kanał dwukierunkowy ($f_{\leftrightarrow}$), którego pełna dwustronna dynamika jest interpretowana w Warstwie Semantycznej ($\mathcal{S}$).

### ♠️ Aksjomat 12 (Multigrafowość Warstwy Relacyjnej)

- Między tymi samymi dwiema strefami w tym samym hiperkonektorze może istnieć wiele niezależnych, równoległych przepływów tego samego typu.

- Funkcja $\pi_F$ nie musi być iniekcją. Dwa odrębne obiekty przepływu $f_1, f_2 \in F_h$ ($f_1 \neq f_2$) mogą posiadać identyczny obraz $\pi_F(f_1) = \pi_F(f_2)$, co tworzy strukturę multigrafu.

## Definicje Zanurzenia

### 📖 Definicja Kanonicznego Zanurzenia Iniekcyjnego ($\Phi$) (Klasycznej Krawędzi w Grafie)

- Klasyczna krawędź grafu (np. $A \to B$) jest po prostu szczególnym przypadkiem hiperkonektora, który posiada dwie odrębne strefy (zawierające odpowiednio $A$ i $B$) oraz jeden przepływ skierowany między nimi.

- Dowolny klasyczny graf skierowany $G = (V, E)$, gdzie $E \subseteq V \times V$, jest reprezentowalny w meta-modelu poprzez kanoniczne zanurzenie iniekcyjne:
   
   $$\Phi: V \sqcup E \hookrightarrow U, \quad \mathrm{gdzie} \quad \Phi(V) \subseteq N \quad \mathrm{oraz} \quad \Phi(E) \subseteq H$$
   
   Dla każdej krawędzi $e = (u, v) \in E$ niech $h_e = \Phi(e) \in H$. Hiperkonektor $h_e$ posiada strukturę $\sigma(h_e) = (\{z_e^{(1)}, z_e^{(2)}\}, \{f_e\})$, gdzie $z_e^{(1)} \neq z_e^{(2)}$, i spełnia warunki:
   
   $$\mu_Z(z_e^{(1)}) = \{\Phi(u)\}, \quad \mu_Z(z_e^{(2)}) = \{\Phi(v)\}, \quad \pi_F(f_e) = (z_e^{(1)}, z_e^{(2)}, \mathrm{dir})$$

### 📖 Definicja Zanurzenia Klasycznej Hiperkrawędzi ($\Phi_{\mathrm{hyper}}$)

- Klasyczna nieskierowana hiperkrawędź Berge’a (worek/zbiór wierzchołków) jest po prostu szczególnym przypadkiem hiperkonektora, który posiada jedną strefę (zawierającą ten zbiór wierzchołków) oraz brak jakichkolwiek wewnętrznych przepływów.

- Dowolny klasyczny hipergraf Berge’a $\mathcal{H} = (V, E)$, gdzie $e \subseteq V$ dla każdego $e \in E$, jest reprezentowalny w meta-modelu poprzez kanoniczne zanurzenie iniekcyjne:
   
   $$\Phi_{\mathrm{hyper}}: V \sqcup E \hookrightarrow U, \quad \mathrm{gdzie} \quad \Phi_{\mathrm{hyper}}(V) \subseteq N \quad \mathrm{oraz} \quad \Phi_{\mathrm{hyper}}(E) \subseteq H$$
   
   Dla każdej hiperkrawędzi $e \in E$ niech $h_e = \Phi_{\mathrm{hyper}}(e) \in H$. Hiperkonektor $h_e$ posiada strukturę $\sigma(h_e) = (\{z_e\}, \emptyset)$ (brak przepływów, $\vert{}F_{h_e}\vert{} = 0$) i spełnia warunek:
   
   $$\mu_Z(z_e) = \{\Phi_{\mathrm{hyper}}(v) \mid v \in e\}$$

### 📖 Definicja Zanurzenia Klasycznej Skierowanej Hiperkrawędzi ($\Phi_{\mathrm{dir\_hyper}}$)

- Klasyczna skierowana hiperkrawędź (łącząca zbiór wejściowy ze zbiorem wyjściowym) jest szczególnym przypadkiem hiperkonektora, który posiada dwie odrębne strefy (zawierające odpowiednio wierzchołki źródłowe i docelowe) oraz jeden przepływ skierowany między tymi strefami.

- Dowolny klasyczny skierowany hipergraf $\mathcal{H}_{\mathrm{dir}} = (V, E_{\mathrm{dir}})$, gdzie każda krawędź jest parą zbiorów $e = (T_e, H_e)$ dla $T_e, H_e \subseteq V$ (zbiór wejściowy Tail i wyjściowy Head), jest reprezentowalny w meta-modelu poprzez kanoniczne zanurzenie iniekcyjne:
   
   $$\Phi_{\mathrm{dir\_hyper}}: V \sqcup E_{\mathrm{dir}} \hookrightarrow U, \quad \mathrm{gdzie} \quad \Phi_{\mathrm{dir\_hyper}}(V) \subseteq N \quad \mathrm{oraz} \quad \Phi_{\mathrm{dir\_hyper}}(E_{\mathrm{dir}}) \subseteq H$$
   
   Dla każdej skierowanej hiperkrawędzi $e = (T_e, H_e) \in E_{\mathrm{dir}}$ niech $h_e = \Phi_{\mathrm{dir\_hyper}}(e) \in H$. Hiperkonektor $h_e$ posiada strukturę $\sigma(h_e) = (\{z_e^{\mathrm{tail}}, z_e^{\mathrm{head}}\}, \{f_e\})$, gdzie $z_e^{\mathrm{tail}} \neq z_e^{\mathrm{head}}$, i spełnia warunki:
   
   $$\mu_Z(z_e^{\mathrm{tail}}) = \{\Phi_{\mathrm{dir\_hyper}}(v) \mid v \in T_e\}, \quad \mu_Z(z_e^{\mathrm{head}}) = \{\Phi_{\mathrm{dir\_hyper}}(v) \mid v \in H_e\}$$
   
   $$\pi_F(f_e) = (z_e^{\mathrm{tail}}, z_e^{\mathrm{head}}, \mathrm{dir})$$

---
---

🫟

```

### [4] `./README.md`

```markdown
# TISDB (Topologically Integrated System Database)

[![Crates.io](https://img.shields.io/crates/v/tisdb.svg)](https://crates.io/crates/tisdb)
[![Documentation](https://docs.rs/tisdb/badge.svg)](https://docs.rs/tisdb)
[![GitHub](https://img.shields.io/badge/GitHub-tisdb-181717?logo=github)](https://github.com/tisdb/tisdb)

TISDB is a hypergraph database engine tailored for TISOS (Topologically Integrated System of Semantic Overlays). It features strict axiom enforcement, atomic I/O, and Redb-based storage.

## O projekcie

TISDB to dedykowany silnik bazy danych grafowych budowany na potrzeby modelu ontologicznego TISOS. Silnik odpowiada za rygorystyczne przestrzeganie aksjomatów strukturalnych (Guardrails) dla Węzłów (Nodes), Hiperkonektorów (Hyperconnectors), Stref (Zones) oraz Przepływów (Flows).

*Uwaga: Projekt jest we wczesnej fazie rozwoju (dev-preview) i jego API może ulegać drastycznym zmianom.*

## Architektura

Więcej informacji na temat decyzji projektowych, modelu pamięci (Zero-Copy mmap) oraz silnika transakcyjnego znajduje się w pliku [ARCHITECTURE.md](ARCHITECTURE.md).

## Model

Szczegółowy opis struktury grafu, definicje encji (Węzły, Hiperkonektory, Strefy, Przepływy) oraz aksjomaty relacyjne znajdują się w pliku [MODEL.rs](MODEL.rs).

## Licencja

Projekt dystrybuowany na podwójnej licencji: MIT oraz Apache-2.0.

---
---

🫟

```

### [5] `./src/domain/flow.rs`

```rust
use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive,
    Serialize,
    Deserialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
#[repr(u8)]
pub enum FlowType {
    Sym = 0,   // Nieskierowane (statyczne)
    Dir = 1,   // Skierowane
    Bidir = 2, // Dwukierunkowe (aktywne)
}

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Flow {
    id: FlowId,
    parent_hyper: HyperconnectorId,
    source_zone: ZoneId,
    target_zone: ZoneId,
    flow_type: FlowType,
    pub header: EntityHeader,
}

impl Flow {
    pub fn new(
        parent_hyper: HyperconnectorId,
        source_zone: ZoneId,
        target_zone: ZoneId,
        flow_type: FlowType,
        class_path: Vec<String>,
    ) -> Self {
        Self {
            id: FlowId::new(),
            parent_hyper,
            source_zone,
            target_zone,
            flow_type,
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> FlowId {
        self.id
    }
    #[inline]
    pub fn parent_hyper(&self) -> HyperconnectorId {
        self.parent_hyper
    }
    #[inline]
    pub fn source_zone(&self) -> ZoneId {
        self.source_zone
    }
    #[inline]
    pub fn target_zone(&self) -> ZoneId {
        self.target_zone
    }
    #[inline]
    pub fn flow_type(&self) -> FlowType {
        self.flow_type
    }
}
```

### [6] `./src/domain/hyper.rs`

```rust
// src/domain/hyper.rs
use crate::domain::id::{FlowId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Hyperconnector {
    id: HyperconnectorId,
    zones: Vec<ZoneId>,
    flows: Vec<FlowId>,
    pub header: EntityHeader,
}

impl Hyperconnector {
    pub fn new(class_path: Vec<String>) -> Self {
        Self {
            id: HyperconnectorId::new(),
            zones: Vec::new(),
            flows: Vec::new(),
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> HyperconnectorId {
        self.id
    }

    #[inline]
    pub fn zones(&self) -> &[ZoneId] {
        &self.zones
    }

    #[inline]
    pub fn flows(&self) -> &[FlowId] {
        &self.flows
    }

    pub fn add_zone(&mut self, zone_id: ZoneId) {
        if !self.zones.contains(&zone_id) {
            self.zones.push(zone_id);
            // Uwaga: Zmiana topologii niekoniecznie oznacza zmianę semantyki (atrybutów) 
            // samego H, ale dla OCC można by tutaj wywoływać bump_revision. Na razie pomijamy.
        }
    }

    pub fn add_flow(&mut self, flow_id: FlowId) {
        if !self.flows.contains(&flow_id) {
            self.flows.push(flow_id);
        }
    }

    pub fn remove_zone(&mut self, zone_id: &ZoneId) -> bool {
        if let Some(pos) = self.zones.iter().position(|z| z == zone_id) {
            self.zones.swap_remove(pos);
            true
        } else {
            false
        }
    }

    pub fn remove_flow(&mut self, flow_id: &FlowId) -> bool {
        if let Some(pos) = self.flows.iter().position(|f| f == flow_id) {
            self.flows.swap_remove(pos);
            true
        } else {
            false
        }
    }
}
```

### [7] `./src/domain/id.rs`

```rust
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};
use ulid::Ulid;

macro_rules! define_id {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(
            Archive,
            Serialize,
            Deserialize,
            SerdeSerialize,
            SerdeDeserialize,
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
        )]
        #[rkyv(derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord))]
        pub struct $name([u8; 16]);

        impl $name {
    #[inline]
    pub fn new() -> Self {
        Self(Ulid::generate().to_bytes())
    }

            #[inline]
            pub fn from_bytes(bytes: [u8; 16]) -> Self {
                Self(bytes)
            }

            #[inline]
            pub fn as_bytes(&self) -> &[u8; 16] {
                &self.0
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", Ulid::from(self.0))
            }
        }
    };
}

define_id!(NodeId, "Unikalne ID atomowego Węzła (N)");
define_id!(HyperconnectorId, "Unikalne ID Hiperkonektora (H)");
define_id!(ZoneId, "Unikalne ID Strefy / Portu (Z)");
define_id!(FlowId, "Unikalne ID Przepływu (F)");

/// Uniwersum Obiektów U = N ⊔ H (Aksjomat 2)
#[derive(
    Archive,
    Serialize,
    Deserialize,
    SerdeSerialize,
    SerdeDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum EntityId {
    Node(NodeId),
    Hyperconnector(HyperconnectorId),
}
```

### [8] `./src/domain/metadata.rs`

```rust
use std::collections::HashMap;

use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

/// Definiuje format tekstu dla zawartości RichText.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Eq, Hash,
)]
#[rkyv(derive(Debug, PartialEq, Eq, Hash))]
pub enum TextFormat {
    Plain,
    Markdown,
    Html,
    Json,
    Xml,
}

/// Skalarny typ pomocniczy dla struktur złożonych (zbiory, przedziały), 
/// zapobiegający rekurencji nieskończonej.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub enum ScalarValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

/// Główny, w pełni elastyczny typ atrybutów dla grafu.
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub enum AttributeValue {
    // --- SKALARNE ---
    ShortText(String),
    LongText(String),
    RichText { format: TextFormat, content: String },
    Integer(i64),
    Float(f64),
    Boolean(bool),

    // --- ZŁOŻONE ---
    /// Zbiór wartości skalarnych (np. Tagi).
    Set(Vec<ScalarValue>),
    /// Uporządkowana lista/sekwencja.
    Sequence(Vec<ScalarValue>),
    /// Zakres/Przedział min-max.
    Range { min: ScalarValue, max: ScalarValue },
}

/// Uniwersalny nagłówek (Entity Header) przypinany do każdego obiektu w systemie (N, H, Z, F).
#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct EntityHeader {
    // 1. Właściwości Systemowe (Audyt)
    created_at: u64,
    updated_at: u64,
    revision: u32,

    // 2. Ontologia (Ścieżka klasyfikacji, np. ["Osoba", "Kobieta"])
    class_path: Vec<String>,

    // 3. Właściwości Dynamiczne - HashMap (rkyv obsługuje to doskonale, a dla nas O(1))
    attributes: HashMap<String, AttributeValue>,
}

impl EntityHeader {
    pub fn new(class_path: Vec<String>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            created_at: now,
            updated_at: now,
            revision: 1,
            class_path,
            attributes: HashMap::new(),
        }
    }

    // --- Systemowe gettery ---
    pub fn created_at(&self) -> u64 { self.created_at }
    pub fn updated_at(&self) -> u64 { self.updated_at }
    pub fn revision(&self) -> u32 { self.revision }
    pub fn class_path(&self) -> &[String] { &self.class_path }

    // --- Modulatory atrybutów ---
    pub fn set_attribute(&mut self, key: impl Into<String>, value: AttributeValue) {
        self.attributes.insert(key.into(), value);
        self.bump_revision();
    }

    pub fn get_attribute(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    pub fn remove_attribute(&mut self, key: &str) -> Option<AttributeValue> {
        let removed = self.attributes.remove(key);
        if removed.is_some() {
            self.bump_revision();
        }
        removed
    }

    // --- Prywatne podbicie wersji ---
    fn bump_revision(&mut self) {
        self.revision += 1;
        self.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
    }
}
```

### [9] `./src/domain/node.rs`

```rust
// src/domain/node.rs
use crate::domain::id::NodeId;
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Node {
    id: NodeId,
    pub header: EntityHeader,
}

impl Node {
    pub fn new(class_path: Vec<String>) -> Self {
        Self { 
            id: NodeId::new(),
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> NodeId {
        self.id
    }
}
```

### [10] `./src/domain/zone.rs`

```rust
use crate::domain::id::{EntityId, HyperconnectorId, ZoneId};
use crate::domain::metadata::EntityHeader;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

#[derive(
    Archive, Serialize, Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq,
)]
#[rkyv(derive(Debug, PartialEq))]
pub struct Zone {
    id: ZoneId,
    parent_hyper: HyperconnectorId,
    contained_entities: Vec<EntityId>,
    pub header: EntityHeader,
}

impl Zone {
    pub fn new(parent_hyper: HyperconnectorId, class_path: Vec<String>) -> Self {
        Self {
            id: ZoneId::new(),
            parent_hyper,
            contained_entities: Vec::new(),
            header: EntityHeader::new(class_path),
        }
    }

    #[inline]
    pub fn id(&self) -> ZoneId {
        self.id
    }

    #[inline]
    pub fn parent_hyper(&self) -> HyperconnectorId {
        self.parent_hyper
    }

    #[inline]
    pub fn contained_entities(&self) -> &[EntityId] {
        &self.contained_entities
    }

    pub fn add_entity(&mut self, entity: EntityId) {
        if !self.contained_entities.contains(&entity) {
            self.contained_entities.push(entity);
        }
    }

    pub fn remove_entity(&mut self, entity: &EntityId) -> bool {
        if let Some(pos) = self.contained_entities.iter().position(|e| e == entity) {
            self.contained_entities.swap_remove(pos);
            true
        } else {
            false
        }
    }
}
```

### [11] `./src/engine.rs`

```rust
// src/engine.rs
use std::collections::{HashMap, HashSet};

use crate::domain::{
    EntityId, Flow, FlowId, FlowType, Hyperconnector, HyperconnectorId, Node, NodeId, Zone, ZoneId,
};
use crate::error::CoreError;
use crate::storage::StorageBackend;
use rkyv::util::AlignedVec;

pub struct CisowskiEngine<S: StorageBackend> {
    storage: S,
    nodes: HashMap<NodeId, Node>,
    hypers: HashMap<HyperconnectorId, Hyperconnector>,
    zones: HashMap<ZoneId, Zone>,
    flows: HashMap<FlowId, Flow>,
}

impl<S: StorageBackend> CisowskiEngine<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            nodes: HashMap::new(),
            hypers: HashMap::new(),
            zones: HashMap::new(),
            flows: HashMap::new(),
        }
    }

    // --- AKSIOMAT 1: Tworzenie Atomowych Węzłów (N) ---
    pub fn create_node(&mut self, class_path: Vec<String>) -> NodeId {
        let node = Node::new(class_path);
        let id = node.id();
        self.nodes.insert(id, node);
        id
    }

    // --- AKSIOMAT 2 & 7: Tworzenie Hiperkonektora (H) ---
    pub fn create_hyperconnector(&mut self, class_path: Vec<String>) -> HyperconnectorId {
        let hyper = Hyperconnector::new(class_path);
        let id = hyper.id();
        self.hypers.insert(id, hyper);
        id
    }

    // --- AKSIOMAT 3 & 4: Tworzenie Strefy wewnątrz konkretnego Hiperkonektora ---
    pub fn create_zone(&mut self, parent_h: HyperconnectorId, class_path: Vec<String>) -> Result<ZoneId, CoreError> {
        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;

        let zone = Zone::new(parent_h, class_path);
        let zone_id = zone.id();

        self.zones.insert(zone_id, zone);
        hyper.add_zone(zone_id);

        Ok(zone_id)
    }

    // --- AKSIOMAT 8 & 9: Zawartość Strefy μZ oraz Weryfikacja Acykliczności Zagnieżdżenia ---
    pub fn add_entity_to_zone(
        &mut self,
        zone_id: ZoneId,
        entity: EntityId,
    ) -> Result<(), CoreError> {
        match entity {
            EntityId::Node(nid) => {
                if !self.nodes.contains_key(&nid) {
                    return Err(CoreError::NotFound(nid.to_string()));
                }
            }
            EntityId::Hyperconnector(hid) => {
                if !self.hypers.contains_key(&hid) {
                    return Err(CoreError::NotFound(hid.to_string()));
                }

                let target_zone = self
                    .zones
                    .get(&zone_id)
                    .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

                if self.is_transitively_contained(hid, target_zone.parent_hyper()) {
                    return Err(CoreError::Axiom9CycleDetected);
                }
            }
        }

        let zone = self
            .zones
            .get_mut(&zone_id)
            .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        zone.add_entity(entity);
        Ok(())
    }

    pub fn remove_entity_from_zone(
        &mut self,
        zone_id: &ZoneId,
        entity: &EntityId,
    ) -> Result<(), CoreError> {
        let zone = self
            .zones
            .get_mut(zone_id)
            .ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        if !zone.remove_entity(entity) {
            return Err(CoreError::NotFound(format!(
                "Encja {:?} nie znajduje się w strefie {}",
                entity, zone_id
            )));
        }
        Ok(())
    }

    // --- AKSIOMAT 5, 6 & 10: Tworzenie Przepływu (F) ---
    pub fn create_flow(
        &mut self,
        parent_h: HyperconnectorId,
        source_z: ZoneId,
        target_z: ZoneId,
        flow_type: FlowType,
        class_path: Vec<String>,
    ) -> Result<FlowId, CoreError> {
        let src = self
            .zones
            .get(&source_z)
            .ok_or_else(|| CoreError::NotFound(source_z.to_string()))?;
        let tgt = self
            .zones
            .get(&target_z)
            .ok_or_else(|| CoreError::NotFound(target_z.to_string()))?;

        if src.parent_hyper() != parent_h || tgt.parent_hyper() != parent_h {
            return Err(CoreError::InvalidFlowBoundary);
        }

        let flow = Flow::new(parent_h, source_z, target_z, flow_type, class_path);
        let flow_id = flow.id();

        self.flows.insert(flow_id, flow);

        let hyper = self
            .hypers
            .get_mut(&parent_h)
            .ok_or_else(|| CoreError::NotFound(parent_h.to_string()))?;
        hyper.add_flow(flow_id);

        Ok(flow_id)
    }

    // ========================================================================
    // ŚCISŁE USUWANIE (RESTRICT / GUARDRAILS)
    // ========================================================================

    pub fn delete_node(&mut self, node_id: &NodeId) -> Result<(), CoreError> {
        let entity = EntityId::Node(*node_id);
        
        let usage_reason = self.zones.values().find_map(|zone| {
            if zone.contained_entities().contains(&entity) {
                Some(format!("Węzeł należy do Strefy {}", zone.id()))
            } else {
                None
            }
        });

        if let Some(reason) = usage_reason {
            return Err(CoreError::InUse(node_id.to_string(), reason));
        }

        self.nodes.remove(node_id).ok_or_else(|| CoreError::NotFound(node_id.to_string()))?;
        Ok(())
    }

    pub fn delete_zone(&mut self, zone_id: &ZoneId) -> Result<(), CoreError> {
        let zone = self.zones.get(zone_id).ok_or_else(|| CoreError::NotFound(zone_id.to_string()))?;

        if !zone.contained_entities().is_empty() {
            return Err(CoreError::InUse(
                zone_id.to_string(),
                "Strefa zawiera przypisane encje (najpierw odepnij zawartość)".into(),
            ));
        }

        let has_flows = self.flows.values().any(|f| f.source_zone() == *zone_id || f.target_zone() == *zone_id);
        if has_flows {
             return Err(CoreError::InUse(
                zone_id.to_string(),
                "Strefa posiada aktywne przepływy (najpierw usuń przepływy)".into(),
            ));
        }

        if let Some(hyper) = self.hypers.get_mut(&zone.parent_hyper()) {
            hyper.remove_zone(zone_id);
        }

        self.zones.remove(zone_id);
        Ok(())
    }

    pub fn delete_flow(&mut self, flow_id: &FlowId) -> Result<(), CoreError> {
        let flow = self
            .flows
            .remove(flow_id)
            .ok_or_else(|| CoreError::NotFound(flow_id.to_string()))?;

        if let Some(hyper) = self.hypers.get_mut(&flow.parent_hyper()) {
            hyper.remove_flow(flow_id);
        }

        Ok(())
    }

    pub fn delete_hyperconnector(&mut self, hyper_id: &HyperconnectorId) -> Result<(), CoreError> {
        let hyper = self.hypers.get(hyper_id).ok_or_else(|| CoreError::NotFound(hyper_id.to_string()))?;

        let entity = EntityId::Hyperconnector(*hyper_id);
        let nested_usage = self.zones.values().find_map(|z| {
             if z.contained_entities().contains(&entity) {
                 Some(format!("Hiperkonektor jest zagnieżdżony w Strefie {}", z.id()))
             } else {
                 None
             }
        });
        if let Some(reason) = nested_usage {
             return Err(CoreError::InUse(hyper_id.to_string(), reason));
        }

        for zone_id in hyper.zones() {
            if let Some(zone) = self.zones.get(zone_id) {
                if !zone.contained_entities().is_empty() {
                    return Err(CoreError::InUse(
                        hyper_id.to_string(),
                        format!("Hiperkonektor zawiera zajętą Strefę {}", zone_id),
                    ));
                }
            }
        }

        let zones_to_remove = hyper.zones().to_vec();
        for zone_id in zones_to_remove {
             self.zones.remove(&zone_id);
        }
        
        let flows_to_remove = hyper.flows().to_vec();
        for flow_id in flows_to_remove {
             self.flows.remove(&flow_id);
        }

        self.hypers.remove(hyper_id);
        Ok(())
    }

    fn is_transitively_contained(
        &self,
        ancestor: HyperconnectorId,
        target: HyperconnectorId,
    ) -> bool {
        if ancestor == target {
            return true;
        }

        let mut visited = HashSet::new();
        let mut stack = vec![ancestor];

        while let Some(current_h_id) = stack.pop() {
            if current_h_id == target {
                return true;
            }

            if !visited.insert(current_h_id) {
                continue;
            }

            if let Some(current_h) = self.hypers.get(&current_h_id) {
                for &z_id in current_h.zones() {
                    if let Some(zone) = self.zones.get(&z_id) {
                        for entity in zone.contained_entities() {
                            if let EntityId::Hyperconnector(child_h_id) = entity {
                                stack.push(*child_h_id);
                            }
                        }
                    }
                }
            }
        }

        false
    }

    pub fn get_node(&self, id: &NodeId) -> Option<&Node> { self.nodes.get(id) }
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node> { self.nodes.get_mut(id) }

    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Option<&Hyperconnector> { self.hypers.get(id) }
    pub fn get_hyperconnector_mut(&mut self, id: &HyperconnectorId) -> Option<&mut Hyperconnector> { self.hypers.get_mut(id) }

    pub fn get_zone(&self, id: &ZoneId) -> Option<&Zone> { self.zones.get(id) }
    pub fn get_zone_mut(&mut self, id: &ZoneId) -> Option<&mut Zone> { self.zones.get_mut(id) }

    pub fn get_flow(&self, id: &FlowId) -> Option<&Flow> { self.flows.get(id) }
    pub fn get_flow_mut(&mut self, id: &FlowId) -> Option<&mut Flow> { self.flows.get_mut(id) }
}

use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Serialize, Deserialize, Debug)]
#[rkyv(derive(Debug))]
pub struct DatabaseSnapshot {
    pub nodes: Vec<Node>,
    pub hypers: Vec<Hyperconnector>,
    pub zones: Vec<Zone>,
    pub flows: Vec<Flow>,
}

impl<S: StorageBackend> CisowskiEngine<S> {
    pub fn save(&mut self) -> Result<(), CoreError> {
        let snapshot = DatabaseSnapshot {
            nodes: self.nodes.values().cloned().collect(),
            hypers: self.hypers.values().cloned().collect(),
            zones: self.zones.values().cloned().collect(),
            flows: self.flows.values().cloned().collect(),
        };

        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&snapshot)
            .map_err(|e| CoreError::SerializationError(format!("Write error: {}", e)))?;

        let mut payload = Vec::with_capacity(4 + bytes.len());
        payload.extend_from_slice(b"CDB1");
        payload.extend_from_slice(&bytes);
        
        // Zastąpiliśmy stare write_bytes i truncate jednym, bezpiecznym poleceniem
        self.storage.atomic_write_snapshot(&payload)?;
        
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), CoreError> {
        let len = self.storage.len();
        if len == 0 {
            return Ok(());
        }
        
        if len < 4 {
             return Err(CoreError::StorageError(
                "Nieprawidłowy format pliku bazy (ucięty nagłówek)".into(),
            ));
        }

        let file_bytes = self.storage.read_bytes(0, len as usize)?;

        if &file_bytes[0..4] != b"CDB1" {
            return Err(CoreError::StorageError(
                "Nieprawidłowy format pliku bazy (brak magika CDB1)".into(),
            ));
        }

        let mut archive_bytes = AlignedVec::<16>::new();
        archive_bytes.extend_from_slice(&file_bytes[4..]);
        
        let archived = rkyv::access::<ArchivedDatabaseSnapshot, rkyv::rancor::Error>(archive_bytes.as_slice())
            .map_err(|e| CoreError::SerializationError(format!("Validation/Access error: {}", e)))?;

        let snapshot: DatabaseSnapshot = rkyv::deserialize::<
            DatabaseSnapshot,
            rkyv::rancor::Error,
        >(archived)
        .map_err(|e| CoreError::SerializationError(format!("Deserialize error: {}", e)))?;

        self.nodes = snapshot.nodes.into_iter().map(|n| (n.id(), n)).collect();
        self.hypers = snapshot.hypers.into_iter().map(|h| (h.id(), h)).collect();
        self.zones = snapshot.zones.into_iter().map(|z| (z.id(), z)).collect();
        self.flows = snapshot.flows.into_iter().map(|f| (f.id(), f)).collect();

        Ok(())
    }
}
```

### [12] `./src/error.rs`

```rust
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum CoreError {
    #[error("Obiekt o podanym ID nie istnieje: {0}")]
    NotFound(String),

    #[error("Naruszenie Aksjomatu 3: Strefa należy już do innego hiperkonektora")]
    Axiom3Violation,

    #[error("Naruszenie Aksjomatu 5: Przepływ należy już do innego hiperkonektora")]
    Axiom5Violation,

    #[error("Naruszenie Aksjomatu 9: Wykryto cykl zagnieżdżenia lub cykl cykliczny w relacji R_contain")]
    Axiom9CycleDetected,

    #[error("Naruszenie Aksjomatu 10: Przepływ próbuje łączyć strefy z różnych hiperkonektorów")]
    InvalidFlowBoundary,

    #[error("Błąd warstwy pamięci masowej (IO/Storage): {0}")]
    StorageError(String),

    // --- ZMIANA: Dodano pole (String) aby przenosić komunikat błędu rancor ---
    #[error("Błąd binarnej serializacji/deserializacji (rkyv): {0}")]
    SerializationError(String),

    // --- DODANE BŁĘDY RESTRICT/GUARDRAILS ---
    #[error("Obiekt {0} jest używany i nie może zostać usunięty: {1}")]
    InUse(String, String),
}
```

### [13] `./src/lib.rs`

```rust
pub mod error;

pub mod domain {
    pub mod flow;
    pub mod hyper;
    pub mod id;
    pub mod metadata;
    pub mod node;
    pub mod zone;

    pub use flow::{Flow, FlowType};
    pub use hyper::Hyperconnector;
    pub use id::{EntityId, FlowId, HyperconnectorId, NodeId, ZoneId};
    pub use metadata::{AttributeValue, EntityHeader, ScalarValue, TextFormat};
    pub use node::Node;
    pub use zone::Zone;
}

pub mod storage {
    pub mod backend;
    pub mod file;
    pub mod memory;
	pub mod redb_backend;

    pub use backend::StorageBackend;
    pub use file::FileStorage;
    pub use memory::MemoryStorage;
	pub use redb_backend::{RedbStorage, RedbWriteTxn, RedbReadTxn};
}

pub mod engine;

// Re-eksporty dla wygody używania crate'a na zewnątrz
pub use domain::*;
pub use engine::CisowskiEngine;
pub use error::CoreError;
pub use storage::*;
```

### [14] `./src/storage/backend.rs`

```rust
// src/storage/backend.rs
use crate::error::CoreError;

pub trait StorageBackend {
    /// Czyta bajty z trwałego nośnika
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError>;
    
    /// Wykonuje w pełni atomowy zapis całego zrzutu pamięci.
    /// Jeśli operacja się nie powiedzie, stary stan bazy pozostaje nietknięty.
    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError>;
    
    /// Zwraca wielkość obecnego zrzutu
    fn len(&self) -> u64;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
```

### [15] `./src/storage/file.rs`

```rust
// src/storage/file.rs
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use fslock::LockFile;

use crate::error::CoreError;
use crate::storage::backend::StorageBackend;

pub struct FileStorage {
    path: PathBuf,
    file: Option<File>, // Opcjonalny, aby można było go bezpiecznie zamknąć przy podmianie pliku na Windowsie
    _lock: LockFile,
}

impl FileStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let db_path = path.as_ref().to_path_buf();
        let lock_path = db_path.with_extension("cdb.lock");
        
        let mut lock = LockFile::open(&lock_path)
            .map_err(|e| CoreError::StorageError(format!("Nie można utworzyć pliku blokady: {e}")))?;

        if !lock
            .try_lock()
            .map_err(|e| CoreError::StorageError(format!("Błąd prób blokady pliku: {e}")))?
        {
            return Err(CoreError::StorageError(
                "Plik bazy jest używany przez inny proces".into(),
            ));
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&db_path)
            .map_err(|e| CoreError::StorageError(format!("Błąd otwarcia pliku bazy: {e}")))?;

        Ok(Self { 
            path: db_path, 
            file: Some(file), 
            _lock: lock 
        })
    }
}

impl StorageBackend for FileStorage {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError> {
        let mut file = self.file.as_ref()
            .ok_or_else(|| CoreError::StorageError("Uchwyt pliku jest zamknięty".into()))?;
            
        file.seek(SeekFrom::Start(offset))
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        let mut buffer = vec![0u8; len];
        file.read_exact(&mut buffer)
            .map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(buffer)
    }

    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError> {
        let temp_path = self.path.with_extension("cdb.tmp");

        // 1. Otwórz tymczasowy plik i zapisz dane
        let mut temp_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&temp_path)
            .map_err(|e| CoreError::StorageError(format!("Błąd tworzenia pliku tmp: {e}")))?;

        temp_file.write_all(payload)
            .map_err(|e| CoreError::StorageError(format!("Błąd zapisu do pliku tmp: {e}")))?;
            
        // 2. Wymuś zrzut z buforów OS fizycznie na dysk
        temp_file.sync_all()
            .map_err(|e| CoreError::StorageError(format!("Błąd synchronizacji I/O dysku: {e}")))?;

        // 3. Zamknij stary uchwyt (niezbędne na Windowsie przed zrobieniem atomowego rename)
        self.file = None;

        // 4. Atomowa podmiana pliku (na platformach POSIX i nowoczesnym NTFS)
        std::fs::rename(&temp_path, &self.path)
            .map_err(|e| CoreError::StorageError(format!("Błąd atomowej podmiany pliku bazy: {e}")))?;

        // 5. Otwórz nowy plik i przypisz do struktury aby przywrócić możliwość czytania
        let new_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| CoreError::StorageError(format!("Błąd ponownego otwarcia zaktualizowanego pliku: {e}")))?;

        self.file = Some(new_file);

        Ok(())
    }

    fn len(&self) -> u64 {
        if let Some(f) = &self.file {
            f.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        }
    }
}
```

### [16] `./src/storage/memory.rs`

```rust
// src/storage/memory.rs
use crate::error::CoreError;
use crate::storage::backend::StorageBackend;

#[derive(Debug, Default, Clone)]
pub struct MemoryStorage {
    data: Vec<u8>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { data: bytes }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl StorageBackend for MemoryStorage {
    fn read_bytes(&self, offset: u64, len: usize) -> Result<Vec<u8>, CoreError> {
        let start = offset as usize;
        let end = start + len;
        if end > self.data.len() {
            return Err(CoreError::StorageError("Odczyt poza zakresem pamięci".into()));
        }
        Ok(self.data[start..end].to_vec())
    }

    fn atomic_write_snapshot(&mut self, payload: &[u8]) -> Result<(), CoreError> {
        // W pamięci RAM "atomowa" podmiana to po prostu nadpisanie wektora sklonowanymi bajtami
        self.data = payload.to_vec();
        Ok(())
    }

    fn len(&self) -> u64 {
        self.data.len() as u64
    }
}
```

### [17] `./src/storage/redb_backend.rs`

```rust
use std::path::Path;

use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition, ReadTransaction, WriteTransaction};
use rkyv::{rancor::Error as RancorError, Archive};

use crate::domain::{
    Flow, FlowId, Hyperconnector, HyperconnectorId, Node, NodeId, Zone, ZoneId
};
use crate::error::CoreError;

// 1. Definicje fizycznych tabel wewnątrz pliku B-Tree
// Zastosowanie tablic [u8; 16] dla kluczy gwarantuje doskonałą wydajność indeksów redb (stała długość)
const NODES_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("nodes");
const HYPERS_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("hypers");
const ZONES_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("zones");
const FLOWS_TABLE: TableDefinition<[u8; 16], &[u8]> = TableDefinition::new("flows");
const META_TABLE: TableDefinition<&str, u32> = TableDefinition::new("metadata");

pub struct RedbStorage {
    db: Database,
}

impl RedbStorage {
    /// Otwiera bazę i natychmiast inicjalizuje schemat, co chroni przed błędem "TableDoesNotExist"
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, CoreError> {
        let db = Database::create(path)
            .map_err(|e| CoreError::StorageError(format!("Błąd otwarcia bazy redb: {}", e)))?;
        
        let write_txn = db.begin_write().map_err(|e| CoreError::StorageError(e.to_string()))?;
        {
            let _ = write_txn.open_table(NODES_TABLE).unwrap();
            let _ = write_txn.open_table(HYPERS_TABLE).unwrap();
            let _ = write_txn.open_table(ZONES_TABLE).unwrap();
            let _ = write_txn.open_table(FLOWS_TABLE).unwrap();
            
            let mut meta = write_txn.open_table(META_TABLE).unwrap();
            if meta.get("format_version").unwrap().is_none() {
                meta.insert("format_version", &2).unwrap(); // Zgodnie z formatem v2 (unaligned)
            }
        }
        write_txn.commit().map_err(|e| CoreError::StorageError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Otwiera izolowaną transakcję zapisu (blokuje innych pisarzy)
    pub fn begin_write(&self) -> Result<RedbWriteTxn, CoreError> {
        let txn = self.db.begin_write().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(RedbWriteTxn { txn })
    }

    /// Otwiera bez-blokadową transakcję odczytu ze zrzutu MVCC bazy (Snapshot Isolation)
    pub fn begin_read(&self) -> Result<RedbReadTxn, CoreError> {
        let txn = self.db.begin_read().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(RedbReadTxn { txn })
    }
}

// =====================================================================
// TRANSAKCJA ODCZYTU (READ-ONLY)
// =====================================================================
pub struct RedbReadTxn {
    txn: ReadTransaction,
}

impl RedbReadTxn {
    pub fn get_node(&self, id: &NodeId) -> Result<Option<Node>, CoreError> {
        let table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Node as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Node, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Result<Option<Hyperconnector>, CoreError> {
        let table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Hyperconnector as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Hyperconnector, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_zone(&self, id: &ZoneId) -> Result<Option<Zone>, CoreError> {
        let table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Zone as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Zone, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn get_flow(&self, id: &FlowId) -> Result<Option<Flow>, CoreError> {
        let table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Flow as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Flow, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }
}

// =====================================================================
// TRANSAKCJA ZAPISU (READ-WRITE)
// =====================================================================
pub struct RedbWriteTxn {
    txn: WriteTransaction,
}

impl RedbWriteTxn {
    // --- NODE ---
    pub fn get_node(&self, id: &NodeId) -> Result<Option<Node>, CoreError> {
        let table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Node as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Node, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_node(&self, node: &Node) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(node).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*node.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    pub fn delete_node(&self, id: &NodeId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(NODES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- HYPERCONNECTOR ---
    pub fn get_hyperconnector(&self, id: &HyperconnectorId) -> Result<Option<Hyperconnector>, CoreError> {
        let table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Hyperconnector as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Hyperconnector, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_hyperconnector(&self, hyper: &Hyperconnector) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(hyper).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*hyper.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    pub fn delete_hyperconnector(&self, id: &HyperconnectorId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(HYPERS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- ZONE ---
    pub fn get_zone(&self, id: &ZoneId) -> Result<Option<Zone>, CoreError> {
        let table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Zone as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Zone, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_zone(&self, zone: &Zone) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(zone).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*zone.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    pub fn delete_zone(&self, id: &ZoneId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(ZONES_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- FLOW ---
    pub fn get_flow(&self, id: &FlowId) -> Result<Option<Flow>, CoreError> {
        let table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        if let Some(guard) = table.get(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))? {
            let archived = rkyv::access::<<Flow as Archive>::Archived, RancorError>(guard.value())
                .map_err(|e| CoreError::SerializationError(format!("Błąd dostępu mmap: {}", e)))?;
            let entity = rkyv::deserialize::<Flow, RancorError>(archived)
                .map_err(|e| CoreError::SerializationError(format!("Błąd deserializacji: {}", e)))?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    pub fn put_flow(&self, flow: &Flow) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        let bytes = rkyv::to_bytes::<RancorError>(flow).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        table.insert(*flow.id().as_bytes(), bytes.as_slice()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    pub fn delete_flow(&self, id: &FlowId) -> Result<(), CoreError> {
        let mut table = self.txn.open_table(FLOWS_TABLE).map_err(|e| CoreError::StorageError(e.to_string()))?;
        table.remove(*id.as_bytes()).map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }

    // --- Zatwierdzenie (ACID Commit) ---
    pub fn commit(self) -> Result<(), CoreError> {
        self.txn.commit().map_err(|e| CoreError::StorageError(e.to_string()))?;
        Ok(())
    }
}
```

### [18] `./tests/axiom_tests.rs`

```rust
use tisdb::{CisowskiEngine, CoreError, EntityId, FlowType, MemoryStorage};

#[test]
fn test_node_and_hyperconnector_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let node_id = engine.create_node(vec!["TestNode".into()]);
    let hyper_id = engine.create_hyperconnector(vec!["TestHyper".into()]);

    assert!(engine.get_node(&node_id).is_some());
    assert!(engine.get_hyperconnector(&hyper_id).is_some());
}

#[test]
fn test_axiom_3_and_4_zone_creation() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let hyper_id = engine.create_hyperconnector(vec![]);
    let zone_id = engine.create_zone(hyper_id, vec![]).unwrap();

    let zone = engine.get_zone(&zone_id).unwrap();
    assert_eq!(zone.parent_hyper(), hyper_id);
}

#[test]
fn test_axiom_9_cycle_prevention() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    engine
        .add_entity_to_zone(z1, EntityId::Hyperconnector(h2))
        .unwrap();

    let result = engine.add_entity_to_zone(z2, EntityId::Hyperconnector(h1));
    assert_eq!(result, Err(CoreError::Axiom9CycleDetected));
}

#[test]
fn test_axiom_10_flow_boundaries() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    let h1 = engine.create_hyperconnector(vec![]);
    let z1 = engine.create_zone(h1, vec![]).unwrap();

    let h2 = engine.create_hyperconnector(vec![]);
    let z2 = engine.create_zone(h2, vec![]).unwrap();

    let result = engine.create_flow(h1, z1, z2, FlowType::Dir, vec![]);
    assert_eq!(result, Err(CoreError::InvalidFlowBoundary));
}
```

### [19] `./tests/metadata_tests.rs`

```rust
use tisdb::{AttributeValue, CisowskiEngine, MemoryStorage};

#[test]
fn test_metadata_attributes_lifecycle() {
    let storage = MemoryStorage::new();
    let mut engine = CisowskiEngine::new(storage);

    // 1. Tworzenie węzła z zadeklarowaną klasą
    let nid = engine.create_node(vec!["Osoba".into(), "Mężczyzna".into()]);
    
    // 2. Pobranie i modyfikacja atrybutów (Update)
    let node = engine.get_node_mut(&nid).unwrap();
    assert_eq!(node.header.revision(), 1); // Wersja po utworzeniu

    node.header.set_attribute("Wiek", AttributeValue::Integer(35));
    node.header.set_attribute("Aktywny", AttributeValue::Boolean(true));

    assert_eq!(node.header.revision(), 3); // Wersja urosła dwukrotnie po dwóch update'ach

    // 3. Sprawdzenie, czy dane przetrwały i są prawidłowego typu
    let fetched_node = engine.get_node(&nid).unwrap();
    assert_eq!(fetched_node.header.class_path(), &["Osoba", "Mężczyzna"]);
    
    match fetched_node.header.get_attribute("Wiek") {
        Some(AttributeValue::Integer(val)) => assert_eq!(*val, 35),
        _ => panic!("Nieprawidłowy typ lub brak atrybutu Wiek"),
    }
}
```

### [20] `./tests/persistence_tests.rs`

```rust
use std::fs;
use tisdb::{AttributeValue, CisowskiEngine, EntityId, FileStorage, ScalarValue, TextFormat};

#[test]
fn test_binary_save_and_load_cycle() {
    let db_path = "test_persistence.cdb";
    let lock_path = "test_persistence.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let (node_id, hyper_id, zone_id) = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);

        let nid = engine.create_node(vec!["PersistNode".into()]);
        let hid = engine.create_hyperconnector(vec!["PersistHyper".into()]);
        let zid = engine.create_zone(hid, vec![]).unwrap();

        engine
            .add_entity_to_zone(zid, EntityId::Node(nid))
            .unwrap();

        engine.save().expect("Błąd podczas zapisu migawki");
        (nid, hid, zid)
    };

    // Ponowne otwarcie tej samej bazy z dysku
    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().expect("Błąd podczas odczytu migawki");

        assert!(engine.get_node(&node_id).is_some());
        assert!(engine.get_hyperconnector(&hyper_id).is_some());

        let zone = engine.get_zone(&zone_id).unwrap();
        assert_eq!(zone.contained_entities().len(), 1);
        assert_eq!(zone.contained_entities()[0], EntityId::Node(node_id));
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.class_path()[0], "PersistNode");
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_truncate_on_shrink() {
    let db_path = "test_truncate.cdb";
    let lock_path = "test_truncate.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // KROK 1: Zapis dużej bazy
    let size_large = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        for _ in 0..100 {
             engine.create_node(vec![]);
        }
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    // KROK 2: Nadpisanie małą bazą
    let size_small = {
        let storage = FileStorage::open(db_path).unwrap(); // Otwiera istniejacy plik
        let mut engine = CisowskiEngine::new(storage);
        // Nadpisuje 100 węzłów z pamięci pustym grafem
        engine.save().unwrap();
        fs::metadata(db_path).unwrap().len()
    };
    
    assert!(size_small < size_large, "Plik powinien zostać skrócony (truncated)");
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_attributes_persistence_hashmap() {
    let db_path = "test_attrs.cdb";
    let lock_path = "test_attrs.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    let node_id = {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let nid = engine.create_node(vec![]);
        
        let node = engine.get_node_mut(&nid).unwrap();
        node.header.set_attribute("wiek", AttributeValue::Integer(99));
        node.header.set_attribute("opis", AttributeValue::RichText { 
            format: TextFormat::Markdown, 
            content: "**Test**".to_string() 
        });
        node.header.set_attribute("tagi", AttributeValue::Set(vec![
             ScalarValue::String("tag1".into()),
             ScalarValue::String("tag2".into()),
        ]));
        
        engine.save().unwrap();
        nid
    };

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        engine.load().unwrap();
        
        let node = engine.get_node(&node_id).unwrap();
        assert_eq!(node.header.get_attribute("wiek"), Some(&AttributeValue::Integer(99)));
        
        match node.header.get_attribute("opis") {
            Some(AttributeValue::RichText { format, content }) => {
                assert_eq!(format, &TextFormat::Markdown);
                assert_eq!(content, "**Test**");
            },
            _ => panic!("Błąd persystencji RichText"),
        }
        
        match node.header.get_attribute("tagi") {
            Some(AttributeValue::Set(vec)) => {
                assert_eq!(vec.len(), 2);
            },
            _ => panic!("Błąd persystencji Set"),
        }
    }
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}
```

### [21] `./tests/redb_poc_tests.rs`

```rust
use std::fs;
use redb::{Database, ReadableDatabase, ReadableTable, TableDefinition};
use rkyv::{Archive, Deserialize, Serialize};
use rkyv::rancor::Error as RancorError;

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(derive(Debug, PartialEq))]
pub struct DummyNode {
    pub id: u64,
    pub class: String,
}

#[derive(Archive, Serialize, Deserialize, Debug, PartialEq)]
#[rkyv(derive(Debug, PartialEq))]
pub struct DummyZone {
    pub id: u64,
    pub name: String,
}

const NODES_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("nodes");
const ZONES_TABLE: TableDefinition<u64, &[u8]> = TableDefinition::new("zones");
const METADATA_TABLE: TableDefinition<&str, u32> = TableDefinition::new("metadata");

#[test]
fn test_redb_prototype_atomicity_and_unaligned_rkyv_integration() {
    let db_path = "test_redb_unaligned.db";
    let _ = fs::remove_file(db_path); 

    // KROK 1: Inicjalizacja i atomowa transakcja zapisu
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        
        {
            let mut node_table = write_txn.open_table(NODES_TABLE).unwrap();
            let mut meta_table = write_txn.open_table(METADATA_TABLE).unwrap();

            meta_table.insert("format_version", &2).unwrap();

            let node1 = DummyNode { id: 101, class: "Proces".into() };
            let bytes1 = rkyv::to_bytes::<RancorError>(&node1).unwrap();
            node_table.insert(101, bytes1.as_slice()).unwrap();
        } 
        
        write_txn.commit().unwrap();
    }

    // KROK 2: Odczyt transakcyjny
    {
        let db = Database::create(db_path).unwrap(); 
        let read_txn = db.begin_read().unwrap();
        
        let node_table = read_txn.open_table(NODES_TABLE).unwrap();
        
        let guard = node_table.get(101).unwrap().unwrap();
        let node1_bytes = guard.value();

        let archived_node = rkyv::access::<ArchivedDummyNode, RancorError>(node1_bytes)
            .expect("Rkyv odrzucił bajty z bazy redb - sprawdz wyrównanie (unaligned)!");
            
        assert_eq!(archived_node.id, 101);
        
        let loaded_node: DummyNode = rkyv::deserialize::<DummyNode, RancorError>(archived_node).unwrap();
        assert_eq!(loaded_node.id, 101);
    }

    let _ = fs::remove_file(db_path);
}

#[test]
fn test_redb_rollback_atomicity() {
    let db_path = "test_redb_rollback.db";
    let _ = fs::remove_file(db_path); 

    // 1. Zapis stabilny (Stan początkowy)
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        {
            let mut meta_table = write_txn.open_table(METADATA_TABLE).unwrap();
            meta_table.insert("format_version", &2).unwrap();
            
            // NOWE: Inicjalizujemy puste tabele, aby istniały dla transakcji odczytu
            let _ = write_txn.open_table(NODES_TABLE).unwrap();
            let _ = write_txn.open_table(ZONES_TABLE).unwrap();
        }
        write_txn.commit().unwrap();
    }

    // 2. Próba wielotabelowego zapisu (Symulacja przerwania / Abort)
    {
        let db = Database::create(db_path).unwrap();
        let write_txn = db.begin_write().unwrap();
        {
            let mut node_table = write_txn.open_table(NODES_TABLE).unwrap();
            let mut zone_table = write_txn.open_table(ZONES_TABLE).unwrap();
            
            // Częściowe dodanie danych
            let n = DummyNode { id: 777, class: "A".into() };
            let z = DummyZone { id: 888, name: "B".into() };
            
            node_table.insert(777, rkyv::to_bytes::<RancorError>(&n).unwrap().as_slice()).unwrap();
            zone_table.insert(888, rkyv::to_bytes::<RancorError>(&z).unwrap().as_slice()).unwrap();
            
            // Brak commit(). drop(write_txn) robi automatyczny rollback.
        }
    }

    // 3. Weryfikacja: Częściowe dane NIE MOGĄ znajdować się na dysku
    {
        let db = Database::create(db_path).unwrap();
        let read_txn = db.begin_read().unwrap();
        
        // Teraz open_table nie wyrzuci błędu, bo tabele zostały utworzone w kroku 1.
        let node_table = read_txn.open_table(NODES_TABLE).unwrap();
        let zone_table = read_txn.open_table(ZONES_TABLE).unwrap();
        
        assert!(node_table.get(777).unwrap().is_none(), "Baza naruszyła izolację - Node istnieje!");
        assert!(zone_table.get(888).unwrap().is_none(), "Baza naruszyła izolację - Zone istnieje!");
        
        let meta_table = read_txn.open_table(METADATA_TABLE).unwrap();
        assert_eq!(meta_table.get("format_version").unwrap().unwrap().value(), 2);
    }
    
    let _ = fs::remove_file(db_path); 
}
```

### [22] `./tests/storage_tests.rs`

```rust
use std::fs::{self, OpenOptions};
use std::io::Write;

use tisdb::{CisowskiEngine, CoreError, FileStorage};

#[test]
fn test_file_storage_lifecycle() {
    let db_path = "test_graph.cdb";
    let lock_path = "test_graph.cdb.lock";

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    {
        let storage = FileStorage::open(db_path).expect("Nie udało się utworzyć magazynu plików");
        let mut engine = CisowskiEngine::new(storage);

        let _node_id = engine.create_node(vec![]);
        let _hyper_id = engine.create_hyperconnector(vec![]);
    }

    assert!(fs::metadata(db_path).is_ok());

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}

#[test]
fn test_broken_header() {
    let db_path = "test_broken.cdb";
    let lock_path = "test_broken.cdb.lock";
    
    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);

    // Tworzymy "uszkodzony" plik
    {
        let mut file = OpenOptions::new().write(true).create(true).open(db_path).unwrap();
        file.write_all(b"CDB").unwrap(); // Tylko 3 bajty, za mało na nagłówek
    }

    {
        let storage = FileStorage::open(db_path).unwrap();
        let mut engine = CisowskiEngine::new(storage);
        let result = engine.load();
        
        match result {
             Err(CoreError::StorageError(msg)) => assert!(msg.contains("ucięty nagłówek")),
             _ => panic!("System nie wykrył uszkodzonego nagłówka"),
        }
    }

    let _ = fs::remove_file(db_path);
    let _ = fs::remove_file(lock_path);
}
```

## End Structure Summary

```plaintext
  ▣─┬ tisdb                             [499.7 KiB]                      A:/A-JAN/WIN-DOCS/REPO_OWN_RUST/GIT 
    │                                                                    _tisdb/tisdb/                       
 1  ├──• ARCHITECTURE.md                [  5.2 KiB] [2026-39-2 09:37:04] ./ARCHITECTURE.md                   
 2  ├──• Cargo.toml                     [  1.7 KiB] [2026-39-2 09:41:00] ./Cargo.toml                        
 3  ├──• MODEL.md                       [ 15.4 KiB] [2026-39-2 09:37:16] ./MODEL.md                          
 4  ├──• README.md                      [  1.4 KiB] [2026-39-2 09:36:23] ./README.md                         
    ├──┬ docs                           [417.7 KiB] [2026-39-2 07:45:33] ./docs/                             
    │  └──┬ images                      [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/                      
    │     └──• GRAFY_CISOWSKIEGO.png    [417.7 KiB] [2026-39-2 07:45:33] ./docs/images/GRAFY_CISOWSKIEGO.png 
    ├──┬ src                            [ 44.2 KiB] [2026-39-2 07:47:10] ./src/                              
 5  │  ├──• engine.rs                   [ 12.8 KiB] [2026-39-1 23:08:51] ./src/engine.rs                     
 6  │  ├──• error.rs                    [  1.1 KiB] [2026-39-1 22:33:50] ./src/error.rs                      
 7  │  ├──• lib.rs                      [    858 B] [2026-39-2 08:59:18] ./src/lib.rs                        
    │  ├──┬ domain                      [ 11.6 KiB] [2026-39-2 07:47:10] ./src/domain/                       
 8  │  │  ├──• flow.rs                  [  1.8 KiB] [2026-39-1 21:54:01] ./src/domain/flow.rs                
 9  │  │  ├──• hyper.rs                 [  2.0 KiB] [2026-39-1 21:52:34] ./src/domain/hyper.rs               
10  │  │  ├──• id.rs                    [  1.9 KiB] [2026-37-3 13:41:25] ./src/domain/id.rs                  
11  │  │  ├──• metadata.rs              [  3.6 KiB] [2026-39-1 22:48:17] ./src/domain/metadata.rs            
12  │  │  ├──• node.rs                  [    701 B] [2026-39-1 21:52:19] ./src/domain/node.rs                
13  │  │  └──• zone.rs                  [  1.5 KiB] [2026-39-1 21:53:46] ./src/domain/zone.rs                
    │  └──┬ storage                     [ 17.9 KiB] [2026-39-2 08:58:04] ./src/storage/                      
14  │     ├──• backend.rs               [    610 B] [2026-39-1 23:07:07] ./src/storage/backend.rs            
15  │     ├──• file.rs                  [  3.7 KiB] [2026-39-1 23:07:44] ./src/storage/file.rs               
16  │     ├──• memory.rs                [  1.1 KiB] [2026-39-1 23:08:08] ./src/storage/memory.rs             
17  │     └──• redb_backend.rs          [ 12.4 KiB] [2026-39-2 09:02:45] ./src/storage/redb_backend.rs       
    └──┬ tests                          [ 14.0 KiB] [2026-39-2 08:53:26] ./tests/                            
18     ├──• axiom_tests.rs              [  1.9 KiB] [2026-39-1 21:55:27] ./tests/axiom_tests.rs              
19     ├──• metadata_tests.rs           [  1.1 KiB] [2026-39-1 21:56:23] ./tests/metadata_tests.rs           
20     ├──• persistence_tests.rs        [  4.6 KiB] [2026-39-1 22:49:39] ./tests/persistence_tests.rs        
21     ├──• redb_poc_tests.rs           [  4.8 KiB] [2026-39-2 08:53:14] ./tests/redb_poc_tests.rs           
22     └──• storage_tests.rs            [  1.6 KiB] [2026-39-1 22:50:01] ./tests/storage_tests.rs            
```

---
*Generated automatically by querypath-snapshot*
