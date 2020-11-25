Fan controller for Schenker VIA 15 Pro / Mechrevo Code 01 / Tuxedo Pulse and other TongFang PF5NU1G based laptops.

Uses hand-tweaked PID controller to determine fan speed.

# Requirements

Requires [tuxedo-cc-wmi](https://github.com/tuxedocomputers/tuxedo-cc-wmi) kernel module.

[AUR Package](https://aur.archlinux.org/packages/tuxedo-cc-wmi/) can be installed with `yay -S tuxedo-cc-wmi`

# Installation

```
cargo build --release
sudo cp target/release/tongfang-fan-control /usr/local/bin
sudo nano /etc/systemd/system/tongfang-fan-control.service
# paste following
[Unit]
Description=Run Tongfang Fan Control

[Service]
Type=simple
Restart=always
ExecStart=/usr/local/bin/tongfang-fan-control
User=root

[Install]
WantedBy=multi-user.target

sudo systemctl enable tongfang-fan-control
sudo systemctl start tongfang-fan-control
```
