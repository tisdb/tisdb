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
