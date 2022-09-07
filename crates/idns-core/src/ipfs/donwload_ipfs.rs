use anyhow::Result;

// ipfs下载文档说明 https://docs.ipfs.tech/install/command-line/#system-requirements

const IPFS_VERSION: &'static str = "v0.15.0";

fn get_ipfs_archive_url() -> Result<String> {
    let mut url = String::new();
    if cfg!(target_os = "linux") {
        //
        url = format!(
            "https://dist.ipfs.tech/kubo/{}/kubo_{}_linux-amd64.tar.gz",
            IPFS_VERSION, IPFS_VERSION,
        );
    } else if cfg!(target_os = "windows") {
        //
        url = format!(
            "https://dist.ipfs.tech/kubo/{}/kubo_{}_windows-amd64.zip",
            IPFS_VERSION, IPFS_VERSION,
        );
    } else if cfg!(target_os = "macos") {
        //
        url = format!(
            "https://dist.ipfs.tech/kubo/{}/kubo_{}_darwin-amd64.tar.gz",
            IPFS_VERSION, IPFS_VERSION,
        );
    }
    Ok(url)
}

pub fn download_ipfs() -> Result<()> {
    //获取IPFS的下载路径
    let url = get_ipfs_archive_url()?;
    //进行文件的下载
    Ok(())
}
