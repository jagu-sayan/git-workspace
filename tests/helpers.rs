use ssh_key::{Algorithm::Ed25519, LineEnding, PrivateKey};
// use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::gitea::{Gitea, GiteaRepo};

pub fn generate_test_ssh_key() -> (String, String) {
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

pub struct GiteaServer {
    pub gitea: Gitea,
    pub private_key: String,
    pub username: String,
    pub password: String,
}

pub fn get_gitea_server(number_of_repositories: usize) -> GiteaServer {
    let (private_key, public_key) = generate_test_ssh_key();
    let (username, password) = ("jagu-sayan".to_string(), "42".to_string());
    let mut gitea = Gitea::default().with_admin_account(&username, &password, Some(public_key));
    for idx in 1..(number_of_repositories + 1) {
        gitea = gitea.with_repo(GiteaRepo::Private(
            format!("private-repo-{}", idx).to_string(),
        ));
    }
    GiteaServer {
        gitea,
        private_key,
        username,
        password,
    }
}
