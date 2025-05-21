use anyhow::Result;
use rusb::{Context, DeviceDescriptor, DeviceHandle, UsbContext}; 
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about="USB phone detector")]
struct Cli 
{
    #[arg(long)]
    xiaomi: bool,
}

fn main() -> Result<()> 
{
    let args = Cli::parse();
    let ctx = Context::new()?;
    println!("Scanning USB bus for devices");

    for device in ctx.devices()?.iter() 
    {
       let desc = device.device_descriptor()?; 
       if args.xiaomi && desc.vendor_id() != 0x2717
       {
           continue;
       }

    println!("Found device VID={:04x}, PID={:04x}", 
        desc.vendor_id(), 
        desc.product_id());

    let mut handle: DeviceHandle<_> = device.open()
        .map_err(|e| anyhow::anyhow!("Failed to open device: {}", e))?;

    if let Err(e) = handle.set_active_configuration(1)
    {
        eprintln!("Could not set configuration #1 (maybe it's already set): {}", e);
    }

   // mtp is often on interface 0 or 1. 
    handle.claim_interface(0)
       .map_err(|e| anyhow::anyhow!("Could not claim interface 0: {}", e))?;

    let config = handle.device().active_config_descriptor()?;
    for interface in config.interfaces()
    {
        for descriptor in interface.descriptors()
        {
            for endpoint in descriptor.endpoint_descriptors()
            {
                let addr = endpoint.address();
                let attr = endpoint.transfer_type();
                println!(
                " → interface {}.{} endpoint 0x{:02x}, {:?}",
                interface.number(), descriptor.setting_number(), addr, attr
                );
            }
        }
    }
    break;
    }
    Ok(())
}

