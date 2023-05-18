use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Deserializer, Serializer};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Body {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Error {
        code: usize,
        text: String,
    },
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Generate,
    GenerateOk {
        id: usize,
    },
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Add {
        delta: usize,
    },
    AddOk
}

struct Server {
    id: String,
    nodes: Vec<String>,
    msg_id: usize,
    unique_id: usize,
    broadcasts: Vec<usize>,
}

impl Server {
    fn new(id: String, nodes: Vec<String>) -> Server {
        Server {
            id,
            nodes,
            msg_id: 0,
            unique_id: 0,
            broadcasts: Vec::new(),
        }
    }

    fn response(&mut self, msg: Message) -> Message {
        eprintln!("Received {msg:?}");

        let response_payload = match msg.body.payload {
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Init { .. } => Payload::InitOk,
            Payload::Generate => Payload::GenerateOk {
                id: self.generate_id(),
            },
            Payload::Broadcast { message } => {
                self.broadcasts.push(message);
                Payload::BroadcastOk
            }
            Payload::Read => Payload::ReadOk {
                messages: self.broadcasts.clone(),
            },
            Payload::Topology { .. } => Payload::TopologyOk,

            _ => panic!("Not implemented"),
        };

        let msg = Message {
            src: self.id.clone(),
            dst: msg.src,
            body: Body {
                msg_id: Some(self.msg_id),
                in_reply_to: msg.body.msg_id,
                payload: response_payload,
            },
        };
        self.msg_id += 1;
        eprintln!("Sending {msg:?}");
        msg
    }

    fn generate_id(&mut self) -> usize {
        let id = self.unique_id;
        self.unique_id += 1;
        id
    }
}

fn main() -> Result<(), ()> {
    let stdin = std::io::stdin().lock();
    let stdout = std::io::stdout().lock();

    let mut input = Deserializer::from_reader(stdin).into_iter::<Message>();
    let mut output = Serializer::new(stdout);

    let init_message = input.next().unwrap().unwrap();
    let (node_id, node_ids) = match init_message.body.payload.clone() {
        Payload::Init { node_id, node_ids } => (node_id, node_ids),
        _ => panic!("First message was not Init"),
    };
    let mut server = Server::new(node_id, node_ids);
    server
        .response(init_message)
        .serialize(&mut output)
        .unwrap();
    print!("\n");
    for message in input {
        let message = message.unwrap();
        let response = server.response(message);
        response.serialize(&mut output).unwrap();
        print!("\n");
    }
    Ok(())
}
