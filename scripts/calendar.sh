#!/usr/bin/env bash
# calendar.sh - Generate calendar data for eww widget
# Usage: calendar.sh [month_offset]
# Output: JSON with header and weeks array

set -euo pipefail

# Month offset from current (default 0)
OFFSET=${1:-0}

# Calculate target month/year
TARGET_DATE=$(date -d "$(date +%Y-%m-01) ${OFFSET} months" +%Y-%m-%d)
YEAR=$(date -d "$TARGET_DATE" +%Y)
MONTH=$(date -d "$TARGET_DATE" +%m)
MONTH_NAME=$(LC_TIME=C date -d "$TARGET_DATE" +"%B %Y")

# Get today's date for highlighting
TODAY=$(date +%Y-%m-%d)

# First day of month (1=Mon, 7=Sun)
FIRST_DAY_OF_MONTH=$(date -d "$YEAR-$MONTH-01" +%u)

# Days in this month
DAYS_IN_MONTH=$(date -d "$YEAR-$MONTH-01 +1 month -1 day" +%d)

# Days in previous month (for filling start of grid)
PREV_MONTH=$(date -d "$YEAR-$MONTH-01 -1 month" +%Y-%m-%d)
DAYS_IN_PREV_MONTH=$(date -d "$PREV_MONTH +1 month -1 day" +%d)
PREV_MONTH_YM=$(date -d "$PREV_MONTH" +%Y-%m)

# Next month for filling end of grid
NEXT_MONTH_YM=$(date -d "$YEAR-$MONTH-01 +1 month" +%Y-%m)

# Build weeks array
WEEKS="["

# Calculate how many days from previous month to show
if [[ $FIRST_DAY_OF_MONTH -eq 1 ]]; then
    PREV_DAYS_TO_SHOW=0
else
    PREV_DAYS_TO_SHOW=$((FIRST_DAY_OF_MONTH - 1))
fi

# Starting day from previous month
PREV_START_DAY=$((DAYS_IN_PREV_MONTH - PREV_DAYS_TO_SHOW + 1))

# Current day counter for this month
DAY_COUNTER=1
NEXT_MONTH_DAY=1

for week in {0..5}; do
    if [[ $week -gt 0 ]]; then
        WEEKS+=","
    fi
    WEEKS+="["

    for dow in {0..6}; do
        if [[ $dow -gt 0 ]]; then
            WEEKS+=","
        fi

        CELL_INDEX=$((week * 7 + dow))

        if [[ $CELL_INDEX -lt $PREV_DAYS_TO_SHOW ]]; then
            # Previous month days
            DISPLAY_DAY=$((PREV_START_DAY + CELL_INDEX))
            CHECK_DATE="${PREV_MONTH_YM}-$(printf '%02d' $DISPLAY_DAY)"
            IS_TODAY=$([[ "$CHECK_DATE" == "$TODAY" ]] && echo "true" || echo "false")
            WEEKS+="{\"day\":\"$DISPLAY_DAY\",\"today\":$IS_TODAY,\"other_month\":true}"
        elif [[ $DAY_COUNTER -le $DAYS_IN_MONTH ]]; then
            # Current month days
            CHECK_DATE="${YEAR}-${MONTH}-$(printf '%02d' $DAY_COUNTER)"
            IS_TODAY=$([[ "$CHECK_DATE" == "$TODAY" ]] && echo "true" || echo "false")
            WEEKS+="{\"day\":\"$DAY_COUNTER\",\"today\":$IS_TODAY,\"other_month\":false}"
            ((DAY_COUNTER++))
        else
            # Next month days
            CHECK_DATE="${NEXT_MONTH_YM}-$(printf '%02d' $NEXT_MONTH_DAY)"
            IS_TODAY=$([[ "$CHECK_DATE" == "$TODAY" ]] && echo "true" || echo "false")
            WEEKS+="{\"day\":\"$NEXT_MONTH_DAY\",\"today\":$IS_TODAY,\"other_month\":true}"
            ((NEXT_MONTH_DAY++))
        fi
    done

    WEEKS+="]"

    # Stop after showing all current month days (need at least 5 rows)
    if [[ $DAY_COUNTER -gt $DAYS_IN_MONTH && $week -ge 4 ]]; then
        break
    fi
done

WEEKS+="]"

# Output JSON
echo "{\"header\":\"$MONTH_NAME\",\"weeks\":$WEEKS}"
