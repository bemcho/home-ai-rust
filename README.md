

Home AI rewritten in RUST

- pre build
  - sudo apt install libgtk2.0-dev  libspeechd-dev 

  - chmod +x deps/install-install-opencv.sh
  - export OPENCV_VERSION=4.2.0 && ./deps/install-install-opencv.sh
  - sudo ldconfig

- build   
   - rustup update
   - cargo build

- run (you need camera) 
   - cargo run