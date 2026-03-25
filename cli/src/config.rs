use serde::{Deserialize, Serialize};

/// Top-level configuration for RHEL 10 RKE2 image builds.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageConfig {
    pub rke2: Rke2Config,
    pub os: OsConfig,
}

// ── RKE2 / Kubernetes ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rke2Config {
    pub node_role: NodeRole,
    pub kubernetes: KubernetesConfig,
    pub cni: Cni,
    pub networking: NetworkingConfig,
    pub tls: TlsConfig,
    pub cluster_join: ClusterJoinConfig,
    pub node: NodeConfig,
    pub etcd: EtcdConfig,
    pub security: SecurityConfig,
    pub components: ComponentsConfig,
    pub cloud_provider: CloudProviderConfig,
}

impl Default for Rke2Config {
    fn default() -> Self {
        Self {
            node_role: NodeRole::Server,
            kubernetes: KubernetesConfig::default(),
            cni: Cni::Canal,
            networking: NetworkingConfig::default(),
            tls: TlsConfig::default(),
            cluster_join: ClusterJoinConfig::default(),
            node: NodeConfig::default(),
            etcd: EtcdConfig::default(),
            security: SecurityConfig::default(),
            components: ComponentsConfig::default(),
            cloud_provider: CloudProviderConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NodeRole {
    Server,
    Agent,
    DedicatedEtcd,
    DedicatedControlPlane,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesConfig {
    pub rke2_version: String,
    pub channel: String,
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            rke2_version: "1.32".into(),
            channel: "stable".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Cni {
    Canal,
    Calico,
    Cilium,
    Flannel,
    Multus,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingConfig {
    pub cluster_cidr: String,
    pub service_cidr: String,
    pub cluster_dns: String,
    pub cluster_domain: String,
    pub service_node_port_range: String,
    pub disable_ipv6: bool,
    pub dual_stack: bool,
    pub dual_stack_cluster_cidr: String,
    pub dual_stack_service_cidr: String,
}

impl Default for NetworkingConfig {
    fn default() -> Self {
        Self {
            cluster_cidr: "10.42.0.0/16".into(),
            service_cidr: "10.43.0.0/16".into(),
            cluster_dns: "10.43.0.10".into(),
            cluster_domain: "cluster.local".into(),
            service_node_port_range: "30000-32767".into(),
            disable_ipv6: true,
            dual_stack: false,
            dual_stack_cluster_cidr: String::new(),
            dual_stack_service_cidr: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    pub tls_san: Vec<String>,
    pub write_kubeconfig_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterJoinConfig {
    pub server_url: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeConfig {
    pub node_name: String,
    pub node_ip: String,
    pub node_labels: Vec<String>,
    pub node_taints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EtcdConfig {
    pub snapshots_enabled: bool,
    pub snapshot_schedule_cron: String,
    pub snapshot_retention: u32,
    pub s3_backup: bool,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
}

impl Default for EtcdConfig {
    fn default() -> Self {
        Self {
            snapshots_enabled: true,
            snapshot_schedule_cron: "0 */12 * * *".into(),
            snapshot_retention: 5,
            s3_backup: false,
            s3_endpoint: String::new(),
            s3_bucket: String::new(),
            s3_region: "us-east-1".into(),
            s3_access_key: String::new(),
            s3_secret_key: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    pub cis_profile: String,
    pub pod_security_admission_config: String,
    pub selinux: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentsConfig {
    pub disable: Vec<String>,
    pub ingress_controller: String,
    pub enable_servicelb: bool,
}

impl Default for ComponentsConfig {
    fn default() -> Self {
        Self {
            disable: vec![],
            ingress_controller: "ingress-nginx".into(),
            enable_servicelb: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CloudProviderConfig {
    pub name: String,
    pub config_path: String,
}

// ── RHEL 10 OS Configuration ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsConfig {
    pub image: OsImageConfig,
    pub disk: DiskConfig,
    pub users: UsersConfig,
    pub packages: PackagesConfig,
    pub network: OsNetworkConfig,
    pub firewall: FirewallConfig,
    pub timezone: TimezoneConfig,
    pub kernel: KernelConfig,
    pub hardening: HardeningConfig,
    pub edge: EdgeConfig,
    pub custom_files: Vec<CustomFile>,
}

impl Default for OsConfig {
    fn default() -> Self {
        Self {
            image: OsImageConfig::default(),
            disk: DiskConfig::default(),
            users: UsersConfig::default(),
            packages: PackagesConfig::default(),
            network: OsNetworkConfig::default(),
            firewall: FirewallConfig::default(),
            timezone: TimezoneConfig::default(),
            kernel: KernelConfig::default(),
            hardening: HardeningConfig::default(),
            edge: EdgeConfig::default(),
            custom_files: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsImageConfig {
    pub base_image: String,
    pub output_type: String,
    pub hostname: String,
    pub image_version: String,
}

impl Default for OsImageConfig {
    fn default() -> Self {
        Self {
            base_image: "registry.redhat.io/rhel10/rhel-bootc:latest".into(),
            output_type: "anaconda-iso".into(),
            hostname: String::new(),
            image_version: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskConfig {
    pub root_size: String,
    pub rancher_size: String,
    pub extra_mounts: Vec<MountPoint>,
    pub partition_mode: String,
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            root_size: "20 GiB".into(),
            rancher_size: "60 GiB".into(),
            extra_mounts: vec![],
            partition_mode: "auto-lvm".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountPoint {
    pub path: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersConfig {
    pub admin_username: String,
    pub admin_password: String,
    pub ssh_keys: Vec<String>,
    pub extra_users: Vec<UserEntry>,
    pub extra_groups: Vec<GroupEntry>,
}

impl Default for UsersConfig {
    fn default() -> Self {
        Self {
            admin_username: "admin".into(),
            admin_password: "changeme".into(),
            ssh_keys: vec![],
            extra_users: vec![],
            extra_groups: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub name: String,
    pub groups: Vec<String>,
    pub shell: String,
    pub ssh_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntry {
    pub name: String,
    pub gid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackagesConfig {
    pub extra_packages: Vec<String>,
    pub package_groups: Vec<String>,
    pub enable_services: Vec<String>,
    pub disable_services: Vec<String>,
    pub embedded_containers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsNetworkConfig {
    pub method: String, // "dhcp" or "static"
    pub static_ip: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: Vec<String>,
    pub interface: String,
    pub advanced: Vec<AdvancedNetworkEntry>,
}

impl Default for OsNetworkConfig {
    fn default() -> Self {
        Self {
            method: "dhcp".into(),
            static_ip: String::new(),
            netmask: String::new(),
            gateway: String::new(),
            dns: vec![],
            interface: "link".into(),
            advanced: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedNetworkEntry {
    pub entry_type: String, // bond, vlan, bridge, team
    pub config: String,     // raw NM keyfile or nmcli args
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub open_ports: Vec<String>,
    pub open_services: Vec<String>,
    pub default_zone: String,
}

impl Default for FirewallConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            open_ports: vec![
                "22/tcp".into(),
                "6443/tcp".into(),
                "9345/tcp".into(),
                "10250/tcp".into(),
                "2379-2380/tcp".into(),
                "30000-32767/tcp".into(),
                "30000-32767/udp".into(),
            ],
            open_services: vec!["ssh".into()],
            default_zone: "public".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneConfig {
    pub timezone: String,
    pub ntp_servers: Vec<String>,
    pub locale: String,
    pub keyboard: String,
}

impl Default for TimezoneConfig {
    fn default() -> Self {
        Self {
            timezone: "UTC".into(),
            ntp_servers: vec![],
            locale: "en_US.UTF-8".into(),
            keyboard: "us".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelConfig {
    pub extra_kargs: Vec<String>,
    pub kernel_package: String,
    pub fips: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            extra_kargs: vec![],
            kernel_package: String::new(),
            fips: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardeningConfig {
    pub oscap_profile: String,
    pub tailoring_file: String,
    pub selinux_mode: String,
}

impl Default for HardeningConfig {
    fn default() -> Self {
        Self {
            oscap_profile: "none".into(),
            tailoring_file: String::new(),
            selinux_mode: "enforcing".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EdgeConfig {
    pub greenboot: bool,
    pub fdo: bool,
    pub ignition_config: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFile {
    pub source: String,
    pub destination: String,
}
