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
- Terminal: wezterm
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
- Border: 2px with gradient (purple to green, Aura theme)
- Window rounding: 10px

### Waybar Conventions
- Config: `waybar/config`, Style: `waybar/style.css`
- Floating bar with 8px top margin, 12px side margins, 14px border radius
- **Icon spacing**: Always use 2 spaces (≈8px) between icons and text (e.g., `"󰍛  {usage}%"`)
- Modules grouped with lighter background (`rgba(109, 109, 109, 0.15)`)
- Colors follow Aura theme: purple (clock), green (cpu, audio, network), blue (memory, bandwidth), orange (battery)

### Color Palette (Aura Theme)
```css
:root {
  --black: #15141b;
  --white: #edecee;
  --gray: #6d6d6d;
  --purple: #a277ff;
  --purple-fading: #3d375e7f;
  --green: #61ffca;
  --orange: #ffca85;
  --pink: #f694ff;
  --blue: #82e2ff;
  --red: #ff6767;

  --background-color: var(--black);
  --foreground-color: var(--white);
  --muted-color: var(--gray);
  --selection-color: var(--purple-fading);
  --primary-color: var(--purple);
  --secondary-color: var(--green);
  --tertiary-color: var(--orange);
  --quaternary-color: var(--pink);
  --quinary-color: var(--blue);
  --senary-color: var(--red);

  --warning-color: var(--orange);
  --success-color: var(--green);
  --error-color: var(--red);

  --black-soft: #15141b;
  --white-soft: #bdbdbd;
  --gray-soft: #6d6d6d;
  --purple-soft: #8464c6;
  --purple-fading-soft: #3d375e7f;
  --green-soft: #54c59f;
  --orange-soft: #c7a06f;
  --pink-soft: #c17ac8;
  --blue-soft: #6cb2c7;
  --red-soft: #c55858;

  --background-soft-color: var(--black-soft);
  --foreground-soft-color: var(--white-soft);
  --muted-soft-color: var(--gray-soft);
  --selection-soft-color: var(--purple-fading-soft);
  --primary-soft-color: var(--purple-soft);
  --secondary-soft-color: var(--green-soft);
  --tertiary-soft-color: var(--orange-soft);
  --quaternary-soft-color: var(--pink-soft);
  --quinary-soft-color: var(--blue-soft);
  --senary-soft-color: var(--red-soft);

  --warning-soft-color: var(--orange-soft);
  --success-soft-color: var(--green-soft);
  --error-soft-color: var(--red-soft);
}
```

## Validation

After editing, reload config:
```bash
hyprctl reload
```

Check for errors:
```bash
hyprctl systeminfo
```
