---
title: "From Netlify Subdomain to Custom Domain: My Journey with GoDaddy"
slug: netlify-godaddy-domain-guide
authors: [Sheetal]
tags: [hosting, netlify, godaddy, domain, dns, webdev]
---

# Connecting My GoDaddy Domain to Netlify: A Step-by-Step Guide

## Introduction

I recently found myself in a common situation: I had my portfolio site live on Netlify at `sheetalssr.netlify.app`, and I'd just purchased a custom domain (`sheetal.me`) from GoDaddy. This post documents my journey through the process and the interesting things I learned about web hosting and DNS along the way.

## Understanding Web Hosting

### Types of Hosting

- **Shared Hosting**: Multiple sites share server resources. Cost-effective but performance can be inconsistent.
- **VPS (Virtual Private Server)**: Dedicated virtual machine resources on a shared physical server.
- **Dedicated Hosting**: Your own physical server. Maximum control but requires technical expertise.
- **Cloud Hosting**: Distributed across multiple servers for better scalability and reliability.
- **Serverless Architecture**: Pay-per-use model where the cloud provider manages the infrastructure.

### Where Netlify Fits In

Netlify specializes in static site hosting with a serverless approach. It's perfect for:

- Portfolio websites
- Blogs
- Marketing sites
- Documentation

## The DNS Connection

### How DNS Works

Think of DNS as the internet's phonebook. When someone types `sheetal.me`:

1. Their browser asks a DNS resolver for the IP address
2. The resolver checks with root nameservers
3. The request is directed to GoDaddy's nameservers
4. The IP address is returned and the site loads

## Connecting GoDaddy to Netlify

### Step 1: Set Up in Netlify

1. Log into your Netlify dashboard
2. Go to Site settings > Domain management
3. Add your custom domain (`sheetal.me`)
4. Netlify will provide DNS records to verify ownership

### Step 2: Configure GoDaddy DNS

1. Log into your GoDaddy account
2. Navigate to your domain's DNS settings
3. Add Netlify's provided records:
   - A record: `@` points to Netlify's IP
   - CNAME: `www` points to your Netlify URL

### Step 3: Choose Your Redirect Type

Netlify offers three options:

- **Permanent (301)**: Best for permanent moves (recommended)
- **Temporary (302)**: For testing or temporary redirects
- **Forward with masking**: Hides the destination URL

## The Waiting Game

DNS changes can take up to 48 hours to propagate globally. Here's why:

- DNS records are cached by ISPs
- TTL (Time To Live) settings affect how long old records persist
- Some users might see changes before others

## Troubleshooting Tips

If your site isn't loading:

1. Double-check DNS records for typos
2. Verify Netlify's SSL certificate is active
3. Use [whatsmydns.net](https://www.whatsmydns.net/) to check propagation
4. Clear your browser cache or try incognito mode

## Final Thoughts

Connecting a GoDaddy domain to Netlify is straightforward once you understand the DNS handshake. The key is patience while changes propagate. Now, when someone visits `sheetal.me`, they'll see my portfolio hosted on Netlify!
