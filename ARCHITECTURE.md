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
