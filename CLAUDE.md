# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Hyprland configuration for a Wayland compositor. Single config file setup (no split configs).

## Configuration

- **Main config**: `hyprland.conf`
- **Wiki**: https://wiki.hypr.land/Configuring/

## Current Setup

### Hardware
- Monitor: DP-1 ultrawide (3440x1440@144Hz, HDR, 10-bit)
- GPU: NVIDIA (Wayland-specific env vars configured)

### Programs
- Terminal: kitty
- File manager: dolphin
- Launcher: wofi
- Bar: waybar
- Notifications: dunst
- Wallpaper: hyprpaper

### Key Bindings (SUPER = mainMod)
| Binding | Action |
|---------|--------|
| SUPER+Q | Terminal |
| SUPER+C | Kill window |
| SUPER+M | Exit Hyprland |
| SUPER+E | File manager |
| SUPER+R | App launcher |
| SUPER+V | Toggle floating |
| SUPER+S | Scratchpad |
| SUPER+1-0 | Workspaces 1-10 |
| SUPER+SHIFT+1-0 | Move to workspace |
| SUPER+arrows | Move focus |

### Layout
- Dwindle layout with pseudotiling enabled
- Gaps: 5px inner, 20px outer
- Border: 2px with gradient (cyan to green)
- Window rounding: 10px

## Validation

After editing, reload config:
```bash
hyprctl reload
```

Check for errors:
```bash
hyprctl systeminfo
```
