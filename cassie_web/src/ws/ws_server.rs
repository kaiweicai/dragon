use futures_channel::mpsc::unbounded;
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use log::info;
use std::{io::Error as IoError, net::SocketAddr};

use crate::middleware::checked_token;
use crate::ws::ws_handle::{handle_msg, off_line, off_line_by_uid, on_line};
use crate::ws::PEER_MAP;
use crate::APPLICATION_CONTEXT;
use cassie_config::config::ApplicationConfig;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::protocol::Message;

//核心请求处理
async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    //构建权限错误信息
    let resp = Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Some("token 不存在".into()))
        .unwrap();
    let ws_stream =
        tokio_tungstenite::accept_hdr_async(raw_stream, |req: &Request, response: Response| {
            //获取url参数
            match req.uri().query() {
                //没有参数证明 没权限
                None => {
                    return Err(resp);
                }
                Some(query) => {
                    let t = query.split("=").collect::<Vec<&str>>();
                    //参数错误也返回错误
                    if t.len() < 2 || t.get(0).unwrap().clone() != "access_token" {
                        return Err(resp);
                    }
                    let access_token = t.get(1).unwrap();
                    //验证token 是否正确 错误则返回
                    match checked_token(access_token) {
                        Ok(data) => {
                            off_line_by_uid(data.id().to_string());
                            on_line(data, addr);
                        }
                        Err(_) => {
                            return Err(resp);
                        }
                    }
                }
            }
            Ok(response)
        })
        .await;

    match ws_stream {
        Ok(ws_s) => {
            info!("WebSocket 连接成功: {}", addr);
            // 讲连接进来的客户端加入map方便全局使用
            let (tx, rx) = unbounded();
            let p = PEER_MAP.clone();
            p.lock().unwrap().insert(addr, tx);
            let (outgoing, incoming) = ws_s.split();
            let broadcast_incoming = incoming.try_for_each(|msg| {
                match msg {
                    Message::Text(ms) => {
                        handle_msg(addr, ms);
                    }
                    _ => {}
                }
                future::ok(())
            });
            let receive_from_others = rx.map(Ok).forward(outgoing);
            pin_mut!(broadcast_incoming, receive_from_others);
            future::select(broadcast_incoming, receive_from_others).await;
            info!("{} 断开连接", &addr);
            off_line(&addr);
        }
        Err(_) => {}
    }
}

//实例化一个 ws server
pub async fn init_ws() -> Result<(), IoError> {
    let cassie_config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    //如果ws端口开启了 则启动 websocket
    if cassie_config.server().ws().is_some() {
        let addr = format!(
            "{}:{}",
            cassie_config.server().host(),
            cassie_config.server().ws().clone().unwrap()
        );
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("绑定失败");
        info!("WS Listening on: {}", addr);
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(stream, addr));
        }
    }
    Ok(())
}
