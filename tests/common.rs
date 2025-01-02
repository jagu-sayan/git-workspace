use ssh_key::{Algorithm::Ed25519, LineEnding, PrivateKey};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::{ExitCode, Termination};
use testcontainers::core::ExecCommand;
use testcontainers::runners::SyncRunner;
use testcontainers::{Container, ImageExt};
use testcontainers_modules::gitea::GiteaRepo;
use testcontainers_modules::gitea::{Gitea, GITEA_HTTP_PORT, GITEA_SSH_PORT};

pub struct GiteaContainer {
    pub gitea: Container<Gitea>,
    pub url: String,
    pub ca: String,
    pub username: String,
    pub password: String,
    pub private_key: String,
    pub token: String,
}

impl GiteaContainer {
    fn generate_test_ssh_key() -> (String, String) {
        let private_key =
            PrivateKey::random(&mut rand::thread_rng(), Ed25519).expect("Failed to generate key");
        let public_key = private_key.public_key();

        // Convert to OpenSSH format strings
        let private_key_str = private_key
            .to_openssh(LineEnding::LF)
            .expect("Failed to serialize private key");
        let public_key_str = public_key
            .to_openssh()
            .expect("Failed to serialize public key");

        (private_key_str.to_string(), public_key_str.to_string())
    }

    pub fn start() -> Self {
        let (private_key, public_key) = Self::generate_test_ssh_key();
        let (username, password) = ("jagu".to_string(), "42".to_string());
        let gitea = Gitea::default()
            .with_admin_account(&username, &password, Some(public_key))
            .with_repo(GiteaRepo::Private("private-repo-1".to_string()))
            .with_repo(GiteaRepo::Private("private-repo-2".to_string()))
            .with_tls(true)
            .with_mapped_port(443, GITEA_HTTP_PORT)
            .with_mapped_port(22, GITEA_SSH_PORT)
            .start()
            .expect("to start the container");
        let url = "http://localhost".to_string();
        let ca = gitea.image().tls_ca().unwrap().to_string();

        // Generate token
        let command = ExecCommand::new(vec![
            "/usr/local/bin/gitea",
            "admin",
            "user",
            "generate-access-token",
            "--username",
            &username,
            "--scopes",
            "read:organization,read:repository",
        ]);
        let mut token = String::new();
        gitea
            .exec(command)
            .expect("to generate access token")
            .stdout()
            .read_to_string(&mut token)
            .unwrap();
        let token = token
            .split(":")
            .nth(1)
            .expect("to parse token from output")
            .trim()
            .to_string();

        Self {
            gitea,
            url,
            ca,
            username,
            password,
            private_key,
            token,
        }
    }

    pub fn save_cert(&self, path: &Path) -> std::io::Result<PathBuf> {
        // Create bundle.pem file
        let cert_path = path.join("bundle.pem");
        let mut file = File::create(&cert_path)?;
        file.write_all(self.ca.as_bytes())?;

        Ok(cert_path)
    }

    pub fn add_repo() {
        todo!()
    }

    pub fn archive_repo() {
        todo!()
    }

    pub fn remove_repo() {
        todo!()
    }
}

impl Termination for &'static GiteaContainer {
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

pub fn create_test_config(dir: &Path, filename: &str, content: &str) -> PathBuf {
    let config_path = dir.join(filename);
    let mut file = File::create(&config_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    config_path
}
