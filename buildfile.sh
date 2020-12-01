#!/bin/bash




cd express_server
cd public
rm chesscheckersgame -r



#build the chesspoker package
cd chesspoker_package
# dont think i need to build it here
#cargo build

cd ..
cd wasm_builder

#build the wasm package with the target of web
wasm-pack build --target web

cd pkg

#copy the package created into the frontend wasm file directory
cp wasm_builder.js ../../chesscheckersgame/wasmfiles/
cp wasm_builder_bg.wasm ../../chesscheckersgame/wasmfiles/

#copy the chesspoker game to the server
cd ..
cd ..
cp ./chesscheckersgame ./express_server/public -r

#run the server
cd express_server
npm run start
