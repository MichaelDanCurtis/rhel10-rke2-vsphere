use crate::config::ImageConfig;

pub fn render(cfg: &ImageConfig) -> String {
    let version = cfg.rke2.kubernetes.rke2_version.replace('.', "-"); // "1.32" -> "1-32"
    let version_dot = &cfg.rke2.kubernetes.rke2_version; // "1.32"

    format!(
        r#"# RKE2 RPM repositories for RHEL 10 (auto-generated)

[rancher-rke2-common-latest]
name=Rancher RKE2 Common Latest
baseurl=https://rpm.rancher.io/rke2/latest/common/centos/10/noarch
enabled=1
gpgcheck=1
repo_gpgcheck=0
gpgkey=https://rpm.rancher.io/public.key

[rancher-rke2-{version}-latest]
name=Rancher RKE2 {version_dot} Latest
baseurl=https://rpm.rancher.io/rke2/latest/{version_dot}/centos/10/x86_64
enabled=1
gpgcheck=1
repo_gpgcheck=0
gpgkey=https://rpm.rancher.io/public.key
"#
    )
}
