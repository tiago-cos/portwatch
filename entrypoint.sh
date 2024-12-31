#!/bin/sh

OUTPUT_FILE=".env"

> "$OUTPUT_FILE"

env | while IFS= read -r line; do
  printf '%s\n' "$line" | sed 's/"/\\"/g' >> "$OUTPUT_FILE"
done

crond -f
