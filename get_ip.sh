#!/bin/bash

#===Get all blocked domain. for example in russia ===
LIST_URL="https://raw.githubusercontent.com/bypassOS-dev/qw/refs/heads/main/list_of_block_domain"
LIST_FILE="knows_ip.txt"
LIST_IP="result.txt"

#===Check of the dig===
if ! command -v dig &> /dev/null; then
    echo "dig is not found. install it..."
    sudo apt-get update && sudo apt-get install -y dnsutils
fi

# If file is exist then just remove it a download again
echo "Downloading fresh domain list..."
rm -f "$LIST_FILE" "$LIST_IP"
curl -s -o "$LIST_FILE" "$LIST_URL"

while read -r domain; do
    if [[ -z "$domain" || "$domain" == .* ]]; then
        continue
    fi
    (
        ip=$(dig +time=2 +tries=1 +short "$domain" | head -n1)
        if [ -n "$ip" ]; then
            echo "$domain = $ip" >> "$LIST_IP"
        fi
    ) &
done < "$LIST_FILE"

wait

echo -e "\e[30;42mScript finished. Done!\e[0m"
