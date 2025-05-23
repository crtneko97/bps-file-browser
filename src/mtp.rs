use anyhow::{Result, Context};
use rusb::{Context as UsbContext, DeviceHandle, UsbContext as _};
use std::convert::TryInto;
use std::char;
use std::time::Duration;

pub struct DeviceInfo {
    pub manufacturer: String,
    pub model: String,
    pub serial_number: String,
}

pub struct MtpSession {
    handle: DeviceHandle<UsbContext>,
    ep_in: u8,
    ep_out: u8,
    transaction_id: u32,
}

impl MtpSession {
    pub fn new(handle: DeviceHandle<UsbContext>) -> Result<Self> {
        let cfg = handle.device().active_config_descriptor()?;
        let mut ep_in = 0;
        let mut ep_out = 0;

        'outer: for iface in cfg.interfaces() {
            for desc in iface.descriptors() {
                for ep in desc.endpoint_descriptors() {
                    match (ep.transfer_type(), ep.address() & 0x80) {
                        (rusb::TransferType::Bulk, 0x80) => ep_in = ep.address(),
                        (rusb::TransferType::Bulk, 0x00) => ep_out = ep.address(),
                        _ => {}
                    }
                    if ep_in != 0 && ep_out != 0 {
                        break 'outer;
                    }
                }
            }
        }

        Ok(Self { handle, ep_in, ep_out, transaction_id: 1 })
    }

    pub fn get_device_info(&mut self) -> Result<Vec<u8>> {
        // Build the 12‐byte command block:
        let mut cmd = [0u8; 12];
        cmd[0..4].copy_from_slice(&12u32.to_le_bytes());
        cmd[4..6].copy_from_slice(&1u16.to_le_bytes());
        cmd[6..8].copy_from_slice(&0x1001u16.to_le_bytes());
        cmd[8..12].copy_from_slice(&self.transaction_id.to_le_bytes());

        // Send it…
        self.handle
            .write_bulk(self.ep_out, &cmd, Duration::from_secs(1))
            .context("Failed to write GetDeviceInfo")?;

        // Read the response…
        let mut resp = vec![0u8; 512];
        let size = self.handle
            .read_bulk(self.ep_in, &mut resp, Duration::from_secs(1))
            .context("Failed to read GetDeviceInfo response")?;
        resp.truncate(size);
        self.transaction_id += 1;
        Ok(resp)
    }

    /// Parse raw GetDeviceInfo bytes into a DeviceInfo struct
    pub fn parse_device_info(raw: &[u8]) -> Result<DeviceInfo> {
        let mut i = 12usize; // skip container header

        fn read_u16(buf: &[u8], idx: &mut usize) -> u16 {
            let v = u16::from_le_bytes(buf[*idx..*idx + 2].try_into().unwrap());
            *idx += 2;
            v
        }
        fn read_u32(buf: &[u8], idx: &mut usize) -> u32 {
            let v = u32::from_le_bytes(buf[*idx..*idx + 4].try_into().unwrap());
            *idx += 4;
            v
        }
        fn read_string(buf: &[u8], idx: &mut usize) -> String {
            let count = buf[*idx] as usize;
            *idx += 1;
            let mut s = String::new();
            for _ in 0..count {
                let code = u16::from_le_bytes(buf[*idx..*idx + 2].try_into().unwrap());
                *idx += 2;
                if code == 0 { break; }
                s.push(char::from_u32(code as u32).unwrap_or('?'));
            }
            s
        }

        // Skip to manufacturer
        let _ = read_u16(raw, &mut i);
        let _ = read_u32(raw, &mut i);
        let _ = read_u16(raw, &mut i);
        let _ = read_string(raw, &mut i);
        let _ = read_u16(raw, &mut i);

        let manufacturer  = read_string(raw, &mut i);
        let model         = read_string(raw, &mut i);
        let _ = read_string(raw, &mut i); // device version
        let serial_number = read_string(raw, &mut i);

        Ok(DeviceInfo { manufacturer, model, serial_number })
    }
}

