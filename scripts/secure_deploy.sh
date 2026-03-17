#!/bin/bash
echo "Deploying Secure Worker to Cloudflare..."
wrangler deploy

echo "Setting Secret API Key..."
# This will prompt you to enter the secret in the terminal securely
wrangler secret put API_KEY
