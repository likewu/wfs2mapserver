## Examples of how to use various  `lvgl-rs` widgets/components

All examples can be executed using:
```shell
cargo run --example <name>
```
while in the `lvgl-rs` directory (i.e. one up from this).

The `DEP_LV_CONFIG_PATH` environment variable is necessary, as explained in the [README](../README.md).


sudo apt install libssl-dev
sudo apt install libsdl2-dev
sudo apt install libsndfile1-dev
sudo apt-get install gcc-aarch64-linux-gnu
sudo apt-get install g++-aarch64-linux-gnu
sudo apt-get install gcc-arm-linux-gnueabihf
sudo apt-get install g++-arm-linux-gnueabihf

Raspberry Pi 2B:
CC=clang cargo build --target=armv7-unknown-linux-gnueabihf

Raspberry Pi 4B:
LD_LIBRARY_PATH=/home/leafcolor/app/arm-linux-gnueabihf DEP_LV_CONFIG_PATH='/mnt/data/app/julia/wfs2map/src/lvgldemo/include' cargo build --example sdl2 -p lvgldemo --target=arm-unknown-linux-gnueabihf

mkdir /usr/include/arm-linux-gnueabihf
cp raspbian /usr/include/arm-linux-gnueabihf/SDL2 to /usr/include/arm-linux-gnueabihf/
cp raspbian /usr/lib/arm-linux-gnueabihf/ to /usr/lib/arm-linux-gnueabihf/
cargo build --target=arm-unknown-linux-gnueabihf


$ sdl2-config --cflags
-I/usr/include/SDL2 -D_REENTRANT
$ sdl2-config --libs
-lSDL2
sdl2-config --static-libs
/usr/lib//libSDL2.a -lm -lasound -lm -ldl -lpthread -lpulse-simple -lpulse -pthread -lsamplerate -lX11 -lXext -lXcursor -lXi -lXfixes -lXrandr -lXss -ldrm -lgbm -lwayland-egl -lwayland-client -lwayland-cursor -lxkbcommon -ldecor-0 -lpthread -lrt


qemu-system-aarch64 -M raspi3b -drive "format=raw,if=sd,file=g:/2025-05-06-raspios-bullseye-armhf.img" -no-reboot -append "rw earlyprintk loglevel=8 root=/dev/mmcblk0p2 rootdelay=1" -dtb bcm2710-rpi-3-b-plus.dtb -kernel kernel8.img -usb -device usb-mouse -device usb-kbd -device usb-net,netdev=net0 -netdev user,id=net0,hostfwd=tcp::5022-:22
