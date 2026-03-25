#!/bin/bash
NEW_KEY=$(openssl rand -base64 32)
echo "Rotating COMELEC API Key..."
wrangler secret put API_KEY <<EOF
$NEW_KEY
EOF
echo "New Key generated and deployed to Cloudflare Edge."
