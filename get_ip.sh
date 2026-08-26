#!/bin/bash

#===Get all blocked domain. for example in russia ===
LIST_URL="https://raw.githubusercontent.com/runetfreedom/russia-blocked-geosite/main/data/community.lst"
LIST_FILE="knows_ip.txt"
LIST_IP="result.txt"
AGE_HOURS=6
#===Check of the dig===
if ! command -v dig &> /dev/null; then
    echo "dig is not found. install it..."
    sudo apt-get update && sudo apt-get install -y dnsutils
fi
#===Is the list of domain old?===
need_download=0
if [! -f "$LIST_FILE"]; then
    #It's mean that file doesn't exist. Download:
    need_download=1
else
    #The file is exist. Check his age:
    file_age=$(($(date +%s) - $(stat -c %Y "LIST_FILE")))
    AGE_SECOND=$((AGE_HOURS * 3600))
    if ["file_age" -gt "AGE_SECOND"]; then
        need_download=1
    fi
fi

for domain in "${domains[@]}"; do
  ip=$(dig +short "$domain" | head -n1)
  echo "$domain —> $ip"
done