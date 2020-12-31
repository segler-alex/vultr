use reqwest::blocking::Response;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
struct VultrAccountRoot {
    account: VultrAccount,
}

#[derive(Deserialize, Debug)]
pub struct VultrAccount {
    pub name: String,
    pub email: String,
    pub balance: f32,
    pub pending_charges: f32,
    pub last_payment_date: String,
    pub last_payment_amount: f32,
    pub acls: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct VultrDomainsRoot {
    domains: Vec<VultrDomain>,
}

#[derive(Deserialize, Debug)]
struct VultrDomainRoot {
    domain: VultrDomain,
}

#[derive(Deserialize, Debug)]
pub struct VultrDomain {
    pub domain: String,
    pub date_created: String,
}

#[derive(Deserialize, Debug)]
struct VultrDomainRecordsRoot {
    records: Vec<VultrDomainRecord>,
}

#[derive(Deserialize, Debug)]
struct VultrDomainRecordRoot {
    record: VultrDomainRecord,
}

#[derive(Deserialize, Debug)]
pub struct VultrDomainRecord {
    pub id: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub name: String,
    pub data: String,
    pub priority: i32,
    pub ttl: u32,
}

#[derive(Deserialize, Debug)]
pub struct VultrRegionsRoot {
    regions: Vec<VultrRegion>,
}

#[derive(Deserialize, Debug)]
pub struct VultrRegion {
    pub id: String,
    pub city: String,
    pub country: String,
    pub continent: String,
    pub options: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct VultrPlansRoot {
    plans: Vec<VultrPlan>,
}

#[derive(Deserialize, Debug)]
pub struct VultrPlan {
    pub id: String,
    pub vcpu_count: u8,
    pub ram: u32,
    pub disk: f32,
    pub bandwidth: f32,
    pub monthly_cost: f32,
    #[serde(rename = "type")]
    pub plan_type: String,
    pub locations: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct VultrOSRoot {
    os: Vec<VultrOS>,
}

#[derive(Deserialize, Debug)]
pub struct VultrOS {
    pub id: u32,
    pub name: String,
    pub arch: String,
    pub family: String,
}

#[derive(Deserialize, Debug)]
pub struct VultrSSHKeyRoot {
    ssh_key: VultrSSHKey,
}

#[derive(Deserialize, Debug)]
pub struct VultrSSHKeysRoot {
    ssh_keys: Vec<VultrSSHKey>,
}

#[derive(Deserialize, Debug)]
pub struct VultrSSHKey {
    pub id: String,
    pub date_created: String,
    pub name: String,
    pub ssh_key: String,
}

#[derive(Deserialize, Debug)]
pub struct VultrInstanceRoot {
    instance: VultrInstance,
}

#[derive(Deserialize, Debug)]
pub struct VultrInstancesRoot {
    instances: Vec<VultrInstance>,
}

#[derive(Deserialize, Debug)]
pub struct VultrInstance {
    pub id: String,
    pub os: String,
    pub ram: f32,
    pub disk: f32,
    pub main_ip: String,
    pub vcpu_count: u32,
    pub region: String,
    pub plan: String,
    pub date_created: String,
    pub status: String,
    pub allowed_bandwidth: f32,
    pub netmask_v4: String,
    pub gateway_v4: String,
    pub power_status: String,
    pub server_status: String,
    pub v6_network: String,
    pub v6_main_ip: String,
    pub v6_network_size: u64,
    pub label: String,
    pub internal_ip: String,
    pub kvm: String,
    pub tag: String,
    pub os_id: u32,
    pub app_id: u32,
    pub firewall_group_id: String,
    pub features: Vec<String>,
}

pub struct VultrApi {
    token: String,
}

#[derive(Deserialize, Debug)]
struct VultrError {
    error: String,
}

#[derive(Debug)]
struct VultrApiError {
    msg: String,
}

impl Display for VultrApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl Error for VultrApiError {}

impl<'a> VultrApi {
    pub fn new<S>(token: S) -> VultrApi
    where
        S: Into<String>,
    {
        VultrApi {
            token: token.into(),
        }
    }

    fn get(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client.get(url).bearer_auth(&self.token).send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: VultrError = resp.json()?;
            Err(Box::new(VultrApiError { msg: result.error }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    fn post(
        &self,
        url: &str,
        map: HashMap<&str, String>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .post(url)
            .bearer_auth(&self.token)
            .json(&map)
            .send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: VultrError = resp.json()?;
            Err(Box::new(VultrApiError { msg: result.error }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    fn delete(&self, url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let resp = client.delete(url).bearer_auth(&self.token).send()?;
        let status = resp.status();
        if status.is_client_error() {
            let result: VultrError = resp.json()?;
            Err(Box::new(VultrApiError { msg: result.error }))
        } else {
            Ok(resp.error_for_status()?)
        }
    }

    pub fn get_account_info(&self) -> Result<VultrAccount, Box<dyn std::error::Error>> {
        Ok(self
            .get("https://api.vultr.com/v2/account")?
            .json::<VultrAccountRoot>()?
            .account)
    }

    pub fn get_dns_domain_list(&self) -> Result<Vec<VultrDomain>, Box<dyn std::error::Error>> {
        Ok(self
            .get("https://api.vultr.com/v2/domains")?
            .json::<VultrDomainsRoot>()?
            .domains)
    }

    pub fn get_dns_domain<S>(&self, domain: S) -> Result<VultrDomain, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/domains/{}", domain.into());
        Ok(self.get(&url)?.json::<VultrDomainRoot>()?.domain)
    }

    pub fn delete_dns_domain<S>(&self, domain: S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/domains/{}", domain.into());
        self.delete(&url)?;
        Ok(())
    }

    pub fn create_dns_domain<S>(
        &self,
        domain: S,
        ip: Option<String>,
        dns_sec: bool,
    ) -> Result<VultrDomain, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let mut map = HashMap::new();
        map.insert("domain", domain.into());
        if let Some(ip) = ip {
            map.insert("ip", ip);
        }
        map.insert(
            "dns_sec",
            if dns_sec {
                String::from("enabled")
            } else {
                String::from("disabled")
            },
        );

        let url = "https://api.vultr.com/v2/domains";
        Ok(self.post(&url, map)?.json::<VultrDomainRoot>()?.domain)
    }

    pub fn create_dns_domain_record<S1, S2, S3, S4>(
        &self,
        domain: S1,
        record_type: S2,
        name: S3,
        ip: S4,
        ttl: Option<u32>,
        priority: Option<u32>,
    ) -> Result<VultrDomainRecord, Box<dyn std::error::Error>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("type", record_type.into());
        map.insert("name", name.into());
        map.insert("data", ip.into());
        if let Some(ttl) = ttl {
            map.insert("ttl", ttl.to_string());
        }
        if let Some(priority) = priority {
            let priority = priority.to_string();
            map.insert("priority", priority);
        }

        let url = format!("https://api.vultr.com/v2/domains/{}/records", domain.into());
        Ok(self
            .post(&url, map)?
            .json::<VultrDomainRecordRoot>()?
            .record)
    }

    pub fn get_dns_domain_records<S>(
        &self,
        domain: S,
    ) -> Result<Vec<VultrDomainRecord>, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.vultr.com/v2/domains/{}/records", domain.into());
        let resp: VultrDomainRecordsRoot =
            client.get(&url).bearer_auth(&self.token).send()?.json()?;
        Ok(resp.records)
    }

    pub fn delete_dns_domain_record<S1, S2>(
        &self,
        domain: S1,
        record_id: S2,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let url = format!(
            "https://api.vultr.com/v2/domains/{}/records/{}",
            domain.into(),
            record_id.into(),
        );
        self.delete(&url)?;
        Ok(())
    }

    pub fn get_plans(&self) -> Result<Vec<VultrPlan>, Box<dyn std::error::Error>> {
        let url = format!("https://api.vultr.com/v2/plans");
        Ok(self.get(&url)?.json::<VultrPlansRoot>()?.plans)
    }

    pub fn get_regions(&self) -> Result<Vec<VultrRegion>, Box<dyn std::error::Error>> {
        let url = format!("https://api.vultr.com/v2/regions");
        Ok(self.get(&url)?.json::<VultrRegionsRoot>()?.regions)
    }

    pub fn get_os_list(&self) -> Result<Vec<VultrOS>, Box<dyn std::error::Error>> {
        let url = format!("https://api.vultr.com/v2/os");
        Ok(self.get(&url)?.json::<VultrOSRoot>()?.os)
    }

    pub fn get_sshkey_list(&self) -> Result<Vec<VultrSSHKey>, Box<dyn std::error::Error>> {
        let url = format!("https://api.vultr.com/v2/ssh-keys");
        Ok(self.get(&url)?.json::<VultrSSHKeysRoot>()?.ssh_keys)
    }

    pub fn get_sshkey<S>(&self, key_id: S) -> Result<VultrSSHKey, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/ssh-keys/{}", key_id.into());
        Ok(self.get(&url)?.json::<VultrSSHKeyRoot>()?.ssh_key)
    }

    pub fn create_sshkey<S>(
        &self,
        name: S,
        ssh_key: S,
    ) -> Result<VultrSSHKey, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("name", name.into());
        map.insert("ssh_key", ssh_key.into());

        let url = format!("https://api.vultr.com/v2/ssh-keys");
        Ok(self.post(&url, map)?.json::<VultrSSHKeyRoot>()?.ssh_key)
    }

    pub fn delete_sshkey<S>(&self, key_id: S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/ssh-keys/{}", key_id.into());
        self.delete(&url)?;
        Ok(())
    }

    pub fn get_instance_list(&self) -> Result<Vec<VultrInstance>, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::new();
        let url = format!("https://api.vultr.com/v2/instances");
        let resp: VultrInstancesRoot = client.get(&url).bearer_auth(&self.token).send()?.json()?;
        Ok(resp.instances)
    }

    pub fn get_instance<S>(
        &self,
        instance_id: S,
    ) -> Result<VultrInstance, Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/instances/{}", instance_id.into());
        Ok(self.get(&url)?.json::<VultrInstanceRoot>()?.instance)
    }

    pub fn create_instance<S1, S2, S3, S4, S5, S6>(
        &self,
        region_id: S1,
        plan_id: S2,
        instance_type: VultrInstanceType,
        enable_ipv6: bool,
        label: S3,
        sshkey_id: S4,
        backups: bool,
        ddos_protection: bool,
        activation_email: bool,
        hostname: S5,
        tag: S6,
    ) -> Result<VultrInstance, Box<dyn std::error::Error>>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
        S5: Into<String>,
        S6: Into<String>,
    {
        let mut map: HashMap<&str, String> = HashMap::new();
        map.insert("region", region_id.into());
        map.insert("plan", plan_id.into());
        match instance_type {
            VultrInstanceType::OS(id) => map.insert("os_id", id.to_string()),
            VultrInstanceType::ISO(id) => map.insert("iso_id", id.to_string()),
            VultrInstanceType::Snapshot(id) => map.insert("snapshot_id", id.to_string()),
            VultrInstanceType::App(id) => map.insert("app_id", id.to_string()),
        };
        map.insert("enable_ipv6", enable_ipv6.to_string());
        map.insert("label", label.into());
        map.insert("sshkey_id", sshkey_id.into());
        map.insert("backups", backups.to_string());
        map.insert("ddos_protection", ddos_protection.to_string());
        map.insert("activation_email", activation_email.to_string());
        map.insert("hostname", hostname.into());
        map.insert("tag", tag.into());

        let url = format!("https://api.vultr.com/v2/instances");
        Ok(self.post(&url, map)?.json::<VultrInstanceRoot>()?.instance)
    }

    pub fn delete_instance<S>(&self, instance_id: S) -> Result<(), Box<dyn std::error::Error>>
    where
        S: Into<String>,
    {
        let url = format!("https://api.vultr.com/v2/instances/{}", instance_id.into());
        self.delete(&url)?;
        Ok(())
    }
}

pub enum VultrInstanceType {
    OS(u32),
    ISO(String),
    Snapshot(String),
    App(String),
}
