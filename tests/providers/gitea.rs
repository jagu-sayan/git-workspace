mod helpers;
use testcontainers::{runners::AsyncRunner, ImageExt};
use testcontainers_modules::gitea::{self, Gitea, GiteaRepo};

#[tokio::test]
async fn default_gitea_server() {
    let server = helpers::get_gitea_server(22);
    let gitea = server
        .gitea
        .with_mapped_port(443, gitea::GITEA_HTTP_PORT)
        .with_mapped_port(22, gitea::GITEA_SSH_PORT)
        .start()
        .await
        .unwrap();

    // let url = format!(
    //     "http://localhost:{port}/api/v1/users/{}",
    //     gitea::GITEA_DEFAULT_ADMIN_USERNAME
    // );
    // GiteaRepo.

    // Anonymous query Gitea API for user info
    // let response = reqwest::Client::new().get(url).send().await.unwrap();
    // assert_eq!(response.status(), 200);
}
