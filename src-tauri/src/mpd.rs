use mpd_client::{
    client::{Connection, ConnectionEvent, ConnectionEvents, Subsystem},
    protocol::MpdProtocolError,
    Client,
};
use tauri::{AppHandle, Manager};
use tokio::net::TcpStream;

pub async fn initialize_connection(addr: &str) -> Result<Connection, MpdProtocolError> {
    let conn = TcpStream::connect(addr).await?;
    Client::connect(conn).await
}

pub async fn event_handler(handle: AppHandle, mut events: ConnectionEvents) -> tauri::Result<()> {
    while let Some(event) = events.next().await {
        match event {
            ConnectionEvent::SubsystemChange(ev) => match ev {
                Subsystem::Queue => {
                    handle.emit_all("queue", ())?;
                }
                Subsystem::Options => {
                    handle.emit_all("status", ())?;
                }
                Subsystem::Player => {
                    handle.emit_all("status", ())?;
                    handle.emit_all("currentsong", ())?;
                }
                _ => {}
            },
            ConnectionEvent::ConnectionClosed(e) => {
                eprintln!("Connection closed unexpectly: {}", e);
                handle.emit_all("connection-closed", ())?;
                // TODO: Implement connection lost notification.
                if let Err(e) = handle.emit_all("connection-lost", ()) {
                    log::error!("Failed to emit signal 'connection-lost': {}", e);
                }
                break;
            }
        }
    }
    Ok(())
}
