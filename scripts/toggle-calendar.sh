#!/usr/bin/env bash
# toggle-calendar.sh - Toggle eww calendar widget visibility

CALENDAR_WINDOW="aura-calendar"

# Check if calendar window is currently open
if eww active-windows 2>/dev/null | grep -q "$CALENDAR_WINDOW"; then
    eww close "$CALENDAR_WINDOW"
    ~/.config/hypr/scripts/calendar-nav.sh reset
else
    ~/.config/hypr/scripts/calendar-nav.sh reset
    eww open "$CALENDAR_WINDOW"
fi
