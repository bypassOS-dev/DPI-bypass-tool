#!/bin/bash

#===Get all blocked domain. for example in russia ===
LIST_URL="https://raw.githubusercontent.com/runetfreedom/russia-blocked-geosite/main/data/community.lst"
LIST_FILE="knows_ip.txt"
LIST_IP="result.txt"

if ! command -v dig &> /dev/null; then
    echo "dig is not found. install it..."
    sudo apt-get update && sudo apt-get install -y dnsutils
fi

for domain in "${domains[@]}"; do
  ip=$(dig +short "$domain" | head -n1)
  echo "$domain —> $ip"
done