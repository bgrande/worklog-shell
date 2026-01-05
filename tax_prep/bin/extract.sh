#!/bin/bash

if [ -z "${TARGETPATH}" ]; then
  echo "missing base target path parameter"
  exit 1
fi

BASE=${TARGETPATH}
SAVEFILE=${TARGETPATH}/${TARGETPATH}.csv

find "$BASE"/* -type f \( -iname \*.json \) | while IFS= read -r file; do
  # if the conversion target does not already exists, we create
  if [ ! -f "${SAVEFILE}" ]; then
    echo "Ausgaben;Monat;Absetzbar (Lohnanteil) (brutto);Netto;Brutto gesamt;Kategorie;Bemerkung"> ${SAVEFILE}
  fi
  # now do stuff to save it
  # >> ${SAVEFILE}
done
