use crate::ws::msg::{Msg, MsgType};
use crate::ws::{ADDR_MAP, PEER_MAP, UID_MAP, USER_MAP};
use cassie_domain::vo::jwt::JWTToken;
use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local};
use std::net::SocketAddr;
use tokio_tungstenite::tungstenite::Message;

//消息处理
pub fn handle_msg(addr: SocketAddr, msg: String) {
    let mut massage: Msg = serde_json::from_str(&*msg).unwrap();
    match massage.mt {
        MsgType::Ping => {
            //心跳检测
            {
                let p = PEER_MAP.clone();
                let peers = p.lock().unwrap();
                let recp = peers.get(&addr).unwrap();
                let body = crate::ws::msg::MsgBody {
                    from: "".to_string(),
                    to: "".to_string(),
                    text: "receive".to_string(),
                };
                massage.date = Some(get_date());
                massage.body = Some(body);
                let m = serde_json::to_string(&massage).unwrap();
                recp.unbounded_send(Message::from(m)).unwrap();
            }
        }
        MsgType::Msg => {
            //消息发送 默认发给所有人
            {
                let usermap = USER_MAP.clone();
                let p = PEER_MAP.clone();
                let peers = p.lock().unwrap();
                let userinfomap = usermap.lock().unwrap();
                let indfo = userinfomap.get(&addr).unwrap();
                let mut body = massage.body.clone().unwrap();
                body.from = indfo.username().clone();
                massage.date = Some(get_date());
                massage.body = Some(body);
                // 把消息发送给 非自己的所有用户
                let broadcast_recipients = peers
                    .iter()
                    .filter(|(peer_addr, _)| peer_addr != &&addr)
                    .map(|(_, ws_sink)| ws_sink);
                for recp in broadcast_recipients {
                    let m = serde_json::to_string(&massage).unwrap();
                    recp.unbounded_send(Message::from(m)).unwrap();
                }
            }
        }
        MsgType::UserList => {
            //获取在线用户信息
            //这里多加个括号是为了防止 Mutex锁中毒  达到显示的dorp效果  也可以  dorp(xxx)
            {
                let usermap = USER_MAP.clone();
                let userinfomap = usermap.lock().unwrap();
                //获取到所有的在线用户
                let list: Vec<JWTToken> = userinfomap.iter().map(|(_, v)| v.clone()).collect();
                let p = PEER_MAP.clone();
                let peers = p.lock().unwrap();
                let recp = peers.get(&addr).unwrap();
                let body = crate::ws::msg::MsgBody {
                    from: "".to_string(),
                    to: "".to_string(),
                    text: serde_json::to_string(&list).unwrap(),
                };
                massage.date = Some(get_date());
                massage.body = Some(body);
                let m = serde_json::to_string(&massage).unwrap();
                recp.unbounded_send(Message::from(m)).unwrap();
            }
        }
    }
}

//用户下线
pub fn off_line(addr: &SocketAddr) {
    let p = PEER_MAP.clone();
    let umap = UID_MAP.clone();
    let amap = ADDR_MAP.clone();
    let usermap = USER_MAP.clone();
    p.lock().unwrap().remove(addr);
    usermap.lock().unwrap().remove(addr);
    let uid = amap.lock().unwrap().remove(addr);
    match uid {
        None => {}
        Some(id) => {
            umap.lock().unwrap().remove(&id);
        }
    }
}
//根据用户id下线
pub fn off_line_by_uid(uid: String) {
    let umap = UID_MAP.clone();
    let amap = ADDR_MAP.clone();
    let usermap = USER_MAP.clone();
    let p = PEER_MAP.clone();
    let addr = umap.lock().unwrap().remove(&uid);
    match addr {
        None => {}
        Some(address) => {
            amap.lock().unwrap().remove(&address);
            usermap.lock().unwrap().remove(&address);
            p.lock().unwrap().remove(&address);
        }
    }
}
//用户上线
pub fn on_line(userinfo: JWTToken, addr: SocketAddr) {
    let umap = UID_MAP.clone();
    let amap = ADDR_MAP.clone();
    let usermap = USER_MAP.clone();
    usermap.lock().unwrap().insert(addr, userinfo.clone());
    umap.lock().unwrap().insert(userinfo.id().to_string(), addr);
    amap.lock().unwrap().insert(addr, userinfo.id().to_string());
}

//向单个用户发送消息
fn send_msg(uid: String, msg: String) {
    let peer_map = PEER_MAP.clone();
    let umap = UID_MAP.clone();
    let mp = umap.lock().unwrap();
    let addr = mp.get(&*uid);
    match addr {
        None => {
            //用户不在线
        }
        Some(address) => {
            //如果用户在线 则发送消息
            if let Some(recp) = peer_map.lock().unwrap().get(address) {
                recp.unbounded_send(Message::from(msg.clone())).unwrap();
            }
        }
    }
}
fn get_date() -> String {
    let fmt = "%Y-%m-%d %H:%M:%S";
    let now: DateTime<Local> = Local::now();
    let dft: DelayedFormat<StrftimeItems> = now.format(fmt);
    dft.to_string()
}
