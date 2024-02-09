use crate::{GetCommentsCommand, config::Config};

mod api;
pub use api::*;

pub fn get_pull_request_comments(options: &GetCommentsCommand, github_client: GitHubClient, config: Config) -> Result<(), ()> {
    let pull_request_url = format!("{}/repos/{}/{}/pulls/{}", config.base_url, config.repo_owner, config.repo_name, options.pull_number);
    let pull_request = github_client.get_pull_request(pull_request_url.as_str())?;
   
    let comments_url = format!("{}/repos/{}/{}/issues/{}/comments?per_page={}", config.base_url, config.repo_owner, config.repo_name, options.pull_number, pull_request.comments);
    let comments = github_client.get_pull_request_comments(comments_url.as_str())?;

    println!("{}",pull_request.body);
    for comment in comments{
        println!("{}", comment.body);
    }
    Ok(())
}