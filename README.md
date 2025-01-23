# Mandelbrot generation SIMD vs SISD
## Hemsida
Det går att testa resultaten på https://inda24plusplus.github.io/dexterws-simd/, det är däremot inte lika stor skillnad som på native.
## Native benchmarks
Med en resolution på 800 x 600, och en max-iteration på 500 är detta resultaten.
```
SIMD: 61,722,110.00 ns/iter (+/- 4,115,700.00)
SISD: 233,835,220.00 ns/iter (+/- 3,375,636.00)
```
I ett worst case scenario blir SIMD därmed ~3.5x snabbare

## Kommentarer
Jag använder mig endast av en f32x4 vektor, då WASM verkar endast stödja 128 bitars operationer. Det hade därmed gått att få snabbare på native, men jag valde att inte göra det då resultatet fortfarande är märkbart mycket bättre.

### Resurser
[Mandelbrot generation](https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set)
