# WIP
Designed for the stm32h755 or other stm32h7x microcontroller's.

This uses probe-rs, if you are using wsl, you need to follow this [manual](https://devblogs.microsoft.com/commandline/connecting-usb-devices-to-wsl/):

1. In Ubuntu
```
sudo apt install linux-tools-5.4.0-77-generic hwdata
sudo update-alternatives --install /usr/local/bin/usbip usbip /usr/lib/linux-tools/5.4.0-77-generic/usbip 20

# install probe-rs
# config rules
```

2. In powershell (ADMIN)
```
usbipd wsl list
usbipd wsl attach --busid <busid (stlink)>  # Removes access from windows and adds access to ubuntu.
usbipd wsl detach --busid <busid (stlink)>  # Removes access from ubuntu and adds access to windows.
```