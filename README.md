# wayidle
A simple wayland idle daemon

Use this command to schedule operations to run after the system been idle for a configured duration.

This program depends on the [Session lock](https://wayland.app/protocols/ext-session-lock-v1) wayland protocol, which is currently
in staging. Check that your compositor supports it before using this application.
