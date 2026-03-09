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
cargo install --path . --root ~/.local
```
This will produce a binary into your home directory: `~/.local/bin/touch-edge-glide`

_(if you'd rather not, you can use `cargo build --release` and find it at `TouchEdgeGlide/target/release/touch-edge-glide`)_

### making it run automatically on your system
(assuming the binary had been put in `~/.local/bin`)
The best way to have it go-go is with systemd. It will take care of restarting the daemon if it fails (for example, because touchpad got disconnected, or the system went to suspend). For example:
`code ~/.config/systemd/user/touch-edge-glide.service`:
```ini
[Unit]
Description=TouchEdgeGlide daemon; details on https://github.com/Taugeshtu/TouchEdgeGlide

[Service]
Type=simple
ExecStart=%h/.local/bin/touch-edge-glide
Restart=on-failure
RestartSec=2
StartLimitIntervalSec=30
StartLimitBurst=5

[Install]
WantedBy=default.target
```
And then start the service:
`systemctl --user enable touch-edge-glide --now`

# Configuration
Currently configuration is only possible via modifying the source in [[main.rs]]. External configuration files will come in v0.3.x

### determining touchpad range
For ease of configuration, the binary can be launched with `--monitor` argument. In this case it will report normalized (to 0..1 range) coordinates of the primary touch:
```
touch: x=0.05, y=0.93
touch: x=0.16, y=0.60
touch: x=0.16, y=0.33
touch: x=0.11, y=0.21
touch: x=0.10, y=0.13
touch: x=0.09, y=0.10
```
You can use these values to figure out where (0, 0) and (1, 1) on your specific touchpad are. _(note: in monitor mode TEG runs at a reduced update rate of 5Hz, as to not overwhelm you; normal mode updates at ~60Hz)_