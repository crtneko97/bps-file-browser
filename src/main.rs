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
                
                let raw = session.get_device_info()?;
                let info = mtp::MtpSession::parse_device_info(&raw)?;
                println!("Manufacturer:  {}", info.manufacturer);
                println!("Model:         {}", info.model);
                println!("Serial number: {}", info.serial_number);

            }
            else 
            {
                println!("Non-Xiaomi scan not implemented yet.");
            }
        }
        
        cli::Command::List { xiaomi } =>
        {
            if xiaomi
            {
                println!("Listing files on Xiaomi..");
                let handle = usb::open_xiaomi()?;
                let mut session = mtp::MtpSession::new(handle)?;






                println!("(stub) Works. Confirmed BPS-");
            }
            else
            {
                println!("List for non-Xiamoi not supported.");
            }
        }
        
    }

    Ok(())
}

