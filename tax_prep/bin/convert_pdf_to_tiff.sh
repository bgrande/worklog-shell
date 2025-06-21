#!/bin/bash

for file in "$PWD"/*.pdf
do
    base_file_name=$(basename -s .pdf $file)
    echo "now processing $file" \
    && output_file_tiff="$base_file_name.tiff" \
    && /usr/bin/gs -sDEVICE=tiffgray -r720x720 -sPAPERSIZE=a4 -sCompression=lzw -o ${output_file_tiff} ${file}
done
