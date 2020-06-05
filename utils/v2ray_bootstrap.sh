#!/bin/bash

ipset destroy v2rayallow
ipset flush v2rayallow
ipset create v2rayallow hash:ip hashsize 1024 maxelem 65536
ipset destroy cloudflareallow
ipset flush v2rayallow
ipset create cloudflareallow hash:net hashsize 1024 maxelem 65536
ipset add cloudflareallow 173.245.48.0/20
ipset add cloudflareallow 103.21.244.0/22
ipset add cloudflareallow 103.22.200.0/22
ipset add cloudflareallow 103.31.4.0/22
ipset add cloudflareallow 141.101.64.0/18
ipset add cloudflareallow 108.162.192.0/18
ipset add cloudflareallow 190.93.240.0/20
ipset add cloudflareallow 188.114.96.0/20
ipset add cloudflareallow 197.234.240.0/22
ipset add cloudflareallow 198.41.128.0/17
ipset add cloudflareallow 162.158.0.0/15
ipset add cloudflareallow 104.16.0.0/12
ipset add cloudflareallow 172.64.0.0/13
ipset add cloudflareallow 131.0.72.0/22
export EXPECT_AUTH_TOKEN=$(<EXPECT_AUTH_TOKEN)
RUST_LOG=info ./v2ray-fireopen