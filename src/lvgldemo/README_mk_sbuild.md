https://earthly.dev/blog/cross-compiling-raspberry-pi/

sudo apt install ubuntu-dev-tools cmake curl
mk-sbuild --arch=armhf bullseye --debootstrap-mirror=http://archive.raspbian.org/raspbian/ --name=rpi-bullseye


wget https://github.com/tttapa/docker-arm-cross-toolchain/releases/latest/download/x-tools-armv8-rpi3-linux-gnueabihf-gcc14.tar.xz
wget https://master.dl.sourceforge.net/project/raspberry-pi-cross-compilers/Raspberry%20Pi%20GCC%20Cross-Compiler%20Toolchains/Bullseye/GCC%2014.2.0/Raspberry%20Pi%203A%2B%2C%203B%2B%2C%204%2C%205/cross-gcc-14.2.0-pi_3%2B.tar.gz?viasf=1


/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-gcc examples/hello11.c -o examples/hello11
LIBRARY_PATH=/home/leafcolor/app/arm-linux-gnueabihf /home/leafcolor/opt/cross-pi-gcc-14.2.0-2/bin/arm-linux-gnueabihf-gcc examples/sdl2.c -o examples/sdl2_pi3 -I/usr/include/SDL2 -I/home/leafcolor/opt/cross-pi-gcc-14.2.0-2/arm-linux-gnueabihf/sysroot/usr/include -I/usr/include/arm-linux-gnueabihf -Wl,-rpath-link,/home/leafcolor/app/arm-linux-gnueabihf -Wl,-Bdynamic -lSDL2 -lwayland-client -lwayland-egl -lwayland-cursor -lwayland-server -lsystemd -lasyncns -lbsd -lffi -lasound -lgbm -lxkbcommon -lXxf86vm -ldrm -ldrm_amdgpu -lpulse -lpulsecore-14.2 -lpulsecommon-14.2 -lpulsedsp -ldbus-1 -lsamplerate -lwrap -lmd -lxcb -lexpat -lxml2 -lX11 -lXext -lXau -lXi -lXfixes -lXcursor -lXss -lXdmcp -lXrandr -lXrender -lXinerama -L/home/leafcolor/app/arm-linux-gnueabihf -static
--sysroot=/home/leafcolor/app/arm-linux-gnueabihf
/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-readelf -h examples/hello11
/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-ldd --root /home/leafcolor/opt/cross-pi-gcc-14.2.0-2/arm-linux-gnueabihf/libc examples/hello11
/home/leafcolor/opt/x-tools/armv8-rpi3-linux-gnueabihf/bin/armv8-rpi3-linux-gnueabihf-ldd --root /home/leafcolor/app/arm-linux-gnueabihf ../../target/armv7-unknown-linux-gnueabihf/debug/examples/sdl2
