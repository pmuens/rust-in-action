use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(self)
    }
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        *self
        // ... or
        // CubeSat { id: self.id }
    }
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64,
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

fn check_status(_sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);

    // ----------

    let base_1 = Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base_1: {:?}", base_1);

    {
        let mut base_2 = base_1.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base_1: {:?}", base_1);

    let mut base_3 = base_1.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base_1: {:?}", base_1);
    println!("base_3: {:?}", base_3);

    // ----------

    let mut mail = Mailbox { messages: vec![] };
    let base = GroundStation { radio_freq: 87.65 };

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
        println!("{:?}: {:?}", sat, msg.unwrap().content);
    }
}
