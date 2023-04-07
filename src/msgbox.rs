use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

pub fn show(caption: impl Into<String>, text: impl Into<String>) {
    let caption = HSTRING::from(caption.into());
    let text = HSTRING::from(text.into());
    unsafe { MessageBoxW(None, &text, &caption, MB_DEFMASK); }
}