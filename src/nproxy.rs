use futures::future;
use std::{collections::HashMap, env, path::PathBuf, sync::Mutex};

use crate::{config::config_loader, worker::Worker};

fn get_cwd() -> PathBuf {
    env::current_dir().unwrap()
}

pub async fn start() -> Vec<Result<(), hyper::Error>> {
    let dir = get_cwd().join("config/config.toml");
    let config = config_loader::read_config_from_file(&dir).unwrap();

    let mut workers: HashMap<String, Mutex<Worker>> = HashMap::new();

    for server in config.servers.unwrap() {
        let listen = &server.listen;
        let worker = workers.get(&listen.clone().unwrap());

        if worker.is_some() {
            let mut worker = worker.unwrap().lock().unwrap();
            worker.add_site(server).unwrap();
        } else {
            let mut worker = Worker::new(&listen.clone().unwrap());
            worker.add_site(server.clone()).unwrap();
            workers.insert(listen.clone().unwrap(), Mutex::new(worker));
        }
    }

    let tasks: Vec<_> = workers
        .iter_mut()
        .map(|entry| {
            let mutex = entry.1;
            let worker = mutex.get_mut().unwrap();
            let future = worker.to_owned().start();
            return future;
        })
        .collect();

    future::join_all(tasks).await
}
