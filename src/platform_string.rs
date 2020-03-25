use os_info;
use os_info::{Bitness, Type};

pub fn platform_string() -> Option<String> {
    let info = os_info::get();
    let bitness = match info.bitness() {
        Bitness::X32 => Some("x86"),
        Bitness::X64 => Some("x64"),
        _ => None,
    }?;

    // TODO(thomas-crane): improve this match.
    let os_type = match info.os_type() {
        Type::Windows => "win",
        Type::Macos => "darwin",
        _ => "linux",
    };
    
    let mut result = String::from(os_type);
    result.push('-');
    result.push_str(bitness);
    Some(result)
}
