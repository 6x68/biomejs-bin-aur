use octocrab::instance;
use semver::Version;
use std::collections::HashMap;
use std::fmt::Display;
use std::process::exit;
use tokio::fs::{read_to_string, write};
use tracing::{Level, error, info, warn};
use tracing_subscriber::FmtSubscriber;

const REPO_OWNER: &str = "biomejs";
const REPO_NAME: &str = "biome";

const _67: u8 = 1; // line 6 column 7, very important for our code to work.
// const START_PAGE: u32 = 1;
// const PER_PAGE: u8 = 20;
const PKGBUILD_PATH: &str = "../PKGBUILD";

#[derive(Debug)]
enum GetErr {
    #[allow(dead_code)]
    IOErr(std::io::Error),
    NoPkgVerFound,
    NoLineAfterPkgVer,
}
impl From<std::io::Error> for GetErr {
    fn from(err: std::io::Error) -> GetErr {
        GetErr::IOErr(err)
    }
}

async fn get_pkgbuild_version() -> Result<String, GetErr> {
    let pkgbuild = read_to_string(PKGBUILD_PATH).await?;
    let split = pkgbuild
        .split("pkgver=")
        .nth(1)
        .ok_or_else(|| GetErr::NoPkgVerFound)?;
    let split = split
        .split("\n")
        .nth(0)
        .ok_or_else(|| GetErr::NoLineAfterPkgVer)?;
    Ok(split.to_owned())
}

async fn update_pkgbuild_version<T: Display>(to: T) -> std::io::Result<String> {
    let mut pkgbuild = read_to_string(PKGBUILD_PATH).await?;
    let lines = pkgbuild
        .lines()
        .map(|ln| {
            if ln.starts_with("pkgver=") {
                return format!("pkgver={to}");
            }
            ln.to_owned()
        })
        .collect::<Vec<_>>();
    pkgbuild = lines.join("\n");
    write(PKGBUILD_PATH, &pkgbuild).await?;
    Ok(pkgbuild)
}

/// Resets the pkgrel to `1`
async fn reset_pkgrel() -> std::io::Result<()> {
    let mut pkgbuild = read_to_string(PKGBUILD_PATH).await?;
    let lines = pkgbuild
        .lines()
        .map(|ln| {
            if ln.starts_with("pkgrel=") {
                return "pkgrel=1";
            }
            ln
        })
        .collect::<Vec<_>>();
    pkgbuild = lines.join("\n");
    write(PKGBUILD_PATH, &pkgbuild).await
}

async fn update_hashes<A: Display, B: Display>(
    x64_sum: Option<A>,
    aarch64_sum: Option<B>,
) -> std::io::Result<String> {
    let mut pkgbuild = read_to_string(PKGBUILD_PATH).await?;
    let lines = pkgbuild
        .lines()
        .map(|ln| {
            if let Some(sum) = &x64_sum
                && ln.starts_with("sha256sums_x86_64=")
            {
                return format!("sha256sums_x86_64=('{sum}')");
            }
            if let Some(sum) = &aarch64_sum
                && ln.starts_with("sha256sums_aarch64=")
            {
                return format!("sha256sums_aarch64=('{sum}')");
            }
            ln.to_owned()
        })
        .collect::<Vec<String>>();
    pkgbuild = lines.join("\n");
    write(PKGBUILD_PATH, &pkgbuild).await?;
    Ok(pkgbuild)
}

fn get_sha256<T: AsRef<str>>(digest: T) -> Option<String> {
    let digest = digest.as_ref();
    Some(
        digest
            .split("sha256:")
            .nth(1)?
            .split(" ")
            .nth(0)?
            .to_owned(),
    )
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
enum ArchType {
    Arm64,
    #[default]
    X64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // a builder for `FmtSubscriber`.
    let liker_and_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(liker_and_subscriber)
        .expect("setting default subscriber failed");

    info!("🎤 is this thing on?");

    let pkgbuild_version = get_pkgbuild_version()
        .await
        .expect("failed to parse pkgbuild for version");

    info!("Currently at: {pkgbuild_version}");

    let inst = instance();
    let repo = inst.repos(REPO_OWNER, REPO_NAME);
    let release = repo.releases().get_latest().await.expect("skill issue ahh");
    if !release.tag_name.starts_with("@biomejs/biome@") {
        error!("Latest release is not the biome CLI release");
        exit(1)
    }

    if release.prerelease {
        error!("Latest release is a prerelease");
        exit(1)
    }

    let version = release
        .tag_name
        .split("@biomejs/biome@")
        .nth(1)
        .expect("valid tag name");

    if version.eq(&pkgbuild_version) {
        return info!("Nothing to do, version === pkgbuild_version");
    }

    let pkgbuild_version =
        Version::parse(&pkgbuild_version).expect("valid semver version for pkgbuild");

    info!("Got latest biome version name: {version}");

    let version = Version::parse(version).expect("valid semver version for release");

    if pkgbuild_version > version {
        warn!("pkgbuild_version is > version! How could this happen..?");
    }

    if pkgbuild_version <= version {
        info!("Out of date! ({pkgbuild_version} < {version})");
        let _ = reset_pkgrel().await.map_err(|e| {
            error!("Failed to reset pkgrel to 1: {e:#?}")
        });
    }

    if pkgbuild_version == version {
        return info!("Nothing to do, pkgbuild_version == version");
    }

    update_pkgbuild_version(version)
        .await
        .expect("Failed to update pkgbuild version!");

    // assets and their sha256 digests.
    #[allow(unused_variables)]
    let assets_and_digests = release.assets.iter().filter_map(|a| {
        // don't want musl versions (our PKGBUILD doesn't support it)
        // or ones that aren't for Linux
        if let Some(digest) = &a.digest
            && a.name.starts_with("biome-linux-")
            && !a.name.ends_with("-musl")
        {
            let sha256 = get_sha256(digest);
            return Some((&a.name, sha256));
        }
        None
    });

    let mut hm: HashMap<ArchType, String> = HashMap::with_capacity(2);

    for (name, digest) in assets_and_digests {
        let arch_type = if name.ends_with("-arm64") {
            ArchType::Arm64
        } else {
            ArchType::X64
        };
        if let Some(sum) = digest {
            hm.insert(arch_type, sum);
        }
    }
    update_hashes(hm.get(&ArchType::X64), hm.get(&ArchType::Arm64))
        .await
        .expect("Failed to update hashes!");

    /*let releases = releases.iter().filter(|r| {
        !r.prerelease && r.tag_name.starts_with("@biomejs/biome@")
    }).collect::<Vec<_>>();*/
}

#[cfg(test)]
mod tests {
    use crate::get_pkgbuild_version;

    #[tokio::test]
    async fn test_get_current_version() {
        let v = get_pkgbuild_version()
            .await
            .expect("Failed to get pkgbuild version");
        println!("Version: {v}");
    }
}
