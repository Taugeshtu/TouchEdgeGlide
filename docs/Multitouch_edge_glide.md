#future

This is a bit of a hard problem. Basically, what we want to be, is a touchpad. So that libinput can process our events and synthesize correct gesture events for the compositor (and everybody else down the stack)
This looks roughly like this:
```
hardware touchpad
      ↓  (evdev / /dev/input/eventX)
libinput   ← compositor opens this directly, it's a LIBRARY not a daemon
      ↓  (libinput API calls, inside compositor process)
compositor (niri, sway, etc.)
      ↓  (Wayland protocol)
apps
```

With `evdev` we read on the level of hardware device. With `uinput` we present as  virtual device - alongside devices that `evdev` shows us. So conceptually we are at the right level!
But. We'll need a [[multi-loop]] design here. Because `evdev` does not give us facilities to read multiple touches (at least I haven't seen it??) in a polling manner, so we can't know _where_ our dirty little fingers are on the touchpad. And we need to, even for two-finger scrolling! Therefore, `fetch_events()` style polling, and somehow trying to retain the internal state of all the touches?..

Good news, we probably won't need to `grab()` the touchpad device into our sole proprietorship, and forward all the events out. But for glide events, we probably need a separate loop for emitting events. So fetching loop constructs/updates the internal model of the touches - how many, where; the emitter loop reads the internal model of touches and computes zones activation. And then emits events as a second, secret, virtual touchpad!