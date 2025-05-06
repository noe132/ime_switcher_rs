use core::time;
use objc2_core_foundation::{CFMachPort, CFRunLoop, kCFRunLoopCommonModes};
use objc2_core_graphics::{
  CGEvent, CGEventField, CGEventFlags, CGEventMask, CGEventSource, CGEventSourceStateID,
  CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventTapProxy, CGEventType,
};
use std::{ops::Deref, os::raw::c_void, ptr::NonNull};

mod keycodes;

pub struct DeviceMask {}

impl DeviceMask {
  pub const LCTRL: CGEventFlags = CGEventFlags(1);

  pub const LCTLKEYMASK: CGEventFlags = CGEventFlags(0x00000001);
  pub const LSHIFTKEYMASK: CGEventFlags = CGEventFlags(0x00000002);

  pub const RSHIFTKEYMASK: CGEventFlags = CGEventFlags(0x00000004);
  pub const LCMDKEYMASK: CGEventFlags = CGEventFlags(0x00000008);
  pub const RCMDKEYMASK: CGEventFlags = CGEventFlags(0x00000010);
  pub const LALTKEYMASK: CGEventFlags = CGEventFlags(0x00000020);
  pub const RALTKEYMASK: CGEventFlags = CGEventFlags(0x00000040);
  pub const ALPHASHIFT_STATELESS_MASK: CGEventFlags = CGEventFlags(0x00000080);
  pub const RCTLKEYMASK: CGEventFlags = CGEventFlags(0x00002000);
}

const MODIFIER_CHECK_MASK: CGEventMask = DeviceMask::LCTLKEYMASK.0
  | DeviceMask::LSHIFTKEYMASK.0
  | DeviceMask::RSHIFTKEYMASK.0
  | DeviceMask::LCMDKEYMASK.0
  | DeviceMask::RCMDKEYMASK.0
  | DeviceMask::LALTKEYMASK.0
  | DeviceMask::RALTKEYMASK.0
  | DeviceMask::RCTLKEYMASK.0;

const LCMD_LSHIFT_MASK: CGEventMask = DeviceMask::LCMDKEYMASK.0 | DeviceMask::LSHIFTKEYMASK.0;

fn cg_event_mask_bit(event_type: CGEventType) -> CGEventMask {
  1 << event_type.0
}

static mut EXTRA_KEY_DOWN: bool = false;
static mut MODIFIERS: u64 = 0;

fn handle_event(event_type: CGEventType, event: &CGEvent) {
  unsafe {
    if event_type == CGEventType::FlagsChanged {
      let new_flags = CGEvent::flags(Some(event)).0 & MODIFIER_CHECK_MASK;
      let new_flags_extra_keys = (new_flags & !LCMD_LSHIFT_MASK) != 0;
      let lcmd_or_lshift =
        new_flags == DeviceMask::LCMDKEYMASK.0 || new_flags == DeviceMask::LSHIFTKEYMASK.0;
      if new_flags_extra_keys {
        EXTRA_KEY_DOWN = true;
      }
      let previous_is_lcmd_lshift = MODIFIERS == LCMD_LSHIFT_MASK;

      if !EXTRA_KEY_DOWN && previous_is_lcmd_lshift && lcmd_or_lshift {
        switch_ime_by_simulating();
      }
      if new_flags == 0 {
        EXTRA_KEY_DOWN = false;
      }
      MODIFIERS = new_flags;
    } else if event_type == CGEventType::KeyUp {
      // let key: i64 = i64::try_from(CGEvent::integer_value_field(
      //   Some(event),
      //   CGEventField::KeyboardEventKeycode,
      // ))
      // .unwrap();

      // let mut keys = KEYS.lock().unwrap();
      // keys.retain(|v| *v != key);
      if MODIFIERS == 0 {
        EXTRA_KEY_DOWN = false;
      }
      // println!("up: {}", key);
    } else if event_type == CGEventType::KeyDown {
      let is_repeat =
        CGEvent::integer_value_field(Some(event), CGEventField::KeyboardEventAutorepeat);
      if is_repeat != 0 {
        return;
      }
      // let key = CGEvent::integer_value_field(Some(event), CGEventField::KeyboardEventKeycode);
      // println!("down: {}", key);
      // let mut keys = KEYS.lock().unwrap();
      // keys.push(key);
      EXTRA_KEY_DOWN = true;
    }
  }
}

unsafe extern "C-unwind" fn callback(
  _proxy: CGEventTapProxy,
  event_type: CGEventType,
  event: NonNull<CGEvent>,
  _info: *mut c_void,
) -> *mut CGEvent {
  unsafe {
    handle_event(event_type, event.as_ref());
    let a = event.as_ptr();
    a
  }
}

fn switch_ime_by_simulating() {
  unsafe {
    let src_ref = CGEventSource::new(CGEventSourceStateID::HIDSystemState);
    let src = src_ref.as_deref();
    let key_down_ref = CGEvent::new_keyboard_event(src, keycodes::kVK_ANSI_Equal, true);
    let key_down = key_down_ref.as_deref();
    let key_up_ref = CGEvent::new_keyboard_event(src, keycodes::kVK_ANSI_Equal, false);
    let key_up = key_up_ref.as_deref();

    CGEvent::set_flags(
      key_down,
      CGEventFlags::MaskCommand | CGEventFlags::MaskShift,
    );
    CGEvent::set_flags(key_up, CGEventFlags::MaskCommand | CGEventFlags::MaskShift);

    let loc = CGEventTapLocation::HIDEventTap;
    CGEvent::post(loc, key_down);
    CGEvent::post(loc, key_up);

    // drop as fall out of scope
    // drop(src_ref.unwrap());
    // drop(key_down_ref.unwrap());
    // drop(key_up_ref.unwrap());
  }
}

fn init() {
  let key_event_mask: CGEventMask = cg_event_mask_bit(CGEventType::FlagsChanged)
    | cg_event_mask_bit(CGEventType::KeyDown)
    | cg_event_mask_bit(CGEventType::KeyUp);

  unsafe {
    loop {
      let m_event_tap_ptr = CGEvent::tap_create(
        CGEventTapLocation::SessionEventTap,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        key_event_mask,
        Some(callback),
        std::ptr::null_mut(),
      );
      std::thread::sleep(time::Duration::from_secs(1));
      if m_event_tap_ptr != None {
        return;
      }
    }
  }
}

fn main() {
  init();

  let key_event_mask: CGEventMask = cg_event_mask_bit(CGEventType::FlagsChanged)
    | cg_event_mask_bit(CGEventType::KeyDown)
    | cg_event_mask_bit(CGEventType::KeyUp);

  unsafe {
    let m_event_tap_ptr = CGEvent::tap_create(
      CGEventTapLocation::SessionEventTap,
      CGEventTapPlacement::HeadInsertEventTap,
      CGEventTapOptions::Default,
      key_event_mask,
      Some(callback),
      std::ptr::null_mut(),
    );
    let m_event_tap_ptr = m_event_tap_ptr.as_deref();
    let current_run_loop = CFRunLoop::current().expect("");
    let current_run_loop = current_run_loop.deref();
    let loop_source = CFMachPort::new_run_loop_source(None, m_event_tap_ptr, 0).expect("");
    let loop_source = loop_source.deref();
    CFRunLoop::add_source(current_run_loop, Some(loop_source), kCFRunLoopCommonModes);

    CGEvent::tap_enable(m_event_tap_ptr.expect(""), true);
    std::println!("runLoop");
    CFRunLoop::run();
  }
}
