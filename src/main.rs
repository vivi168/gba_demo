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
  clear_screen();

  write_vram(0xf1, 240 * 160 / 2 + 240 / 2);
  write_io(0x0403, 0);

  loop {}
}

#[no_mangle]
static __IRQ_HANDLER: extern "C" fn() = irq_handler;

extern "C" fn irq_handler() {}
