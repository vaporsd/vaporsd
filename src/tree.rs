pub struct Conf {
	pub version: &'static str,
	pub images: Vec<(&'static str, &'static str)>,
}

pub fn get() -> Conf {
	Conf {
		version: "0.1",
		images: vec![
			("redis", "redis:5.0.4"),
			("haproxy", "haproxy:1.9.7")
		]
	}
}
