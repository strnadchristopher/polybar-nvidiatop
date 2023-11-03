#Script: nvidiatop
A polybar script for monitoring Nvidia GPU usage.
## Dependencies
* nvidia-smi
* nvidia gpu
## Configuration
Nvidiatop currently supports the following command-line options:

```
--used-memory: Display used memory as percentage
--used-memory-mb: Display used memory in MB
--free-memory: Display free memory as percentage
--free-memory-mb: Display free memory in MB
--total-memory: Display total memory in MB
--clock-speed: Display current clock speed in MHz
--max-boost-clock-speed: Display your gpu's set max boost clock speed in MHz
```

## Module
```
[module/nvidiatop]
type = custom/script
format-prefix = "󰍛 "
exec = ~/GitHub/polybar-nvidiatop/target/release/polybar-nvidiatop --clock-speed
interval = 3
```