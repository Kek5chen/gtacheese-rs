use std::ffi::c_void;
use std::fmt::Formatter;
use std::mem::size_of;
use std::ptr;
use thiserror::Error;
use windows::Win32::System::Memory::*;

#[derive(Error, Debug)]
pub struct SignatureError(String);

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn handle_hex_parse_error(
    transformed: &mut Vec<Option<u8>>,
    sig: &str,
    byte: &str,
) -> Result<(), SignatureError> {
    if cfg!(debug_assertions) {
        Err(SignatureError(format!(
            "Failed to parse byte \"{}\" in signature {}. CRASHING, SINCE WE'RE IN DEBUG MODE!",
            sig, byte
        )))
    } else {
        log::warn!(
            "Failed to parse byte \"{}\" in signature {}. Turning this byte into a wildcard (\"??\") instead. Make sure to fix this.",
            sig,
            byte
        );
        transformed.push(None);
        Ok(())
    }
}

fn transform_sig_from_human(sig: &str) -> Result<Vec<Option<u8>>, SignatureError> {
    let mut transformed = Vec::new();
    for b in sig.split_whitespace() {
        if b.starts_with('?') || b.len() > 2 {
            transformed.push(None)
        } else {
            match u8::from_str_radix(b, 16) {
                Ok(n) => transformed.push(Some(n)),
                Err(_) => {
                    handle_hex_parse_error(&mut transformed, sig, b)?;
                }
            }
        }
    }
    Ok(transformed)
}

fn sniff_region(haystack: &[u8], needle: &[Option<u8>]) -> Option<usize> {
    if needle.len() > haystack.len() || haystack.is_empty() {
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

unsafe fn apply_offsets(mut addr: usize, offsets: &[usize]) -> usize {
    if offsets.is_empty() {
        log::debug!("  - No offsets supplied");
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
pub unsafe fn scan_for_data_sig<T>(
    sig_str: &str,
    offsets: &[usize],
) -> Result<*mut T, SignatureError> {
    scan_for_sig(sig_str, offsets, false)
}

pub unsafe fn scan_sig<T>(
    sig_str: &str,
    offsets: &[usize],
) -> Result<*mut T, SignatureError> {
    scan_for_sig(sig_str, offsets, true)
}

pub unsafe fn scan_for_sig<T>(
    sig_str: &str,
    offsets: &[usize],
    include_code: bool,
) -> Result<*mut T, SignatureError> {
    let sig = transform_sig_from_human(sig_str)?;
    log::debug!(
        "Starting pattern scan for {} byte long signature {} and offsets {:?}",
        sig.len(),
        sig_str,
        offsets
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
        // log::debug!(
        //     "  - Scanning Region {} @ {:?} with size {}",
        //     mbi.PartitionId,
        //     mbi.BaseAddress,
        //     mbi.RegionSize
        // );
        count += 1;

        if (mask & mbi.Protect) == mask {
            let memory = std::slice::from_raw_parts(mbi.BaseAddress as *const u8, mbi.RegionSize);
            if let Some(offset) = sniff_region(memory, &sig) {
                let found_pattern_addr = mbi.BaseAddress.byte_add(offset) as usize;
                log::debug!(
                    "    - Found signature at {:?}. Scanned {} regions",
                    found_pattern_addr as *const c_void,
                    count,
                );

                let final_addr = apply_offsets(found_pattern_addr, offsets);
                return Ok(final_addr as *mut T);
            }
        }
        address += mbi.RegionSize;
    }

    Err(SignatureError(format!(
        "Did not find signature. Scanned {} regions",
        count
    )))
}
