# Laptop Security Baseline — Arch Linux

## Disk Encryption
- Full disk encryption with LUKS2
- Encrypt swap partition
- Strong passphrase (20+ chars)

## Firewall — OpenSnitch
- Install: `yay -S opensnitch`
- Enable: `sudo systemctl enable --now opensnitchd`
- UI: `opensnitch-ui &`
- Default policy: DENY (prompt for new connections)
- Allow rules for known good: browsers, git, ssh, cargo, pacman
- Log all denied connections
- Review rules weekly

## OpenSnitch — Detailed Setup

### Install
```bash
yay -S opensnitch
sudo systemctl enable --now opensnitchd
```

### Default Rules
Create rules in `/etc/opensnitchd/rules/`:

- `allow-dns.json` — allow DNS (port 53, udp)
- `allow-browsers.json` — allow firefox, chromium
- `allow-git.json` — allow git, ssh (port 22, 443)
- `allow-cargo.json` — allow cargo, rustup
- `allow-pacman.json` — allow pacman, yay
- `allow-claude.json` — allow claude CLI

### Monitoring
- Check denied connections: `opensnitch-ui` or query SQLite at `/var/lib/opensnitch/stats.db`
- Review new rules weekly
- Netspectre agent will automate analysis (when built)

### Log Location
- SQLite database: `/var/lib/opensnitch/stats.db`
- Tables: `connections`, `rules`, `hosts`, `addrs`, `procs`
- Netspectre reads from this database for traffic analysis

## SSH Key Management
- Ed25519 keys only: `ssh-keygen -t ed25519`
- Passphrase on all keys
- Keys stored in ~/.ssh/ with 600 permissions
- SSH agent with timeout: `ssh-add -t 4h`
- ~/.ssh/config for host aliases

## Browser Hardening
- uBlock Origin
- HTTPS-only mode
- Disable WebRTC (or use extension to limit)
- Clear cookies on close for non-essential sites

## Package Management
- Verify GPG signatures: `SigLevel = Required DatabaseOptional` in /etc/pacman.conf
- Use official repos + AUR with caution
- Review PKGBUILD before installing AUR packages
- Keep system updated: `sudo pacman -Syu` weekly

## General
- Screen lock on idle (5 min)
- No auto-login
- Separate user for daily vs admin tasks (optional)
- Regular backups of ~/.config, ~/th33mptygh05t, ~/.ssh
