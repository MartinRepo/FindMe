#!/bin/bash

FORTUNE_FILE="$HOME/.findu_date"
TODAY=$(date +%Y-%m-%d)

# If fortune hasn't been displayed today, show it
if [ ! -f "$FORTUNE_FILE" ] || [ "$(cat "$FORTUNE_FILE")" != "$TODAY" ]; then
    echo ""
    echo "🎯 Getting today's tech fortune..."
    findu
    echo "$TODAY" > "$FORTUNE_FILE"
    echo ""
fi
