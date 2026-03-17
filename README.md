# COMELEC-CTUMC Minutes Management System

Why Cloudflare? The "Edge" Rationale
Standard hosting often suffers from latency, high maintenance costs, and security vulnerabilities. This system utilizes a Serverless Edge Architecture for three primary reasons:

# 1. Zero-Maintenance Infrastructure
COMELEC-CTUMC requires a system that "just works" without a dedicated sysadmin. Cloudflare Workers are serverless, meaning there is no OS to patch, no Docker containers to manage, and no virtual machines to reboot. The code runs directly on Cloudflare’s global network.

# 2. Physical Proximity & Latency
Instead of the data sitting in a single data center in Singapore or the US, the code is deployed to over 300+ cities globally. When a student leader at CTU Main accesses the minutes, the code executes at the Edge (likely in a Cebu or Manila point-of-presence), ensuring sub-millisecond response times even on campus Wi-Fi.

# 3. Immutable Security & Integrity
Election-related documents require high integrity.

Rust/Wasm: By using Rust instead of JavaScript, we eliminate common memory-based vulnerabilities (like buffer overflows) that hackers often exploit.

Isolation: Each request runs in a dedicated V8 Isolate, ensuring that one session cannot interfere with another’s memory space.

Runtime	WebAssembly (Wasm)	Allows Rust to run at near-native speeds in the browser/cloud environment.

Metadata	Workers KV	A globally distributed Key-Value store. Fast reads for meeting titles, attendees, and timestamps.

Files/Attachments	Cloudflare R2	S3-compatible object storage. Used for large files (PDFs/Images) because it has zero egress fees, saving the organization money.

# System Architecture

Request Ingress: A user (Secretariat/Chairman) sends a request to the API.

Worker Logic: The Rust code validates the request and checks the MOM_STORE (KV) for existing meeting data.

Stateless Stopwatch: To save CPU time and costs, the timer doesn't "run" in the background. It stores the start timestamp. The "Elapsed Time" is calculated dynamically only when requested.

Blob Storage: Binary files are streamed directly into R2, bypassing the memory limits of standard databases.

# Deployment & Initialization

Prerequisites

# Rust & Cargo

Cloudflare Wrangler CLI

1. Initialize Storage
Run the provided script to create your production namespaces:

`sh scripts/init_db.sh`

Configure Environment
Update your wrangler.toml with the IDs generated from the step above. This connects your code to the actual database.

Deploy to the Edge

`make deploy`

# API Reference Summary

POST /api/v1/meeting
Initializes a new session. Sets status to Open.

POST /api/v1/meeting/:id/timer/toggle
Starts or pauses the official stopwatch. Calculates total duration with microsecond precision.

PUT /api/v1/meeting/:id/file/:filename
Uploads official resolutions, attendance sheets, or audio recordings.

PATCH /api/v1/meeting/:id/attendees
Updates the list of present student leaders.








