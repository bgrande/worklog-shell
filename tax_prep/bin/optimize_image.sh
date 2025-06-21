#!/bin/bash

# Überprüfe ob die benötigten Programme installiert sind
command -v convert >/dev/null 2>&1 || { echo "ImageMagick wird benötigt. Bitte installieren Sie es mit: sudo apt-get install imagemagick"; exit 1; }
command -v convertformat >/dev/null 2>&1 || { echo "Leptonica wird benötigt. Bitte installieren Sie es mit: sudo apt-get install leptonica-progs"; exit 1; }

# Funktion zur Bildoptimierung
optimize_image() {
    local input_file="$1"
    local output_file="${input_file%.*}_optimized.${input_file##*.}"

    # Erstelle ein temporäres Verzeichnis
    temp_dir=$(mktemp -d)

    # Kopiere das Originalbild
    cp "$input_file" "$temp_dir/temp.${input_file##*.}"

    # Bildoptimierung mit ImageMagick
    convert "$temp_dir/temp.${input_file##*.}" \
        -strip \
        -interlace Plane \
        -gaussian-blur 0.05 \
        -quality 85% \
        "$temp_dir/temp_compressed.${input_file##*.}"

    # Optional: Leptonica für zusätzliche Optimierung
    if [[ "${input_file,,}" =~ \.(png|jpg|jpeg)$ ]]; then
        convertformat "$temp_dir/temp_compressed.${input_file##*.}" \
            "$temp_dir/temp_final.${input_file##*.}"
    else
        cp "$temp_dir/temp_compressed.${input_file##*.}" \
           "$temp_dir/temp_final.${input_file##*.}"
    fi

    # Verschiebe das finale Bild
    mv "$temp_dir/temp_final.${input_file##*.}" "$output_file"

    # Lösche temporäres Verzeichnis
    rm -rf "$temp_dir"

    echo "Optimiertes Bild gespeichert als: $output_file"
}

# Hauptprogramm
if [ "$#" -eq 0 ]; then
    echo "Verwendung: $0 <Bilddatei1> [Bilddatei2 ...]"
    exit 1
fi

# Verarbeite alle übergebenen Dateien
for file in "$@"; do
    if [ -f "$file" ]; then
        echo "Optimiere $file..."
        optimize_image "$file"
    else
        echo "Warnung: Datei $file nicht gefunden"
    fi
done