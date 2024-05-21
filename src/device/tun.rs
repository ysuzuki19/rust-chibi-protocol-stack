use std::io::{Read, Write};

use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task;
use tun::platform::posix::{Reader, Writer};
use tun::platform::Device;

use crate::error::CustomRes;
use crate::ip::packet::IpPacket;
use crate::ip::payload::PacketFilter;
use crate::ip::{Buf, Deserialize};

pub struct TunDeviceBuilder {
    config: tun::Configuration,
}

impl TunDeviceBuilder {
    pub fn new() -> Self {
        Self {
            config: tun::Configuration::default(),
        }
    }

    pub fn address(mut self, addr: (u8, u8, u8, u8)) -> Self {
        self.config.address(addr);
        self
    }

    pub fn netmask(mut self, mask: (u8, u8, u8, u8)) -> Self {
        self.config.netmask(mask);
        self
    }

    pub fn build(mut self) -> CustomRes<TunDevice> {
        self.config.up();
        TunDevice::setup(self.config)
    }
}

pub struct TunDevice {
    dev: Device,
}

impl TunDevice {
    fn setup(config: tun::Configuration) -> CustomRes<Self> {
        let dev = tun::create(&config)?;
        Ok(Self { dev })
    }

    pub fn bind(self, port: u16) -> CustomRes<(Receiver<IpPacket>, Sender<Buf>, NetworkWorker)> {
        let (inbound, outbound, worker) = NetworkWorker::bind(self.dev, port)?;
        Ok((inbound, outbound, worker))
    }
}

pub struct NetworkWorker {
    _inbound_worker: tokio::task::JoinHandle<CustomRes<()>>,
    _outbound_worker: tokio::task::JoinHandle<CustomRes<()>>,
}

impl NetworkWorker {
    pub fn bind(device: Device, port: u16) -> CustomRes<(Receiver<IpPacket>, Sender<Buf>, Self)> {
        let (reader, writer) = device.split();
        let (inbound_tx, inbound_rx) = mpsc::channel::<IpPacket>(10);
        let read_worker = Self::setup_read_worker(reader, inbound_tx, port);

        let (outbound_tx, outbound_rx) = mpsc::channel::<Buf>(10);
        let write_worker = Self::setup_write_worker(writer, outbound_rx);

        Ok((
            inbound_rx,
            outbound_tx,
            Self {
                _inbound_worker: read_worker,
                _outbound_worker: write_worker,
            },
        ))
    }

    fn setup_read_worker(
        mut reader: Reader,
        inbound_tx: Sender<IpPacket>,
        port: u16,
    ) -> tokio::task::JoinHandle<CustomRes<()>> {
        task::spawn_blocking({
            move || -> CustomRes<()> {
                loop {
                    let mut buf = vec![0; 1500];
                    let len = reader.read(&mut buf)?;
                    if len == 0 {
                        continue;
                    }
                    buf.truncate(len);
                    let packet = match IpPacket::deserialize(&buf) {
                        Ok(packet) => packet,
                        Err(e) => {
                            e.logging();
                            continue;
                        }
                    };
                    if matches!(packet.payload.filter(port), PacketFilter::Pass) {
                        inbound_tx.blocking_send(packet)?;
                    }
                }
            }
        })
    }

    fn setup_write_worker(
        mut writer: Writer,
        mut outbound_rx: Receiver<Buf>,
    ) -> tokio::task::JoinHandle<CustomRes<()>> {
        task::spawn_blocking({
            move || -> CustomRes<()> {
                while let Some(buf) = outbound_rx.blocking_recv() {
                    writer.write_all(buf.as_slice())?;
                }
                Ok(())
            }
        })
    }
}
