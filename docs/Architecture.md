I am picking [[single loop]] because it's so much easier and cleaner ([[Considerations#Loops design]])

# State
- Four [[Zone]]s
- whatever is needed to support evdev and uinput

# Operating principle:
- Start listening to [[evdev events]]
- Update zones' activation level based on new absolute positioning data
- iterate through zones and accumulate total glide delta

We can implement [[Considerations#Progressive glide]] on the side of [[pointer move events]] - there would be something like "delta" on axes that we can scale.