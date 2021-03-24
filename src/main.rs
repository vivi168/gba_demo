#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}

fn write_vram(data: u16, offset: isize) {
  unsafe {
    (0x600_0000 as *mut u16).offset(offset).write_volatile(data);
  }
}

fn write_io(data: u16, offset: isize) {
  unsafe {
    (0x400_0000 as *mut u16).offset(offset).write_volatile(data);
  }
}

fn clear_screen() {
  for i in 0..160*240 {
    write_vram(0, i);
  }
}

#[no_mangle]
extern "C" fn AgbMain() -> ! {
  unsafe {
    (0x4000_208 as *mut u16).write_volatile(1);
    (0x4000_200 as *mut u16).write_volatile(0x0001);
    (0x4000_004 as *mut u16).write_volatile(0x0008);
  }

  clear_screen();

  write_io(0x0403, 0);

  let mut i:u16 = 0;

  loop {
    write_vram(i, 240 * 160 / 2 + 240 / 2);
    i ^= 0xffff;
  }
}
