#!/bin/bash

read -p "Enter target path: " targetpath

echo "Working with \"${targetpath}\" now"

if ! [ -d "${targetpath}" ]; then
 echo "target does not seem to be a dir! Please check and try again!"
 exit 1
fi

# set target path
echo "TARGETPATH=${targetpath}" > .env

# create target filename for each scan
C_YEAR=$(date +%Y)
BASENAME=invoice_scan_$(($C_YEAR-1))
echo "TARGETNAME=${BASENAME}.pdf" >> .env

# set basename
echo "BASENAME=${BASENAME}" >> .env

# set small filename
echo "TARGETNAME_SMALL=${BASENAME}-small.pdf" >> .env
