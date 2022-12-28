use ht_cal::packet::PacketData;
use ht_cal::datetime::{HDateTime, Month, MonthStatus};
use ht_cal::history::HistoryData;
use std::sync::Mutex;
use std::sync::Arc;
use chrono::TimeZone;
use lazy_static::lazy_static;
use log::warn;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

lazy_static!{
    pub static ref HDATE: Arc<Mutex<PacketData>> = Arc::new(Mutex::new(PacketData {
        year: 0,
        month: (MonthStatus::Greater, Month::Zero),
        day: 0,
        second: Default::default(),
        time_since_second_ms: 0,
        time_of_packet_sent_ms: 0,
    }));
    pub static ref HISTORY: Arc<Mutex<HistoryData>> = Arc::new(Mutex::new(HistoryData::new()));

    // env var "HT_HOST" is the host that the ht_cal server is on
    pub static ref HT_HOST: String = std::env::var("HT_HOST").unwrap_or("localhost".to_string());
}

pub const PORT_Q: u16 = 3621;
pub const PORT_H: u16 = 3926;

pub async fn manage_htcal() {
    // loop every 50 ms
    loop {
        // connect via tcp to PORT_Q
        let mut socket = tokio::net::TcpStream::connect((HT_HOST.as_str(), PORT_Q)).await.unwrap();
        // send a single byte
        socket.write_all(&[0]).await.unwrap();
        // read response
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {
            warn!("ht_cal server did not respond");
        }
        // deserialise with rmp-serde
        let response: PacketData = rmp_serde::decode::from_slice(&buf[..n]).unwrap();
        // update HDATE
        {
            let mut hdt = HDATE.lock().unwrap();
            *hdt = response;
        }
        // connect via tcp to PORT_H
        let mut socket = tokio::net::TcpStream::connect((HT_HOST.as_str(), PORT_H)).await.unwrap();
        // send a single byte
        socket.write_all(&[0]).await.unwrap();
        // read response
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        if n == 0 {
            warn!("ht_cal server did not respond");
        }
        // deserialise with rmp-serde
        let response: HistoryData = rmp_serde::decode::from_slice(&buf[..n]).unwrap();
        // update HISTORY
        {
            let mut history = HISTORY.lock().unwrap();
            history.clone_from(&response);
        }
        // sleep for 50 ms
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}