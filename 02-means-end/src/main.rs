use std::collections::BTreeMap;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tracing::{event, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Clone)]
struct Packet {
    method: char,
    args: (i32, i32),
}

impl Packet {
    fn new(bytes: &[u8; 9]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let method = char::from(bytes[0]);
        let arg1 = i32::from_be_bytes(bytes[1..5].try_into()?);
        let arg2 = i32::from_be_bytes(bytes[5..9].try_into()?);

        Ok(Self {
            method,
            args: (arg1, arg2),
        })
    }
}

#[derive(Debug, Clone, Default)]
struct Session {
    history: BTreeMap<i32, i32>,
}

impl Session {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn add_packet(&mut self, p: Packet) {
        self.history.insert(p.args.0, p.args.1);
    }

    fn query(&mut self, min: i32, max: i32) -> i32 {
        if min > max {
            return 0;
        }

        let mut sum: i64 = 0;
        let mut count: i64 = 0;

        let range = self.history.range(min..=max);

        for (_, &i) in range {
            sum += i64::from(i);
            count += 1;
        }

        if count > 0 {
            return (sum / count).try_into().unwrap_or(0);
        }

        0
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf: [u8; 9] = [0; 9];
    let mut session = Session::new();

    loop {
        if let Ok(_) = socket.read_exact(&mut buf).await {
            event!(Level::TRACE, ?buf);

            if let Ok(packet) = Packet::new(&buf) {
                event!(Level::DEBUG, ?packet);

                let reply = match packet.method {
                    'I' => {
                        session.add_packet(packet);
                        None
                    }
                    'Q' => Some(session.query(packet.args.0, packet.args.1)),
                    _ => None,
                };

                if let Some(reply) = reply {
                    event!(Level::INFO, ?reply, "sent");

                    socket.write_all(&reply.to_be_bytes()).await;
                }
            }
        } else {
            event!(Level::INFO, ?socket, "closed");

            return;
        }
    }
}

#[cfg(debug_assertions)]
fn debug_only() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    debug_only();

    let addr = "0.0.0.0:45100";
    let listener = TcpListener::bind(&addr).await?;
    event!(Level::INFO, ?addr, "listening");

    loop {
        let (socket, _) = listener.accept().await?;
        event!(Level::INFO, ?socket, "connected");

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}
