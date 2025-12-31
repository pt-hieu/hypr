#!/bin/bash
# Show dashboard only on workspace 1
# Uses separate eww config in ~/.config/hypr/eww-dashboard

EWW_DASHBOARD="eww --config $HOME/.config/hypr/eww-dashboard"

show_dashboard() {
    $EWW_DASHBOARD open dashboard 2>/dev/null
}

hide_dashboard() {
    $EWW_DASHBOARD close dashboard 2>/dev/null
}

# Get current workspace on start
current=$(hyprctl activeworkspace -j | jq -r '.id')
if [[ "$current" == "1" ]]; then
    show_dashboard
else
    hide_dashboard
fi

# Listen for workspace changes
socat -U - UNIX-CONNECT:"$XDG_RUNTIME_DIR/hypr/$HYPRLAND_INSTANCE_SIGNATURE/.socket2.sock" | while read -r line; do
    case "$line" in
        workspace\>\>1)
            show_dashboard
            ;;
        workspace\>\>*)
            hide_dashboard
            ;;
    esac
done
