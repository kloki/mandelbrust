# MandelbRUST

Mandelbrot in rust.
https://en.wikipedia.org/wiki/Mandelbrot_set

![Example](./example.png)

```
cargo build --release
./target/release/mandelbrust --help
./target/release/mandelbrust
./target/release/mandelbrust -x=-0.745 -y=0.1 --zoom=200 --iterations 10000 --output-file=zoomed.png
```
