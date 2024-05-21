mod device;
mod error;
mod ip;
mod util;

use error::CustomRes;
use ip::payload::handle::HandleStatus;

use crate::ip::Serialize;

#[tokio::main]
async fn main() -> CustomRes<()> {
    let (mut inbound, outbound, _) = device::tun::TunDeviceBuilder::new()
        .address((192, 0, 2, 2))
        .netmask((255, 255, 255, 0))
        .build()?
        .bind(3000)?;
    while let Some(packet) = inbound.recv().await {
        match packet
            .payload
            .handle(packet.header.create_addr_pair())
            .await
        {
            HandleStatus::Send(payloads) => {
                for payload in payloads {
                    let mut packet = ip::packet::IpPacket {
                        header: packet.header.prepare_reply(),
                        payload,
                    };
                    packet.prepare_send();
                    let mut buf = Vec::with_capacity(1500);
                    packet.serialize(&mut buf);
                    outbound.send(buf).await?;
                }
            }
            HandleStatus::Stop => {
                continue;
            }
        }
    }
    Ok(())
}
