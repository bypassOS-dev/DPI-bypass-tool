#!/bin/bash

#===Get all blocked domain. for example in russia ===
LIST_URL="https://raw.githubusercontent.com/itdoginfo/allow-domains/refs/heads/main/Russia/inside-raw.lst"
LIST_FILE="knows_ip.txt"
LIST_IP="result.txt"

#===Check of the dig===
if ! command -v dig &> /dev/null; then
    echo "dig is not found. install it..."
    sudo apt-get update && sudo apt-get install -y dnsutils
fi
#
need_download=0
if [ ! -f "$LIST_FILE" ]; then
    #It's mean that file doesn't exist. Download:
    need_download=1
fi
#download list (if it's need)
if [ "$need_download" -eq 1 ]; then
    echo "File is don't download. Download fresh version..."
    curl -s -o "$LIST_FILE" "$LIST_URL"
else 
    echo "File is exist!"
fi
#clean old result
> "$LIST_IP"

while read -r domain; do
    ip=$(dig +short "$domain" | head -n1)
    echo "$domain = $ip" >> "$LIST_IP"
done < "$LIST_FILE"