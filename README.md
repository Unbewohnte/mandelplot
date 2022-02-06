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


## TODO:
- generate image in parallel