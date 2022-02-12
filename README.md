# mandelplot
### Generate mandelbrot set with specified dimensions, a couple of palettes included !

## Usage
mandelplot [OPTIONS]

OPTIONS:

- `-d, --image_dimensions <image_dimensions>` => Set image dimensions (WIDTHxHEIGHT) [default: 7680x4320]

- `-h, --help` => Print help information

- `-i, --max_iter <max_iter>` => Specify maximum amount of iterations to decide whether the point escapes to infinity or  not [default: 1000]

- `-n, --image_name <image_name>` => Set output image filename [default: mandelbrot]

- `-p, --palette <palette>` => Specify bulb color (light, dark) [default: light]

- `-V, --version` => Print version information


## Naive benchmarks
Singe-threaded (19200x10800) (pre v0.2.0)
- ./mandelplot -d 19200x10800 -p dark  70.06s user 0.14s system 99% cpu 1:10.24 total

Multi-threaded (19200x10800) (v0.2.0)
- ./target/release/mandelplot -d 19200x10800 -p dark  85.28s user 8.06s system 258% cpu 36.079 total

1:10 - 0:36 == 0:34 saved

## TODO:
- ~~generate image in parallel~~
- zooming in