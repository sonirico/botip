# 🤖 BotIP - What's My IP Address?

**The fast, reliable, and free IP address detection service for bots, scripts, and humans.**

[![Website](https://img.shields.io/website?url=http%3A%2F%2Fbotwhatismyipaddress.com)](http://botwhatismyipaddress.com)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🌐 Live Service

**[botwhatismyipaddress.com](http://botwhatismyipaddress.com)**

A spiritual successor to the beloved `bot.whatismyipaddress.com` that unfortunately went offline due to abuse. This service fills that gap with improved performance and reliability.

### For Bots & Scripts

```bash
# Plain text (perfect for scripts)
curl botwhatismyipaddress.com

# IPv6 only
curl -6 botwhatismyipaddress.com

# IPv4 only
curl -4 botwhatismyipaddress.com

# JSON format with detailed info
curl -H "Accept: application/json" botwhatismyipaddress.com
# Returns: {"ip":"1.2.3.4","version":4,"type":"global"}

# Force plain text with query parameter
curl botwhatismyipaddress.com?bot
```

### For Browsers

Just visit [botwhatismyipaddress.com](http://botwhatismyipaddress.com) and you'll see a nice HTML page with your IP address (works with both IPv4 and IPv6).

### Plain Text
```bash
$ curl botwhatismyipaddress.com
192.168.1.100
```

### JSON
```bash
$ curl -H "Accept: application/json" botwhatismyipaddress.com
{
  "ip": "192.168.1.100",
  "version": 4,
  "type": "global"
}
```

**IP Types:** `global`, `private`, `loopback`, `link-local`, `multicast`

## 🛠️ Built With

- **[Rust](https://www.rust-lang.org/)** - For speed and safety
- **[Actix-web](https://actix.rs/)** - High-performance async web framework
- **IPv4 & IPv6** - Full dual-stack support
- **Compression** - Gzip/Brotli support built-in
- **Smart Caching** - Optimized for millions of requests

## Support This Project

Running a free, unlimited service costs money (servers, bandwidth, maintenance). If you find this useful, please consider supporting:

**[☕ Buy me a coffee on Ko-fi](https://ko-fi.com/sonirico)**

Your support helps keep this service free and fast for everyone!

## Contributing

This is open source! Found a bug? Have an idea? PRs are welcome.

## License

MIT License - Use it however you want!

## Acknowledgments

In memory of `bot.whatismyipaddress.com` - you served us well. This is our tribute to you.

