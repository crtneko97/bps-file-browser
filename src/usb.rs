use anyhow::{Result, Context as AnyhowContext};
use rusb::{Context, DeviceHandle, UsbContext};

pub fn open_xiaomi() -> Result<DeviceHandle<Context>> 
{
    let ctx: Context = Context::new()
        .context("Initializing libusb")?;

    for device in ctx.devices()?.iter() 
    {
        let desc = device.device_descriptor()?;
        if desc.vendor_id() == 0x2717 
        {
            let mut handle: DeviceHandle<Context> = device.open()
                .context("Opening Xiaomi device")?;

            let iface = 0;
            if handle.kernel_driver_active(iface).unwrap_or(false) 
            {
                handle.detach_kernel_driver(iface)
                    .context("Detaching kernel driver from interface 0")?;
            }

            let _ = handle.set_active_configuration(1);
            handle.claim_interface(iface)
                .context("Claiming MTP interface")?;

            return Ok(handle);
        }
    }

    anyhow::bail!("No Xiaomi device found");
}

pub fn list_endpoints(handle: &DeviceHandle<Context>) -> Result<()> 
{
    let config = handle.device()
        .active_config_descriptor()?;

    for interface in config.interfaces() 
    {
        for descriptor in interface.descriptors() 
        {
            for ep in descriptor.endpoint_descriptors() 
            {
                let addr = ep.address();
                let kind = ep.transfer_type();
                println!(
                    "interface {}.{} endpoint 0x{:02x}, {:?}",
                    interface.number(),
                    descriptor.setting_number(),
                    addr,
                    kind,
                );
            }
        }
    }

    Ok(())
}

