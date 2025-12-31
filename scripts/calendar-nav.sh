#!/usr/bin/env bash
# calendar-nav.sh - Navigate calendar months
# Usage: calendar-nav.sh [prev|next|reset]

MONTH_FILE="/tmp/eww-calendar-month"

# Get current offset
CURRENT=$(cat "$MONTH_FILE" 2>/dev/null || echo "0")

case "$1" in
    prev)
        echo $((CURRENT - 1)) > "$MONTH_FILE"
        ;;
    next)
        echo $((CURRENT + 1)) > "$MONTH_FILE"
        ;;
    reset)
        echo "0" > "$MONTH_FILE"
        ;;
esac
