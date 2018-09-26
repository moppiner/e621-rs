extern crate e621;

use e621::E621Client;

fn main() {
	let client = E621Client::new();
	let tags = String::from("order:score");
	for post in client.list(tags, 10).unwrap() {
		println!("{}", post.file_url);
	}
}