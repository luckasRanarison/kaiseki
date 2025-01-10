#!/bin/bash

# This is the original source from https://taku910.github.io/mecab/ (broken)
# DOWNLOAD_URL="https://drive.google.com/uc?export=download&id=0B4y35FiV1wh7MWVlSDBCSXZMTXM"

VERSION=2.7.0-20070801
DOWNLOAD_URL="https://sourceforge.net/projects/mecab/files/mecab-ipadic/$VERSION/mecab-ipadic-$VERSION.tar.gz"

echo "Fetching mecab-ipadic..."

curl -Lfs $DOWNLOAD_URL --output mecab-ipadic.tar.gz

echo "Extracting archive..."

tar xvf mecab-ipadic.tar.gz --strip-components=1 -C ./mecab >/dev/null

echo "Operation complete!"
