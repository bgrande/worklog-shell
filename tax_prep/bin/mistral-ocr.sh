#!/bin/bash

if ! [ -f .env ]; then
  echo "we need a .env file. please exec make setup!"
  exit 1
fi

source .env

if ! [ "$TARGETPATH" ]; then
  echo "Missing target path variable!"
  exit 1
fi

MISTRAL_API_KEY=$(cat mistral.key)

UPLOAD_RESULT=$(curl https://api.mistral.ai/v1/files \
  -H "Content-Type: multipart/form-data" \
  -H "Authorization: Bearer ${MISTRAL_API_KEY}" \
  -F file=@"${TARGETPATH}/${TARGETNAME_SMALL};filename=invoice.pdf" \
  -F purpose=ocr)

#echo ${UPLOAD_RESULT}

#UPLOAD_RESULT='{"id": "9bc1017b-9ed4-4edf-a121-063051a7a44c", "object": "file", "bytes": 104725, "created_at": 1749577090, "filename": "invoice.pdf", "purpose": "ocr", "sample_type": "ocr_input", "num_lines": 0, "mimetype": "application/pdf", "source": "upload", "signature": "c9c620dd9a277e39e6f2e980f3b7ebf2"}'

ID=$(echo ${UPLOAD_RESULT} | jq -r '.id')

FILEPATH=$(curl https://api.mistral.ai/v1/files/${ID}/url \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${MISTRAL_API_KEY}")

#FILEPATH='{"url": "https://mistralaifilesapiprodswe.blob.core.windows.net/fine-tune/a497992b-018f-4d56-bb1c-91f219229304/3be6bdf1-7e2d-478d-bc99-a67eb58dcdfb/9bc1017b9ed44edfa121063051a7a44c.pdf?se=2025-06-11T18%3A41%3A42Z&sp=r&sv=2025-05-05&sr=b&sig=573YGvOdw5SdXlW%2BbtL2fsvskVhi/WJRGIlaA8j/dmw%3D"}'

FILEURL=$(echo ${FILEPATH} | jq -r '.url')

echo "using fileurl ${FILEURL} with ID ${ID}"

LANGUAGE=german # todo will be dynamic

QUERY="The target file is an invoice in ${LANGUAGE}. Please extract the following data from the invoice: total amount (total_amount), vat amount (vat_amount), net amount (net_amount), the vat id (vat_id) and tax id (tax_id). Also add the address (issuer_address), name of the issuer (issuer_name) and its banking information if available. Based on the information extracted add a variable with the tax classification based on ${LANGUAGE} tax regulations (classification). Also, create a useful and valid linux filename (filename) based on the invoice information. The filename should not be longer than 60 characters including the .pdf extension. Save each dataset into a separate variable with the key names described above within round brackets and return the finding as JSON object. There can be multiple bank accounts, so use the json variable name 'issuer_banking_info' as an array with a list of objects with the json variable names bank_name, iban and bic for each object. Do not return anything else but the JSON object!"

RESULT=$(curl https://api.mistral.ai/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${MISTRAL_API_KEY}" \
  -d "{
    \"model\": \"mistral-small-latest\",
    \"messages\": [
      {
        \"role\": \"user\",
        \"content\": [
          {
            \"type\": \"text\",
            \"text\": \"${QUERY}\"
          },
          {
            \"type\": \"document_url\",
            \"document_url\": \"${FILEURL}\"
          }
        ]
      }
    ],
    \"document_image_limit\": 4,
    \"document_page_limit\": 10
  }")

echo "got result from request: "
echo ${RESULT}

DATA=$(echo ${RESULT} | jq '.choices[0]' | jq '.message' | jq '.content' | sed -r 's/(`)+//g' | sed -r 's/(json\\n)//g' | jq -r .)

FILENAME=$(echo ${DATA} | jq -r '.filename')

if ! echo $FILENAME | grep -q '.pdf'; then
  echo "Could not find filename. There might be an error!"
  exit 1
fi

OUTPATH=$(basename -s .pdf "${FILENAME}")

# is that the right output path?
echo "$DATA" > "$TARGETPATH"/"$OUTPATH".json
cp "${TARGETPATH}"/"${TARGETNAME_SMALL}" "${TARGETPATH}"/"${FILENAME}"