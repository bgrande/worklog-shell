#!/bin/bash

if [ $1 ]; then
  TARGETPATH=$1
else
  echo "Missing target path parameter. Use like ./ocr.sh <TARGETPATH>"
  exit 1
fi

LANG=deu
if [ $2 ]; then
  LANG=$2
fi

#ADDPARAMS=hocr
ADDPARAMS=
SEGMENTATION=1
DEFAULT_OEM=3
PROCESSES=4

for file in "$PWD/$TARGETPATH/"*.tiff
do
    base_file_name=$(basename -s .tiff $file)
    echo "now processing $file" \
    && output_file_ocr="$base_file_name.xml" \
    && /usr/bin/tesseract -l ${LANG} --psm ${SEGMENTATION} --oem ${DEFAULT_OEM} "${file}" "${output_file_ocr}" -c tessedit_char_whitelist="0123456789.,:;€\$ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÜabcdefghijklmnopqrstuvwxyzäöüß-+*\?!><\"%\(\)\&=" ${ADDPARAMS}
done
