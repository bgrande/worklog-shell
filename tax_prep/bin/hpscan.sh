#!/bin/bash

# needs reportlab
# needs gs
# needs ps2pdf

if ! [ -f .env ]; then
  echo "we need a .env file. please exec make setup!"
  exit 1
fi
if ! [ -f .scantype ]; then
  echo "we need a .scantype file. please exec make adf or make duplex!"
  exit 1
fi

source .env
source .scantype

if ! [ "$TARGETPATH" ]; then
  echo "Missing TARGETPATH variable!"
  exit 1
fi


#MERGE=0
#if [ $2 = "--merge" ]; then
#    MERGE=1
#fi

if ! [ "$TARGETNAME" ]; then
  echo "Missing TARGETNAME variable!"
  exit 1
fi

if ! [ "$SCANTYPE" ]; then
  echo "Missing target SCANTYPE variable!"
  exit 1
fi

echo "using scantype: ${SCANTYPE}"

#scanimage -p -d "hpaio:/net/OfficeJet_5200_series?ip=192.168.0.228" --format=pnm -x 210 -y 297 --resolution 600 --source ADF --mode Gray > test.pnm
# airscan:e1:HP OfficeJet 5200 series
SCANRES=$(hp-scan --device="escl:https://192.168.0.228:443" --res=300 --mode=gray --size=a4 --dest=pdf ${SCANTYPE} --output="$TARGETPATH/$TARGETNAME" 2>&1 >/dev/null)

if echo "${SCANRES}" | grep -q error; then
  echo "error detected while scanning:"
  echo ${SCANRES}
  exit 1
fi

# /usr/bin/gs -sDEVICE=tiffgray -r720x720 -sPAPERSIZE=a4 -sCompression=lzw -o ${outputFile} ${pathToPdf}
ps2pdf -dPDFSETTINGS=/ebook "$TARGETPATH"/"${TARGETNAME}" "${TARGETPATH}"/"${TARGETNAME_SMALL}"