# Loops design
Two choices:
- ![[single loop]]
- ![[multi-loop]]

# Polling vs fetch_events
Choice between [polling](https://docs.rs/evdev/latest/evdev/struct.Device.html#method.get_abs_state) and [fetch_events](https://docs.rs/evdev/latest/evdev/struct.Device.html#method.fetch_events) is a balance between "how much pain do you want from trying to maintain coherent internal state in the face of [possibility that kernel just flushes the event queue](https://docs.rs/evdev/latest/evdev/index.html#synchronizing-versus-raw-modes)" vs "simplicity of often burning CPU cycles".
Goldilocks solution would probably look like "10Hz polling until touch is detected, then kick into high-gear for responsive experience, then go low-rate when touch is gone". Maybe in #v0_4_x.
How big of a deal is it, honestly? I don't know. Should probably look at used CPU time in `htop` from a fresh reboot, compare to other daemons and apps...

# Dynamic devices
It is possible that touchpads are connected/disconnected dynamically (USB touchpad, for example). Should we cover this case?
For now, we ignore that scenario, but we have thought about it! Go us.
After a bit of contemplation, I've arrived at this:
- we will be a systemd daemon
- we will throw non-zero exit codes if there are system/envionment issues (trackpad lost, couldn't bring up uinput device, stuff like that)
- we will let systemd restart us
This looks simple, sane, and sufficient. IF we suddenly have many users complaining that their system going suspended breaks the daemon, then we can consider wrapping device-polling loop in a device-management loop that should handle hotplug gracefully. But I don't expect that to be a massive need because it's just edge gliding, it's not the core touchpad functionality - probably can wait 2-3 seconds for systemd to bring us back up?..

# Progressive glide
Some time in the future it would probably be nice to have control over how fast the pointer is gliding. Ramp can go into [[Configuration]]
As of #v0_1_2 we have this behavior.

# [[Configuration]]
For initial version no configuration means (other than constants in the source); for #v0_3_x or something we should have that. At the very least we'd want configuration for the edge thresholds
