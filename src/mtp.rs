use anyhow::Result;
use rusb::DeviceHandle;
use std::time::Duration;

pub struct MtpSession 
{
    handle: DeviceHandle<rusb::Context>,
    ep_in: u8,
    ep_out: u8,
    transaction_id: u32,
}

impl MtpSession 
{
    pub fn new(mut handle: DeviceHandle<rusb::Context>) -> Result<Self> 
    {
        let cfg = handle.device().active_config_descriptor()?;
        let mut ep_in = 0;
        let mut ep_out = 0;
        'outer: for iface in cfg.interfaces() 
        {
            for desc in iface.descriptors() 
            {
                for ep in desc.endpoint_descriptors() 
                {
                    match (ep.transfer_type(), ep.address() & 0x80) 
                    {
                        (rusb::TransferType::Bulk, 0x80) => ep_in = ep.address(),
                        (rusb::TransferType::Bulk, 0x00) => ep_out = ep.address(),
                        _ => {}
                    }
                    if ep_in != 0 && ep_out != 0 
                    {
                        break 'outer;
                    }
                }
            }
        }
        Ok(Self { handle, ep_in, ep_out, transaction_id: 1 })
    }

    pub fn get_device_info(&mut self) -> Result<Vec<u8>> {
        // Build the 12‐byte command block:
        // [length (u32 LE), type=1 (u16 LE), opcode=0x1001 (u16 LE), trans_id (u32 LE)]
        let mut cmd = [0u8; 12];
        let len = 12u32.to_le_bytes();
        let r#type = 1u16.to_le_bytes();       // Command Block
        let op   = 0x1001u16.to_le_bytes();    // GetDeviceInfo
        let tid  = self.transaction_id.to_le_bytes();
        cmd[0..4].copy_from_slice(&len);
        cmd[4..6].copy_from_slice(&r#type);
        cmd[6..8].copy_from_slice(&op);
        cmd[8..12].copy_from_slice(&tid);

        // Send it
        self.handle.write_bulk(self.ep_out, &cmd, Duration::from_secs(1))?;
        // Then read the response header (usually 12 bytes, but can be longer)
        let mut resp = vec![0u8; 512];
        let size = self.handle.read_bulk(self.ep_in, &mut resp, Duration::from_secs(1))?;
        resp.truncate(size);
        self.transaction_id += 1;
        Ok(resp)
    }
}
