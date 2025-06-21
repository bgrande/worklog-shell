#!/bin/bash

if ! [ -f .env ]; then
  echo "we need a .env file. please exec make setup!"
  exit 1
fi
if ! [ -f .scantype ]; then
  echo "we need a .scantype file. please exec make adf or make duplex!"
  exit 1
fi

source .env

rm ${TARGETPATH}/${TARGETNAME}
rm ${TARGETPATH}/${TARGETNAME_SMALL}