#!/bin/bash
wrangler kv:namespace create MOM_STORE
wrangler r2 bucket create meeting-attachments
echo "Storage initialized. Update wrangler.toml with the IDs above."
