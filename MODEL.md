# Cisowski's Hyperconnector Model (Hiperkonektory Cisowskiego / Grafy Cisowskiego)

Wprowadzam koncepcje "Grafów Cisowskiego" - są nadrzędnym meta-modelem (nadzbiorem), klasyczne teorie grafów i hipergrafów to jedynie jego zubożone podstruktury.

![Diagram](docs/images/GRAFY_CISOWSKIEGO.png)

1. Klasyczny graf skierowany $\Big(\Phi\Big)$

  - Tradycyjne ujęcie: Bezpośrednia relacja binarna 1:1 łącząca dwa wierzchołki $\Big(u \to v\Big)$. Krawędź jest jedynie „płaskim” połączeniem pozbawionym własnej struktury wewnętrznej.

  - Redukcja w Meta-Modelu: Hiperkonektor zredukowany do dwóch jednoelementowych stref portowych $\Big(z_e^{(1)}$ zawiera $u$, $z_e^{(2)}$ zawiera $v\Big)$ powiązanych pojedynczą instancją przepływu skierowanego $\Big(\pi_F(f_e) = (z_e^{(1)}, z_e^{(2)}, \text{dir})\Big)$.

2. Klasyczny hipergraf Berge’a $\Big(\Phi_{\text{hyper}}\Big)$

  - Tradycyjne ujęcie: Płaski podzbiór wierzchołków $\Big(e \subseteq V\Big)$. Wszystkie elementy w krawędzi są równorzędne — brak tam jakichkolwiek wyróżnionych ról, portów czy struktury wewnętrznej.

  - Redukcja w Meta-Modelu: Skrajnie uproszczony hiperkonektor całkowicie pozbawiony wewnętrznej dynamiki relacyjnej $\Big(\vert{}F_{h_e}\vert{} = 0\Big)$, posiadający zaledwie jedną strefę portową $\Big(z_e\Big)$, której funkcja zawartości przechowuje pełny zbiór wierzchołków $\Big(\mu_Z(z_e) = \Phi_{\text{hyper}}(e)\Big)$.

3. Klasyczny skierowany hipergraf $\Big(\Phi_{\text{dir\_hyper}}\Big)$

  - Tradycyjne ujęcie: Relacja wieloargumentowa łącząca podzbiór wejściowy (Tail) z podzbiorem wyjściowym (Head) w formule $T \to H$.

  - Redukcja w Meta-Modelu: Hiperkonektor posiadający dokładnie dwie strefy interfejsowe $\Big(z_e^{\text{tail}}$ oraz $z_e^{\text{head}}\Big)$, z których każda agreguje odpowiedni podzbiór wierzchołków, spięte jednym wewnętrznym przepływem skierowanym między tymi portami.

4. Metagrafy / Grafy hierarchiczne

  - Tradycyjne ujęcie: Umożliwiają łączenie całych podgrafów lub krawędzi z innymi krawędziami, jednak często cierpią na brak twardej izolacji (połączenia skrośne przeskakują poziomy zagnieżdżenia).

  - Redukcja w Meta-Modelu: Pełna, inżynieryjna enkapsulacja. Dowolna strefa może zawierać inne hiperkonektory $\Big(\mu_Z(z) \subseteq U\Big)$, tworząc ufundowaną strukturę zagnieżdżoną $\Big(\mathcal{R}_{\text{contain}}\Big)$, w której jakikolwiek ruch skrośny jest ściśle kontrolowany przez dedykowane porty $\Big(Z_h\Big)$ i lokalne przepływy $\Big(F_h\Big)$.
