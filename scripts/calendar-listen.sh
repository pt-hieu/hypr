#!/usr/bin/env bash
# calendar-listen.sh - Persistent calendar data provider for eww deflisten
# Outputs calendar JSON on start and watches for file changes

MONTH_FILE="/tmp/eww-calendar-month"

# Initialize month offset
echo "0" > "$MONTH_FILE"

# Function to output calendar data
output_calendar() {
    local offset
    offset=$(cat "$MONTH_FILE" 2>/dev/null || echo "0")
    ~/.config/hypr/scripts/calendar.sh "$offset"
}

# Output initial calendar
output_calendar

# Watch for changes to month file using inotifywait
if command -v inotifywait &> /dev/null; then
    while inotifywait -qq -e modify "$MONTH_FILE" 2>/dev/null; do
        output_calendar
    done
else
    # Fallback: poll the file every second
    LAST_OFFSET=""
    while true; do
        CURRENT_OFFSET=$(cat "$MONTH_FILE" 2>/dev/null || echo "0")
        if [[ "$CURRENT_OFFSET" != "$LAST_OFFSET" ]]; then
            LAST_OFFSET="$CURRENT_OFFSET"
            output_calendar
        fi
        sleep 1
    done
fi
