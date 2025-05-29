#!/bin/bash

# This script helps deploy the website to Netlify

# Check if Netlify CLI is installed
if ! command -v netlify &> /dev/null
then
    echo "Netlify CLI is not installed. Installing..."
    npm install -g netlify-cli
fi

# Deploy to Netlify
echo "Deploying to Netlify..."
netlify deploy --dir=. --prod

echo "Deployment complete!"
