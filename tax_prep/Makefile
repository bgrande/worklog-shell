#PROJECT ?= $(shell bash -c 'read -p "define a project name: " projectname; echo $$projectname')
#NAME ?= $(shell bash -c 'read -p "define a company name: " companyname; echo $$companyname')
GREEN :="\e[32m"
RED :="\e[31m"
BOLDGREEN="\e[1;${GREEN}m"
ITALICRED="\e[3;${RED}m"
ENDCOLOR :="\e[0m"

.PHONY: help
help: ## Show this help message
	@echo 'usage: make [target] ...'
	@echo
	@echo 'targets:'
	@sed -n 's/^\([a-z \-]*\):.*\(##.*\)/\1 \2/p' ${MAKEFILE_LIST} | column -t -c 2 -s ':#'

.PHONY: build_parser
build_parser:
	cargo build --release --bin $(PWD)/bin/parse

.PHONY: process
process: adf scan ocr-mistral cleanup ## scan, optimize and ocr a file via adf

.PHONY: process-duplex ## scan, optimize and ocr a file via duplex
process-duplex: duplex scan ocr-mistral ## scan, optimize and ocr a file

.PHONY: adf
adf: ## set scan type to adf
	echo "SCANTYPE=--adf" > .scantype

.PHONY: duplex
duplex: ## set scan type to duplex
	echo "SCANTYPE=--dup" > .scantype

.PHONY: scan
scan: ## scan adf
	$(PWD)/bin/hpscan.sh

.PHONY: scan-dup
scan-dup: ## scan duplex
	$(PWD)/bin/hpscan.sh dup

.PHONY: ocr
ocr: ## do ocr on current dir
	$(PWD)/bin/ocr.sh

.PHONY: ocr-lang
ocr-lang: ## do ocr on current dir with specific language
	$(PWD)/bin/ocr.sh $(LANGUAGE)

.PHONY: ocr-mistral
ocr-mistral: ## do ocr on current dir via mistral AI cloud API
	$(PWD)/bin/mistral-ocr.sh

.PHONY: convert
convert: ## convert scanned pdf files to tiff files
	$(PWD)/bin/convert_pdf_to_tiff.sh

.PHONY: optimize
optimize: ## optimize converted tiff files
	$(PWD)/bin/optimize_image.sh `ls $(TARGETPATH)/*.tiff`

.PHONY: parse
parse: ## parse the ocr file
	@shell bash $(PWD)/bin/parse

.PHONY: setup
setup: ## do setup for most important config
	$(PWD)/bin/setup.sh

.PHONY: cleanup
cleanup: ## cleanup the targetname
	#$(PWD)/bin/cleanup.sh