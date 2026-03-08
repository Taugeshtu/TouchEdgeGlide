# TouchEdgeGlide
Adds "edge zones" to your touchpad; when there's touch happening inside a zone - your pointer is moved in the direction of that zone. Put simply: touch left edge of the touchpad -> cursor glides to the left. Move finger back from the edge, the glide stops.

# Why
Because I'm a windows refuge and my laptop had that feature on windows, and I am missing it dearly on linux; especially in scenarios where I need to drag something across the screen. Yes, there's acceleration and I could try quick-flinging something and then slow down for more accurate positioning, or do that weird dance with using another finger... But the edge is right. there. We can use it! We have the technology!

# How
Watches evdev events, when appropriate - emits uinput events. That simple.
Inspired by [fingerpaint](https://github.com/Wazzaps/fingerpaint), which showed me that absolute touchpad positioning events exist.
