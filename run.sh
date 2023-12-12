sudo apt install libproj-dev libgdal-dev libprotobuf-c-dev
#sudo apt install libmapserver-dev
sudo apt install apt-transport-https ca-certificates -y
#sudo update-ca-certificates
RUSTFLAGS="-Clink-arg=-L -Clink-arg=/mnt/data/app/opt/lib" cargo build
LD_LIBRARY_PATH=/mnt/data/app/opt/lib /mnt/data/app/julia/wfs2map/target/debug/mapserver


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


cd /mnt/data/app/julia/wfs2map/src/JWKS-Server && RUST_LOG=info,dep1=debug cargo run
cd /mnt/data/app/julia/wfs2map/src/JWKS-Server && /mnt/data/app/consul/envoy -c docker/envoy/envoy.yaml -l debug
curl -d "client_id=service&username=johndoe&password=tiger2&grant_type=password" http://192.168.1.11:8000/tokens | jq
curl http://192.168.1.11:8000/.well-known/jwks.json
let ACCESS_TOKEN = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.XOHs619Krkxp3LKBFYa3afSdE5NtDUzj40VNIcP08DY"
curl -v http://192.168.1.11:8000/.well-known/jwks.json -H $"Authorization: Bearer ($ACCESS_TOKEN)"


