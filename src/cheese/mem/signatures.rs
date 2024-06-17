use std::ffi::c_void;
use std::mem::size_of;
use std::ptr;
use windows::Win32::System::Memory::*;

fn hex_parse_error(sig: &str, byte: &str) {
    if cfg!(debug_assertions) {
        log::error!(
            "Failed to parse byte \"{}\" in signature {}. CRASHING, SINCE WE'RE IN DEBUG MODE!",
            sig,
            byte
        );
        panic!("")
    } else {
        log::warn!(
            "Failed to parse byte \"{}\" in signature {}. Ignoring this byte instead.",
            sig,
            byte
        );
    }
}

fn transform_sig_from_human(sig: &str) -> Vec<Option<u8>> {
    sig.split_whitespace()
        .map(|b| {
            if b.starts_with('?') || b.len() > 2 {
                None
            } else {
                match u8::from_str_radix(b, 16) {
                    Ok(n) => Some(n),
                    Err(_) => {
                        hex_parse_error(sig, b);
                        None
                    }
                }
            }
        })
        .collect()
}

fn sniff_region(haystack: &[u8], needle: &[Option<u8>]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }

    'outer: for i in 0..=haystack.len() - needle.len() {
        for (j, &opt_needle_byte) in needle.iter().enumerate() {
            if let Some(needle_byte) = opt_needle_byte {
                if haystack[i + j] != needle_byte {
                    continue 'outer;
                }
            }
        }
        return Some(i);
    }
    None
}

unsafe fn apply_offsets(mut addr: usize, offsets: Option<&[usize]>) -> usize {
    if offsets.is_none() {
        log::debug!("  - No offsets supplied");
        return addr;
    }

    let offsets = offsets.unwrap();
    if offsets.is_empty() {
        log::debug!("  - Said that he had offsets, but was lying");
        return addr;
    }
    
    let deref_offset_count = (offsets.len() - 1).saturating_sub(1) + 1;

    for &offset in offsets.iter().take(deref_offset_count) {
        let prev = addr;
        let disp = ptr::read_unaligned((addr as *const u32).byte_add(offset));
        addr += disp as usize;
        log::debug!(
            "  - We are at {:?} and add {:?}, this gets us a displacement of {:?} and leads to: {:?}",
            prev as *const c_void,
            offset as *const c_void,
            disp as *const c_void,
            addr as *const c_void
        );
    }
    
    if offsets.len() > 1 {
        addr += offsets[offsets.len() - 1];
    }

    addr
}

#[allow(dead_code)]
pub unsafe fn scan_for_data_sig<T>(sig_str: &str, offsets: Option<&[usize]>) -> Option<*mut T> {
    scan_for_sig(sig_str, offsets, false)
}

pub unsafe fn scan_sig<T>(sig_str: &str, offsets: Option<&[usize]>) -> Option<*mut T> {
    scan_for_sig(sig_str, offsets, true)
}

pub unsafe fn scan_for_sig<T>(
    sig_str: &str,
    offsets: Option<&[usize]>,
    include_code: bool,
) -> Option<*mut T> {
    let sig = transform_sig_from_human(sig_str);
    log::debug!(
        "Starting pattern scan for {} length signature {} (Converted: {:?})",
        sig.len(),
        sig_str,
        sig
    );

    let mut address: usize = 0;
    let mut mbi = MEMORY_BASIC_INFORMATION::default();
    let mask = match include_code {
        true => PAGE_EXECUTE_READWRITE,
        false => PAGE_READWRITE,
    };
    let mut count = 0;
    while VirtualQuery(
        Some(address as *const c_void),
        &mut mbi,
        size_of::<MEMORY_BASIC_INFORMATION>(),
    ) != 0
    {
        log::debug!(
            "  - Scanning Region {} @ {:?} with size {}",
            mbi.PartitionId,
            mbi.BaseAddress,
            mbi.RegionSize
        );
        count += 1;

        if (mask & mbi.Protect) == mask {
            let memory =
                std::slice::from_raw_parts(mbi.BaseAddress as *const u8, mbi.RegionSize);
            if let Some(offset) = sniff_region(memory, &sig) {
                let found_pattern_addr = mbi.BaseAddress.byte_add(offset) as usize;
                log::debug!(
                    "    - Found signature at {:?}. Scanned {} regions",
                    found_pattern_addr as *const c_void,
                    count,
                );

                let final_addr = apply_offsets(found_pattern_addr, offsets);
                return Some(final_addr as *mut T);
            }
        }
        address += mbi.RegionSize;
    }

    log::debug!("Did not find signature. Scanned {} regions", count);
    None
}
