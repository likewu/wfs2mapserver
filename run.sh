sudo apt install libproj-dev libgdal-dev
#sudo apt install libmapserver-dev
RUSTFLAGS="-Clink-arg=-L -Clink-arg=/mnt/data/app/opt/lib" cargo build
LD_LIBRARY_PATH=/mnt/data/app/opt/lib /mnt/data/app/julia/wfs2map/target/debug/mapserver


https://demo.mapserver.org/cgi-bin/wfs?SERVICE=WFS&VERSION=1.0.0&REQUEST=GetCapabilities


sudo apt install libcurlpp-dev libxml2-dev libprotobuf-c-dev protobuf-c-compiler libfcgi-dev swig
sudo apt install ruby-full
cd mapserver-8.0.1/build
cmake -DCMAKE_INSTALL_PREFIX=/mnt/data/app/opt -DCMAKE_PREFIX_PATH=/usr/local/pgsql/91:/usr/local:/opt  -DWITH_CLIENT_WFS=ON -DWITH_CLIENT_WMS=ON -DWITH_CURL=ON -DWITH_SOS=ON -DWITH_JAVA=OFF -DWITH_CSHARP=OFF -DWITH_PERL=OFF -DWITH_RUBY=OFF -DWITH_SVGCAIRO=OFF -DWITH_ORACLESPATIAL=OFF ../ >../configure.out.txt
make install
/mnt/data/app/opt/bin/mapserv
