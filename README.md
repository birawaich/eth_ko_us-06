# eth_ko_us-06
ETH: KomA: Übungsserie 06, Aufgabe 5

### Sample Output

```
ETH: Komplexe Analysis: Übungsserie 06, Aufgabe 5
-------------------------------------------------
z0 = 0+0i
f1(z) = cos(z^3 - sin(z))
f2(z) = tan(z^7 + PI/4)

f1(z0) = 1+0i
f2(z0) = 1+0i

(a)
Monte Carlo Intergration f1(z0): 0.9814564348007601-0.0055709462970243i
Monte Carlo Intergration f2(z0): -0.30570194026845654-0.022611581681491317i

(b)
Approximation f1(z0): 0.9791741611826419-0.0005220964632839023i
Approximation f2(z0): -0.2771740382905018+0.029996530197301804i
-------------------------------------------------
```
### Notes
- My first rust program & repository. Made for class: "Komplexe Analysis".
- Heavily improved code by @troublescooter
- Should implement _Monte Carlo Intergration_ and an _Approximation_ to examine the "Mittelwertsatz".
