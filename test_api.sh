#!/bin/bash
API_URL="https://comelec-ctumc-api.your-subdomain.workers.dev"

echo "1. Creating Meeting..."
curl -X POST "$API_URL/api/v1/meeting" \
     -H "Content-Type: application/json" \
     -d '{"id":"CTU-2026-001", "title":"Election Code Review", "attendees":["Comm. Juan", "Comm. Maria"], "meeting_type":"Plenary"}'

echo -e "\n2. Starting Timer.."
curl -X POST "$API_URL/api/v1/meeting/CTU-2026-001/timer/toggle"

echo -e "\n3. Uploading File 1 of 15..."
echo "Sample Content" > resolution.pdf
curl -X PUT "$API_URL/api/v1/meeting/CTU-2026-001/file/resolution.pdf" --data-binary @resolution.pdf

echo -e "\n4. Finalizing Meeting (Locking)..."
curl -X POST "$API_URL/api/v1/meeting/CTU-2026-001/finalize"

echo -e "\n5. Verifying Lock (Should fail to toggle timer)..."
curl -X POST "$API_URL/api/v1/meeting/CTU-2026-001/timer/toggle"
