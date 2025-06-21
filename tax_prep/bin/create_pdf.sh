#!/bin/bash
#echo $PWD
#exit 1

#MERGE=0
#if [ $2 = "--merge" ]; then
#    MERGE=1
#fi

TARGETNAME=test.pdf
if [ $1 ]; then
    TARGETNAME="$1"
fi

TYPE=--adf
if [ $2 ]; then
    if [ $2 = 'dup' ]; then	   
        TYPE=--dup
    fi
fi

PROCESSES=4

LANG=deu
SEGMENTATION=1
DEFAULT_OEM=3

# /usr/bin/gs -sDEVICE=tiffgray -r720x720 -sPAPERSIZE=a4 -sCompression=lzw -o ${outputFile} ${pathToPdf}
ps2pdf -dPDFSETTINGS=/ebook ${TARGETNAME} ${TARGETNAME}-small.pdf




