use std::collections::VecDeque;

#[derive(Debug)]
struct Client {
    id: u32,
    arrival_time: u64,
}

fn add_client(queue: &mut VecDeque<Client>, id: u32, time: u64) {
    queue.push_back(Client {
        id,
        arrival_time: time,
    });
}

fn main() {
    let mut deque: VecDeque<Client> = VecDeque::new();

    add_client(&mut deque, 1, 100);
    add_client(&mut deque, 2, 200);
    add_client(&mut deque, 0, 50);

    println!("{:?}", deque);
}
