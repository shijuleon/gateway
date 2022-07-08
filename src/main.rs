use rand::Rng;
use std::collections::VecDeque;
use std::iter::Iterator;

struct Backend {
  hosts: VecDeque<Host>,
}

impl Backend {
  fn new() -> Self {
    Backend {
      hosts: VecDeque::new(),
    }
  }

  fn set_hosts(&mut self, hosts: VecDeque<Host>) {
    self.hosts = hosts
  }
}

struct Host {
  name: String,
}

enum Scheme {
  RoundRobin,
  Random,
  Undefined,
}

struct BalancingScheme<'a> {
  backend: &'a Backend,
  scheme: Scheme,
  iter: Box<dyn Iterator<Item = &'a Host> + 'a>,
}

impl<'a> Iterator for BalancingScheme<'a> {
  type Item = &'a Host;

  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next()
  }
}

impl<'a> BalancingScheme<'a> {
  fn new(backend: &'a Backend, scheme: String) -> Self {
    let sample = backend.hosts.iter();
    let mut rng = &mut rand::thread_rng();

    match scheme.as_str() {
      "random" => BalancingScheme {
        backend: backend,
        scheme: Scheme::Random,
        iter: Box::new(sample),
      },
      "roundrobin" => BalancingScheme {
        backend: backend,
        scheme: Scheme::RoundRobin,
        iter: Box::new(backend.hosts.iter().cycle()),
      },
      _ => BalancingScheme {
        backend: backend,
        scheme: Scheme::Undefined,
        iter: Box::new(backend.hosts.iter().cycle()),
      },
    }
  }
}

fn random() {}

fn main() {
  let mut hosts = VecDeque::new();
  hosts.push_back(Host {
    name: "A".to_string(),
  });
  hosts.push_back(Host {
    name: "B".to_string(),
  });
  hosts.push_back(Host {
    name: "C".to_string(),
  });

  let mut us_east = Backend::new();
  us_east.set_hosts(hosts);

  let mut rr = BalancingScheme::new(&us_east, "roundrobin".to_string());

  println!("{}", rr.next().unwrap().name);
  println!("{}", rr.next().unwrap().name);
  println!("{}", rr.next().unwrap().name);
  rr = BalancingScheme::new(&us_east, "random".to_string());

  println!("{}", rr.next().unwrap().name);
}
