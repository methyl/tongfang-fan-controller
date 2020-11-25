use std::{fs::File};

use pid::Pid;
use std::os::unix::prelude::*;
use std::{thread, time};

#[macro_use]
extern crate nix;

ioctl_read!(read_fan, 0xEE, 0x10, usize);
ioctl_read!(read_temp, 0xEE, 0x12, usize);
ioctl_write_ptr!(set_enabled, 0xEF, 0x13, usize);
ioctl_write_ptr!(write_fan, 0xEF, 0x10, usize);

fn main() {
    let f = File::open("/dev/tuxedo_cc_wmi").unwrap();
    let fd = f.as_raw_fd();
    let mut pid = Pid::new(-1.0, -0.5, 0.0, 100.0, 100.0, 0.0, 80.0);

    unsafe {
        set_enabled(fd, &1).unwrap();

        let mut current_temp: usize = 0;

        read_temp(fd, &mut current_temp).expect(
            "Failed reading temp, report issue at https://github.com/methyl/tongfang-fan-control",
        );
        loop {
            read_temp(fd, &mut current_temp).expect("Failed reading temp, report issue at https://github.com/methyl/tongfang-fan-control");
            let output = pid.next_control_output(current_temp as f32);
            let clamped_output = output.output.max(0.0).min(100.0);

            let target_fan_speed = (0xc8 as f64 * clamped_output as f64 / 100.0).round() as usize;
            write_fan(fd, &target_fan_speed).expect("Failed writing fan, report issue at https://github.com/methyl/tongfang-fan-control");

            thread::sleep(time::Duration::from_millis(500));
        }
    }
}
