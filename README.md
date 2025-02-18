# Battery-Notify

This is a simple program that notifies you when your battery is low. It should run in the background.

## Requirements
```bash
sudo pacman -S libnotify rustup
```

## Installation
```bash
curl -sSL https://raw.githubusercontent.com/Pazl27/battery-notify/master/scripts/install.sh | bash
```

## Uninstallation
```bash
curl -sSL https://raw.githubusercontent.com/Pazl27/battery-notify/master/scripts/uninstall.sh | bash
```

## Example with Hyprland
```conf
# ~/.config/hyprland/hyprland.conf
exec-once = battery-notify
```
