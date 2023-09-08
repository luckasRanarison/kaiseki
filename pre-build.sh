#!/bin/bash

DOWNLOAD_URL="https://drive.google.com/uc?export=download&id=0B4y35FiV1wh7MWVlSDBCSXZMTXM"

echo "Fetching mecab-ipadic..."
curl -Lfs $DOWNLOAD_URL --output mecab-ipadic.tar.gz
echo "Extracting archive..."
tar xvf mecab-ipadic.tar.gz --strip-components=1 -C ./mecab >/dev/null
cd mecab
touch term.fst term.bin feature.bin matrix.bin char.bin unk.bin
echo "Operation complete!"
