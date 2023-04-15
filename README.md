# tess-test

## Building
- see [tesseract-sys](https://github.com/ccouzens/tesseract-sys) and [opencv-rust](https://github.com/twistedfall/opencv-rust) for more infos and help how to build (for example on windows)
- 

### Local requirements
- Arch-based
  ```sh
  # testdata-all package: tesseract-data, opencv addition: vtk glew fmt openmpi
  pacman -S clang pkg-config leptonica tesseract-dev qt5-base opencv
  ```
- Debian-based
  ```sh
  # testdata-all package: tesseract-ocr-all
  apt install -y pkg-config clang libleptonica-dev libtesseract-dev libopencv-dev
  ```
- for windows and other unix systems, see [tesseract-sys#Building](https://github.com/ccouzens/tesseract-sys#building)

### With docker
> see [build_n_run.sh](./build_n_run.sh) (requires [`docker buildx`](https://docs.docker.com/engine/reference/commandline/buildx/) for building the images)
- build the provided docker images with the given Dockerfiles
  - `builder.Dockerfile` => primary for building the executable and linting
    - no testdata or language files installed, so only useful for building
    - add user "rustacean" with current users `uid:gid` to the environment, so that no permission clash occurs
  - `runner.Dockerfile` => "minimal" sized environment for executable execution
- the script has three modes `run`, `build` or `linting` (default: `run` or first provided argument)
  - `build`: `cargo build` with provided configuration (default: `release`)
  - `run`: executes a `build` (so that the executable is always available) and then runs the executable with the runner image
  - `linting`: [`bacon`](https://github.com/Canop/bacon) `clippy`
