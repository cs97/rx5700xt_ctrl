# rx5700xt_ctrl
```
Usage: rx5700xt_ctrl [OPTION]
	Options:
	oc		set GPU to 2000Mhz 1050mV
	max		set GPU to 1900Mhz  985mV
	eco		set GPU to 1700MHz  875mV
```

### my rx 5700 xt VDDC_CURVE
```
OD_SCLK:
0: 800Mhz
1: 1900Mhz

OD_MCLK:
1: 875MHz

OD_VDDC_CURVE:
0: 800MHz 710mV
1: 1417MHz 806mV
2: 1900MHz 985mV
```


### good OD_VDDC_CURVE example:
```
1700MHz 875mV
1900MHz 985mV
2000MHz 1050mV
2150MHZ 1125mV
```
### install
/etc/default/grub
```
GRUB_CMDLINE_LINUX_DEFAULT="... amdgpu.ppfeaturemask=0xffffffff"
```
```
sudo grub-mkconfig -o /boot/grub/grub.cfg
```
```
cargo build --release
```
```
doas cp target/release/rx5700xt_ctrl /usr/bin/
```

/etc/systemd/system/rx5700_ctrl.service
```
[Unit]
Description=rx5700_ctrl

[Service]
Type=simple
ExecStart=/usr/bin/rx5700_ctrl eco
ExecStop=/usr/bin/rx5700_ctrl max

[Install]
WantedBy=multi-user.target
```
```
systemctl daemon-reload
```
```
systemctl enable rx5700_ctrl.service
```
```
systemctl start rx5700_ctrl.service
```

### auto sclk & mclk clock
```
echo "auto" > /sys/class/drm/card1/device/power_dpm_force_performance_level 
```

### max sclk & mclk clock
```
echo "profile_peak" > /sys/class/drm/card1/device/power_dpm_force_performance_level 
```

### on alpine linux change card0 to card1
```
/sys/class/drm/card1/device/pp_od_clk_voltage
```

