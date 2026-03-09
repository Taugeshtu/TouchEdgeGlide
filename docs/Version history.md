# Roadmap
#v0_2_x
- [ ] App is now a daemon that can be integrated into the system

#v0_3_x
- [ ] [[Configuration]] is a thing

# Done
#v0_1_2
- [x] Better internals - dedicated Zone struct, facilitating [[Configuration]] in the future
- [x] Support for gradient zone activation

#v0_1_1
- [x] Improved docs, allowing other humans to use the project (at least in principle)

#v0_1_0
- [x] Standalone Rust app
- [x] That initializes listening to a touchpad device on startup
- [x] That has a loop detecting absolute position events
- [x] Detects touch being over edge zones
- [x] Glides the cursor in the direction of the edge
