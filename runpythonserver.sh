#!/bin/bash




#delete the old wasm files
rm chesscheckersgame/wasmfiles/ -r
#and remake the empty directory
mkdir chesscheckersgame/wasmfiles

cd wasm_builder

#build the wasm package with the target of web
wasm-pack build --target web


#copy the package created into the frontend wasm file directory
cp pkg/wasm_builder.js ../chesscheckersgame/wasmfiles/
cp pkg/wasm_builder_bg.wasm ../chesscheckersgame/wasmfiles/


#go to that directory
cd  ..
cd chesscheckersgame


echo "running the python server that serves the game files"

#serve the files
python3 -m http.server 8000
