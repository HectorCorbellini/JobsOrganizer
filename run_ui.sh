#!/usr/bin/env bash
# run_ui.sh - Attempts to launch the interactive UI, with robust checks for a graphical environment.

set -euo pipefail

# Check for a valid X11 display. `xset -q` will fail if no X server is available.
if ! xset -q &>/dev/null; then
    echo "ERROR: No graphical display found or X11 forwarding is not configured correctly." >&2
    echo "The interactive UI cannot be launched in a text-only environment." >&2
    echo "" >&2
    echo "To run the UI from a remote server, please connect using SSH with X11 forwarding:" >&2
    echo "  ssh -X user@your-server-address" >&2
    echo "" >&2
    echo "If you are running this in a local terminal, ensure your desktop environment is active." >&2
    exit 1
fi

echo "Graphical display found. Launching the UI..."
echo "Please note: The application window will open on the machine where the display is running."
echo "If you are using SSH with X11 forwarding, it may take a moment to appear."

# Run the application with the UI feature and pass the 'gui' subcommand.
cargo run --features ui -- gui
