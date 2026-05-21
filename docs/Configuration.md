# Configuration

TouchEdgeGlide uses a TOML file for configuration. On its first run, the daemon will create a default configuration file at:
`~/.config/touch-edge-glide/config.toml`

## "Either-or" Logic
To keep things simple, the configuration supports two mutually exclusive modes:

1.  **Generic**: A single set of parameters applied to all four edges.
2.  **Edges**: Specific parameters for each edge (`left`, `right`, `top`, `bottom`). 

**Note**: If the `[edges]` section is present in your config, the `[generic]` section is completely ignored.

## Parameters

### `speed`
- **Type**: Float
- **Description**: How fast the pointer moves (in pixels per frame) when the glide is fully active.
- **Example**: `speed = 5.0`

### `zone_size`
- **Type**: Float (0.0 to 1.0)
- **Description**: The thickness of the edge zone, expressed as a fraction of the touchpad's total width/height. 
- **Example**: `0.15` means the glide starts when your finger is within the outer 15% of the touchpad.

### `full_speed_at`
- **Type**: Float (0.0 to 1.0)
- **Description**: The point where the glide reaches its maximum speed. 
- **Ramping**: If `full_speed_at` is smaller than `zone_size`, the speed will ramp up linearly as you move closer to the physical edge. 
- **Sharp Activation**: If you set `full_speed_at` equal to `zone_size`, the glide will trigger at full speed instantly upon entering the zone.
- **Example**: `full_speed_at = 0.05`

## Example Configuration

```toml
# Use generic settings for all edges
[generic]
speed = 5.0
zone_size = 0.15
full_speed_at = 0.05

# OR: Uncomment this to use per-edge settings
# [edges]
# left   = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
# right  = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
# top    = { speed = 8.0, zone_size = 0.20, full_speed_at = 0.10 }
# bottom = { speed = 5.0, zone_size = 0.15, full_speed_at = 0.05 }
```
