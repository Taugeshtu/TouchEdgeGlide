# TouchEdgeGlide
Adds "edge zones" to your touchpad; when there's touch happening inside a zone - your pointer is moved in the direction of that zone. Put simply: touch left edge of the touchpad -> cursor glides to the left. Move finger back from the edge, the glide stops. For Wayland.

### why
Because I'm a windows refuge and my laptop had that feature on windows, and I am missing it dearly on linux; especially in scenarios where I need to drag something across the screen. Yes, there's acceleration and I could try quick-flinging something and then slow down for more accurate positioning, or do that weird dance with using another finger... But the edge is right.there. We can use it! We have the technology!

### how
Grabs touchpad's current touch points from [evdev](https://github.com/emberian/evdev), when appropriate - emits [uinput](https://github.com/meh/rust-uinput) events. That simple.
Inspired by [fingerpaint](https://github.com/Wazzaps/fingerpaint), which showed me that absolute touchpad positioning events exist.

# Installation
### making sure uinput kernel module is loaded
run `sudo modprobe uinput`, if the output is empty - the module is loaded and everything is great! if it errors, you may need to enable it persistently - search for 'load kernel module on boot' for your distro.

### configuring access to the input device
Because TouchEdgeGlide reads and manipulates input, you have two options:
- run it as root
- udev rule

Because security, I highly recommend the second route - yes, it exposes access to device events for your user **and every process user starts**, but it only exposes a touchpad. Here's how you add a rule to dynamically allow access to touchpad device only for logged-in user:
_(inspired by [this bit of documentation from arch](https://wiki.archlinux.org/title/Udev#Allowing_regular_users_to_use_devices))_
- Find your touchpad's device path with `sudo libinput list-devices`
- Find its vendor/product name
- Write a rule matching that specific device
- Add the uaccess tag
into:
`sudo micro /etc/udev/rules.d/71-touchpad-gestures.rules`
```
ACTION!="remove", ATTRS{name}=="PUT_THE_NAME_OF_THE_TOUCHPAD_DEVICE_HERE", SUBSYSTEM=="input", TAG+="uaccess"
```
(this udev rule, matching not to kernel address, but rather to the device name, should be much more stable; if you match to kernel's handle like `/dev/input/event5` - be prepared that plugging in a USB stick and rebooting will break the enumeration)

&Reload udev:
```
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### getting the binary
Presently, you would have to build it yourself. For this you'd need [rust installed in your system](https://rust-lang.org/tools/install/) - rustup is an easy way to get there for most distros.
Then you'd need to compile the binary. That's easy:
``` bash
# navigate to where you want it to live, for example, ~/Applications/Gits
git clone https://github.com/Taugeshtu/TouchEdgeGlide
cd TouchEdgeGlide
cargo build --release
```
This will produce a binary: `TouchEdgeGlide/target/release/touch-edge-glide`

### making it run automatically on your system
You would have to research it yourself, but many wayland compositors have facilities for launching programs at startup.
For example, in [niri](https://github.com/niri-wm/niri) it looks like this:
```
spawn-at-startup "touch-edge-glide"
```
(assuming the binary had been put somewhere it can be seen via PATH, for example in `~/.local/bin`)