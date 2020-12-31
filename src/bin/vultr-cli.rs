use std::error::Error;
use vultr::VultrApi;
use vultr::VultrInstanceType;
use vultr::VultrOS;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Call program with the following:");
        println!("{} VULTR_API_KEY DOMAIN", args[0]);
        std::process::exit(1);
    }

    let result = do_stuff(&args[1], &args[2]);
    match result {
        Ok(_) => {
            println!("Finished sucessfully");
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn do_stuff(api_key: &str, domain: &str) -> Result<(), Box<dyn Error>> {
    let api = VultrApi::new(api_key);

    let account = api.get_account_info()?;
    println!("ACCOUNT: {:?}", account);

    let new_domain = api.create_dns_domain(domain, None, false)?;
    println!("CREATED DOMAIN: {:?}", new_domain);

    let old_domain = api.get_dns_domain(domain)?;
    println!("GET DOMAIN: {:?}", old_domain);

    let record = api.create_dns_domain_record(domain, "A", "www", "10.0.0.8", None, None)?;
    println!("RECORD CREATED: {:?}", record);

    let records = api.get_dns_domain_records(domain)?;
    println!("RECORDS: {:?}", records);

    let record = api.delete_dns_domain_record(domain, &record.id);
    println!("RECORD DELETED: {:?}", record);

    let domains = api.get_dns_domain_list()?;
    println!("DOMAIN LIST: {:?}", domains);

    let old_domain = api.delete_dns_domain(domain)?;
    println!("DEL DOMAIN: {:?}", old_domain);

    let regions = api.get_regions()?;
    println!("REGIONS: {:?}", regions);

    let plans = api.get_plans()?;
    println!("PLANS: {:?}", plans);

    let mut os = api.get_os_list()?;
    println!("OS: {:?}", os);

    let ssh_key = api.create_sshkey("test", "xxx")?;
    println!("SSH KEY CREATED: {:?}", ssh_key);

    let ssh_key = api.get_sshkey(ssh_key.id)?;
    println!("SSH KEY: {:?}", ssh_key);

    let ssh_keys = api.get_sshkey_list()?;
    println!("SSH KEYS: {:?}", ssh_keys);

    let instances = api.get_instance_list()?;
    println!("INSTANCE LIST: {:?}", instances);

    let ubuntu_list: Vec<VultrOS> = os
        .drain(..)
        .filter(|item| item.family.contains("ubuntu"))
        .collect();
    let instance = api.create_instance(
        &regions[0].id,
        &plans[0].id,
        VultrInstanceType::OS(ubuntu_list[0].id),
        true,
        "mylabel",
        &ssh_key.id,
        false,
        false,
        false,
        "myhostname",
        "mytag",
    )?;
    println!("INSTANCE CREATE: {:?}", instance);

    let ssh_key = api.delete_sshkey(ssh_key.id)?;
    println!("SSH KEY DELETED: {:?}", ssh_key);

    let instance = api.get_instance(instance.id)?;
    println!("INSTANCE GET: {:?}", instance);

    let instance = api.delete_instance(instance.id)?;
    println!("INSTANCE DELETE: {:?}", instance);

    Ok(())
}
