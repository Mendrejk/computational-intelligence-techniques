- fenotyp (danie - porcja): kalorie, węglowodany, tłuszczne, proteiny, cena
- genotyp: porcja dania i ilość porcji, np. 100g zmieniaków, 5 porcji

- cel kaloryczny: 2250
- cel węglowodanów: 275
- cel tłuszczy: 50
- cel protein: 120

dozwolone wachania:
- kalorii: 250
- węglowodanów: 40
- tłuszczy: 100
- protein: 75

funkcja celu:
\- cena ^ 2 + kalorie + węglowodany ^ 2 + tłuszcze + proteiny ^ 2

- populacja: 100
- ilość pokoleń: 1000
- **limit wyboru jednego dania: 10 porcji**

Operacje:
- selekcja maksymalna (?) 85% rodziców, 
- krzyżowanie jednopunktowe
- mutacja losowa 10%
- reinsercja (zachowanie 15% starej populacji)

**Test z wartościami domyślnymi**:

[("potatoes, 100g", 9), ("brown rice, 100g", 2), ("spinach, 100g", 2), ("sweet potato, 100g", 1), ("oatmeal, per serving (40g)", 4), ("eggs, per piece", 9), ("chickpeas can, drained (240g)", 1)]


| fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| ------- | -------------- | ------- | -------- | -------- | ---- |
| 101785 | 7 | 2282 | 315 | 57 |111 | 110 |
| 105093 | 6 | 2488 | 315 | 68 | 144 | 132 |
| 104412 | 9 | 2500 | 315 | 68 | 150 | 141 |
| 102846 | 7 | 2402 | 314 | 63 | 131 | 124 |
| 103876 | 7 | 2479 | 313 | 68 | 146 | 134 |
^table1

```chart
type: line
id: table1
layout: rows
width: 80%
beginAtZero: true
```


**Testy populacyjne - średnie z 3 odpaleń**

| populacja | fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| - |------- | -------------- | ------- | -------- | -------- | ---- |
|1000 |103950 | 5 | 2498 | 67 |139 |131 |
|100 |105093 | 6 | 2488 |315 |68 |144 |132 |
|25 |-99999999 | 8 | 6058 | 571 | 178 | 548 | 1026 |


**Mutacja losowa 35%**

[("potatoes, 100g", 9), ("brown rice, 100g", 6), ("broccoli, 100g", 2), ("sweet potato, 100g", 1), ("eggs, per piece", 9), ("cottage cheese, per serving (200g)", 1)]


| fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| ------- | -------------- | ------- | -------- | -------- | ---- |
| 102799 | 6 | 2348 | 315 | 61 |119 | 114 |
| 102571 | 7 | 2357 | 315 | 61 | 118 | 114 |
| 103187 | 7 | 2462 | 315 | 65 | 146 | 141 |


**Reinsercja - zachowanie 50% populacji**

[("potatoes, 100g", 8), ("brown rice, 100g", 1), ("spinach, 100g", 1), ("sweet potato, 100g", 7), ("oatmeal, per serving (40g)", 1), ("eggs, per piece", 9), ("cottage cheese, per serving (200g)", 2)]

| fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| ------- | -------------- | ------- | -------- | -------- | ---- |
| 102536 | 7 | 2498 | 314 | 67 |140 | 135 |
| 102413 | 6 | 2329 | 314 | 60 | 122 | 116 |
| -99999999 | 14 | 9590 | 979 | 218 | 908 | 1345 |

**Selekcja 80% rodziców**

[("potatoes, 100g", 9), ("sweet potato, 100g", 2), ("oatmeal, per serving (40g)", 1), ("eggs, per piece", 9), ("cottage cheese, per serving (200g)", 1), ("chickpeas can, drained (240g)", 2)]

| fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| ------- | -------------- | ------- | -------- | -------- | ---- |
| 205093 | 6 | 2488 | 315 | 68 |144 | 132 |
| 101757 | 8 | 2466 | 315 | 66 | 137 | 137 |
| -99999999 | 15 | 9815 | 1245 | 191 | 768 | 1292 |

**Tylko ograniczenie kalorii, funkcja celu - negacja ceny**

[("potatoes, 100g", 9), ("brown rice, 100g", 5), ("sweet potato, 100g", 2), ("eggs, per piece", 9)]

| fitness | unikalne dania | kalorie | tłuszcze | proteiny | cena |
| ------- | -------------- | ------- | -------- | -------- | ---- |
| -78 | 4 | 2023 | 290 | 50 |84 | 78 |
| -99999999 | 13 | 7308 | 594 | 190 | 790 | 1114 |
| -78 | 4 | 2023 | 290 | 50 | 84 | 78 |


# Lista 2

## Analiza metod selekcji - elitarna, ruletka, turniej
Średnie funkcji celu z 5 iteracji

### selekcja elitarna

|1|-40000516908|12|

| iteracja | fitness | unikalne dania |
| -------- | ------- | -------------- |
|10|-23005|8|
|20|-15339|8|
|30|-8927|6|
|40|-7686|5|
|50|-8096|6|
|60|-9876|5|
|70|-10333|5|
|80|-6428|5|
|90|-7312|5|
|100|-8859|5|
|200|-7704|5|
|300|-6556|5|
|400|-6514|5|
|500|-6053|5|
|600|-6209|4|
|700|-6043|5|
|800|-6467|5|
|900|-6578|5|
|1000|-6238|5|
|10000|-6160|5|
^selekcjaElitarna

```chart
type: line
id: selekcjaElitarna
layout: columns
select: [fitness]
width: 80%
beginAtZero: true
```