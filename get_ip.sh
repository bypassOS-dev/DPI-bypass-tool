#!/bin/bash
if ! command -v dig &> /dev/null; then
    echo "dig is not found. install it..."
    sudo apt-get update && sudo apt-get install -y dnsutils
fi

domains=("google.com" "github.com" "youtube.com" "discord.com" "soundcloud.com")

for domain in "${domains[@]}"; do
  ip=$(dig +short "$domain" | head -n1)
  echo "$domain —> $ip"
done