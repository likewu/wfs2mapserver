sudo apt install libproj-dev libgdal-dev libprotobuf-c-dev
#sudo apt install libmapserver-dev
sudo apt install apt-transport-https ca-certificates -y
#sudo update-ca-certificates
RUSTFLAGS="-Clink-arg=-L -Clink-arg=/mnt/data/app/opt/lib" cargo build
LD_LIBRARY_PATH=/mnt/data/app/opt/lib /mnt/data/app/julia/wfs2map/target/debug/mapserver


ulimit -c unlimited
sudo bash -c 'echo "/var/corefile/%e-%s.core" > /proc/sys/kernel/core_pattern'
sudo mkdir /var/corefile
sudo chmod a+w /var/corefile
gdb ./target/debug/rustfs /var/corefile/rustfs-6.core


/mnt/data/app/LLVM-20.1.3-Linux-X64/bin/lldb-server platform --server --listen "*:1234"
(lldb) platform select remote-linux
(lldb) platform connect connect://192.168.1.11:1234
(lldb) file ./target/debug/rustfs
(lldb) platform settings -w /mnt/data/app/julia/s3-rustfs
(lldb) platform status
(lldb) b ecstore/src/tier/tier.rs:379
(lldb) breakpoint list
(lldb) run
(lldb) next
(lldb) frame variable
(lldb) print data
(lldb) expression 1+1


#D:\nodejs-nvm\v14.19.1\node E:\app\nodejs\demo\geotiffinfo.js
#http://192.168.1.11:3000/map/2019/1/100/30


https://demo.mapserver.org/cgi-bin/wfs?SERVICE=WFS&VERSION=1.0.0&REQUEST=GetCapabilities
https://demo.mapserver.org/cgi-bin/wfs?SERVICE=WFS&VERSION=1.0.0&REQUEST=getfeature&TYPENAME=continents&MAXFEATURES=100
https://demo.mapserver.org/cgi-bin/wfs?SERVICE=WFS&VERSION=1.0.0&REQUEST=DescribeFeatureType&TYPENAME=continents&OUTPUTFORMAT=XMLSCHEMA


sudo apt install libcurlpp-dev libxml2-dev protobuf-c-compiler libfcgi-dev swig
sudo apt install ruby-full
cd mapserver-8.0.1/build
cmake -DCMAKE_INSTALL_PREFIX=/mnt/data/app/opt -DCMAKE_PREFIX_PATH=/usr/local/pgsql/91:/usr/local:/opt  -DWITH_CLIENT_WFS=ON -DWITH_CLIENT_WMS=ON -DWITH_CURL=ON -DWITH_SOS=ON -DWITH_JAVA=OFF -DWITH_CSHARP=OFF -DWITH_PERL=OFF -DWITH_RUBY=OFF -DWITH_SVGCAIRO=OFF -DWITH_ORACLESPATIAL=OFF -DWITH_MSSQL=OFF ../ >../configure.out.txt
make install
/mnt/data/app/opt/bin/mapserv


https://github.com/perrygeo/openlayers/blob/40f92963727d9e1bac1287ceea2395a1a99f50f7/lib/OpenLayers/Protocol/WFS.js#L30
https://github.com/perrygeo/ol3/blob/163cc5b8730629955390a0a1051d273e5660c22f/src/ol/format/wfsformat.js#L27
https://github.com/perrygeo/ol3/blob/163cc5b8730629955390a0a1051d273e5660c22f/examples/vector-wfs.js#L16


https://www.funnel-labs.io/2022/10/21/envoyproxy-5-authorized-access-with-jwt/
cd /mnt/data/app/julia/wfs2map/src/JWKS-Server && RUST_LOG=info,dep1=debug cargo run
cd /mnt/data/app/julia/wfs2map/src/JWKS-Server && /mnt/data/app/consul/envoy -c docker/envoy/envoy.yaml -l debug
curl -d "client_id=service&username=johndoe&password=tiger2&grant_type=password" http://192.168.1.11:8000/tokens | jq
curl http://192.168.1.11:8000/.well-known/jwks.json
{"keys":[{"alg":"RS256","kty":"RSA","use":"sig","n":"uwRXRCjow4hPZyguA6V4SK2jzcggA6tDlbYvx1m0a8X4Qu1aQ7UWxTXQRFkKgEY4LQkCEs5MJy8JMAX56p4CU6rHB7Elth_JtPToYEPGjmAFzH_2D7LQ49xk4jNJhAs_g4wmcHEPnesiijEc0wc9ZnI6-W2YT2PNAm3r4LYUQu6KS2eRGkHA_6Hi4gRWjFHPk2_j0LYTg3eQOu33Lgum1REusY1omMgflSF1eZdY_-y8HUy4sVNmJ61SOLAqBaICsv0eXtYM5rwR9Ioc0IXIxwQ_hhPMDn4Ck9AN8OqPIX4Cep3ocd3NSao66cwtsZI6qJz6Y338IjM98hhzAsnOSQ","e":"AQAB","kid":"6d4a32197d0c82cf414cdf6f0ccdac39e9c4f338"}]}
let ACCESS_TOKEN = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6IjZkNGEzMjE5N2QwYzgyY2Y0MTRjZGY2ZjBjY2RhYzM5ZTljNGYzMzgifQ.eyJzdWIiOiJqb2huZG9lIiwiZXhwIjoxNzAyNDAyMTQ1LCJpYXQiOjE3MDIzOTg1NDUsImlzcyI6Imh0dHA6Ly8xOTIuMTY4LjEuMTE6ODAwMCIsIm5hbWUiOiJKb2huIERvZSIsInB1cnBvc2UiOiJyZWFkIn0.me0ic7naRpK1QB59G5JKQsF-agzfyo2lEUc8u5vup7ku_RXIXzYF2KB70SDj4ez5luly3intsYOkgtp-VzQdBKGb3qXmKO-jjjxLr2ea7ZxZ0BCVubanRu3yobB0Iu8igKrYoEajEaPdObMDWdqWIhl4-gbtjODwIk5vCIf113kdtROZwe7_GffgCiVDzrtsJeZwGWu2bQtJzFW0GXIT4iw22vMMaUs-G7BBqibkdXOrhpbiXLduLLJJNAxKbMIggtk9vKmqudn1bZxJGn2J7CXGcE0Ni2dHGXcUuN_hmpALgCBlDG1iMxQ4k52CjqIH-MokQzPsQXnBZ1zMH3AiJA"
let ACCESS_TOKEN = (curl -d "client_id=service&username=johndoe&password=tiger2&grant_type=password" http://127.0.0.1:8080/tokens | jq -r '.access_token')
curl -v http://192.168.1.11:8000/api/user -H $"Authorization: Bearer ($ACCESS_TOKEN)"


#ip route add 192.168.101.0/24 via 192.168.1.2 #no use
#LAPTOP-NEGU2RE3 静态ip 192.168.1.21
#无线路由 组网模式：从路由模式
sudo rm /usr/bin/python3
sudo ln -s /usr/bin/python3.8 /usr/bin/python3
sudo netplan apply
