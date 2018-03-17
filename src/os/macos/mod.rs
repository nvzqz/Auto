//! macOS-specific functionality.

#![allow(non_snake_case)]

use std::mem;
use std::os::raw;

use objc::runtime::Class;
use objc::{Encode, Encoding};

#[link(name = "Cocoa", kind = "framework")]
extern {
    fn CFRelease(_: *mut raw::c_void);

    fn CGEventPost(tap_location:raw::c_int, event: CGEvent);
}

pub mod mouse;

lazy_static! {
    static ref NS_EVENT: &'static Class = Class::get("NSEvent").unwrap();
}

cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        type CGFloat = raw::c_double;
    } else {
        type CGFloat = raw::c_float;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct CGPoint {
    x: CGFloat,
    y: CGFloat,
}

impl From<CGPoint> for (f64, f64) {
    #[inline]
    fn from(point: CGPoint) -> (f64, f64) {
        (point.x as _, point.y as _)
    }
}

impl From<(f64, f64)> for CGPoint {
    #[inline]
    fn from((x, y): (f64, f64)) -> CGPoint {
        CGPoint { x: x as _, y: y as _ }
    }
}

type CGEvent = *mut raw::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
enum CGEventType {
    // The null event.
    Null,

    // Mouse events.
    LeftMouseDown,
    LeftMouseUp,
    RightMouseDown,
    RightMouseUp,
    MouseMoved,
    LeftMouseDragged,
    RightMouseDragged,

    // Keyboard events.
    KeyDown,
    KeyUp,
    FlagsChanged,

    // Specialized control devices.
    ScrollWheel,
    TabletPointer,
    TabletProximity,
    OtherMouseDown,
    OtherMouseUp,
    OtherMouseDragged,

    // Out of band event types. These are delivered to the event tap callback
    // to notify it of unusual conditions that disable the event tap.
    TapDisabledByTimeout,
    TapDisabledByUserInput,
}

unsafe impl Encode for CGPoint {
    fn encode() -> Encoding {
        let inner = f64::encode();
        let encoding = format!("{{CGPoint={0}{0}}}", inner.as_str());
        unsafe { Encoding::from_str(&encoding) }
    }
}

/// An event location.
#[derive(Copy, Clone)]
pub enum EventLocation {
    /// The event is placed at the point where HID system events enter the
    /// window server.
    Hid,
    /// The event is placed at the point where HID system and remote control
    /// events enter a login session.
    Session,
    /// The event is placed at the point where session events have been
    /// annotated to flow to an application.
    AnnotatedSession,
}
