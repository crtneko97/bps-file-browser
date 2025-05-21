use anyhow::Result;
use clap::Parser;

mod cli;
mod usb;
mod mtp;

fn main() -> Result<()> 
{
    let args = cli::Cli::parse();

    match args.command 
    {
        cli::Command::Scan { xiaomi } => 
        {
            if xiaomi 
            {
                println!("Looking for Xiaomi phone…");

                let handle = usb::open_xiaomi()?;
                println!("Xiaomi opened! Endpoints:");
                usb::list_endpoints(&handle)?;

                let mut session = mtp::MtpSession::new(handle)?;
                let info = session.get_device_info()?;
                println!("Raw GetDeviceInfo response: {:x?}", info);
            }
            else 
            {
                println!("Non-Xiaomi scan not implemented yet.");
            }
        }
    }

    Ok(())
}

