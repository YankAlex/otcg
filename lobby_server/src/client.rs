use axum::extract::ws::WebSocket;

pub struct Client {
    websocket: WebSocket,
}

impl Client {
    pub fn new(websocket: WebSocket) -> Self {
        Self {
            websocket,
        }
    }
    pub fn get_websocket(self) -> WebSocket {
        self.websocket
    }
    pub async fn is_ready_to_start(&mut self) -> bool {
        true
    }
}
