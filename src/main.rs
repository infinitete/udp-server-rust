extern crate udp_server;

use udp_server::server;

fn main() {
    server::run([0, 0, 0, 0], 9001);
}
