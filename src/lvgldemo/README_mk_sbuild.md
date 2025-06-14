https://earthly.dev/blog/cross-compiling-raspberry-pi/

sudo apt install ubuntu-dev-tools cmake curl
mk-sbuild --arch=armhf bullseye --debootstrap-mirror=http://archive.raspbian.org/raspbian/ --name=rpi-bullseye


wget https://github.com/tttapa/docker-arm-cross-toolchain/releases/latest/download/x-tools-armv8-rpi3-linux-gnueabihf-gcc14.tar.xz


/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-gcc examples/hello11.c -o examples/hello11
/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-readelf -h examples/hello11