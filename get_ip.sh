#!/bin/bash
domains=("google.com" "github.com" "youtube.com" "discord.com" "soundcloud.com")
for domain in "${domains[@]}"; do
  ip=$(dig +short "$domain" | head -n1)
  echo "$domain —> $ip"
done