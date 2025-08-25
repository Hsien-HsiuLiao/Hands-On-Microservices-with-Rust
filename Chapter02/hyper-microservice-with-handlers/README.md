# Hyper 1.7.0 Microservice with Handlers - Vercel Deployment

This microservice demonstrates how to use hyper 1.7.0 with route handling and deploy it to Vercel as a serverless function.

## Features

- **hyper 1.7.0** - Modern HTTP library
- **Route handling** - Different responses for different paths
- **Vercel deployment** - Serverless function deployment
- **HTML responses** - Structured content with explanations

## Local Development

```bash
cargo run
```

Visit `http://localhost:8080` to see the service in action.

## Vercel Deployment

### Prerequisites

1. Install Vercel CLI:
```bash
npm i -g vercel
```

2. Login to Vercel:
```bash
vercel login
```

### Deploy

1. Deploy to Vercel:
```bash
vercel
```

2. For production deployment:
```bash
vercel --prod
```

### How It Works

- **`api/index.rs`** - Main serverless function
- **`vercel.json`** - Vercel configuration
- **Routes** - All `/api/*` requests are handled by the Rust function

### API Endpoints

- **`/`** - Returns HTML page with service information
- **Any other path** - Returns 404 Not Found

## Architecture

This microservice uses:
- **hyper 1.7.0** for HTTP handling
- **vercel_runtime** for serverless compatibility
- **tokio** for async runtime
- **Route matching** for different HTTP paths

## Benefits of Vercel Deployment

- **Serverless** - No server management needed
- **Global CDN** - Fast response times worldwide
- **Auto-scaling** - Handles traffic spikes automatically
- **Easy deployment** - Git-based deployments
- **Cost-effective** - Pay only for what you use 