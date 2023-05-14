# power-profiles-cfg

Configurable power profiles for power-profiles-daemon

## Why?
Many tools exist for configuring laptops based on being plugged in to AC or running on battery, however, there isn't any tool that lets you configure settings based on the power plan.

That's why I wrote this, which is a companion to power-profiles-daemon (the closest thing to a standard for setting power plans).

## Installation

```
./configure
make
sudo make install
```

## Uninstall

```
sudo make uninstall
```

## Configuration file
The configuration file is located at `/etc/power-profiles-cfg/profiles.ron`

Initially, the program will query `power-profiles-daemon` for the list of available profiles and populates the file accordingly.

### Example configuration
```
{
  "power-saver": (
    driver: "platform_profile",
    turbo: false,
    governor: "conservative",
  ),
  "balanced": (
    driver: "platform_profile",
    turbo: false,
    governor: "ondemand",
  ),
  "performance": (
    driver: "platform_profile",
    turbo: true,
    governor: "performance",
  ),
}
```

### Available settings
Currently, you're only able to set turbo boost and change the governor per profile.

#### Governor
You can get a list of available governors by running `cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors`.
Any governor from that list can be set.

I plan to add more in the future (CPU frequency, threshold, etc).

I may extend this in the future to also allow differing profiles for AC/battery power (i.e `balanced` for AC and battery could be different).

## GUI?
A GUI companion is on my to do list, but I will be focusing on getting the initial features implemented for the daemon.
