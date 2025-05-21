use anyhow::Result;
use rusb::{Context, DeviceDescriptor, UsbContext}; 
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
        let desc: DeviceDescriptor = device.device_descriptor()?;
        let vid = desc.vendor_id();
        let pid = desc.product_id();

        if args.xiaomi && vid != 0x2717 
        {
            continue;
        }

        println!("• Device — VID: {:04x}, PID: {:04x}", vid, pid);
    }

    Ok(())
}

