extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::time::Instant;

use ballista::cluster;

pub fn main() {
    let _ = ::env_logger::init();

    let now = Instant::now();

    let matches = App::new("Ballista")
        .version("0.1.0")
        .author("Andy Grove <andygrove73@gmail.com>")
        .about("Distributed compute platform")
        .subcommand(
            SubCommand::with_name("create-cluster")
                .about("Create a ballista cluster")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .takes_value(true)
                        .short("n")
                        .help("Ballista cluster name"),
                )
                .arg(
                    Arg::with_name("image")
                        .required(true)
                        .takes_value(true)
                        .short("i")
                        .help("Docker image containing ballista executor"),
                ),
                .arg(
                    Arg::with_name("executors")
                        .short("e")
                        .required(true)
                        .takes_value(true)
                        .help("number of executor pods to create"),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete-cluster")
                .about("Delete a ballista cluster")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .takes_value(true)
                        .short("n")
                        .help("Ballista cluster name"),
                ),
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Execute a ballista application")
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .takes_value(true)
                        .short("n")
                        .help("Ballista cluster name"),
                )
                .arg(
                    Arg::with_name("app")
                        .required(true)
                        .takes_value(true)
                        .short("a")
                        .help("Docker image containing application"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("create-cluster", Some(subcommand_matches)) => create_cluster(subcommand_matches),
        ("delete-cluster", Some(subcommand_matches)) => delete_cluster(subcommand_matches),
        ("run", Some(subcommand_matches)) => execute(subcommand_matches),
        _ => {
            println!("Invalid subcommand");
            std::process::exit(-1);
        }
    }

    println!(
        "Executed subcommand {} in {} seconds",
        matches.subcommand_name().unwrap(),
        now.elapsed().as_millis() as f64 / 1000.0
    );
}

fn create_cluster(matches: &ArgMatches) {
    let cluster_name = matches.value_of("name").unwrap();
    let image= matches.value_of("image").unwrap();
    let exec_node_count = matches
        .value_of("executors")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let namespace = "default";

    // create a cluster with 12 pods (one per month)
    for i in 1..=exec_node_count {
        let pod_name = format!("ballista-{}-{}", cluster_name, i);
        cluster::create_ballista_executor(namespace, &pod_name, image)
            .unwrap();
    }
}

fn delete_cluster(matches: &ArgMatches) {
    let cluster_name = matches.value_of("name").unwrap();
    let pod_name_prefix = format!("ballista-{}-", cluster_name);
    let namespace = "default";
    let all_pods = cluster::list_pods(namespace).unwrap();

    for name in all_pods {
        if name.starts_with(&pod_name_prefix) {
            cluster::delete_pod(namespace, &name).unwrap();
        }
    }
}

fn execute(matches: &ArgMatches) {
    let cluster_name = matches.value_of("name").unwrap();
    let image_name = matches.value_of("app").unwrap();
    let namespace = "default";

    let pod_name = format!("ballista-{}-app", cluster_name);

    cluster::create_pod(namespace, &pod_name, image_name).unwrap();
}