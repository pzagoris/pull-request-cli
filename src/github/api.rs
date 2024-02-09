use crate::config::Config;
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT, AUTHORIZATION},
};
use serde::Deserialize;

/// according to the documentation: https://docs.github.com/en/rest/reference/pulls
/// the media type should be set as below
const MEDIA_TYPE_HEADER: &str = "application/vnd.github.v3+json"; 
const USER_AGENT_HEADER: &str = "User";

/// Client for the GitHub API.
pub struct GitHubClient {
	/// The underlying HTTP client.
	client: reqwest::blocking::Client,

	/// The headers to add to each request.
	headers: HeaderMap,
}


impl GitHubClient{

    pub fn new(config: Config) -> Result<Self, ()> {
        let client = reqwest::blocking::Client::new();
        let mut headers = HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_static(MEDIA_TYPE_HEADER));
            headers.insert(ACCEPT, HeaderValue::from_static(USER_AGENT_HEADER));
        let mut token: String = "token ".to_owned();
        if !config.github_token.is_empty(){
            token.push_str(&config.github_token);
            headers.insert(AUTHORIZATION, HeaderValue::from_str(&token).unwrap());
        }
        Ok(Self { client, headers })
    }

    pub fn get_pull_request(&self, url: &str) -> Result<PullRequest, ()> {
        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .map_err(|e| println!("Failed send GET request for pull request {}: {}", url, e))?
            .error_for_status()
            .map_err(|e| println!("Server returned an error when getting pull request: {}", e))?;

        let res: PullRequest= response.json().map_err(|e| println!("Error during the deserialization of Pull Request: {}", e))?;
        Ok(res)
    }

    pub fn get_pull_request_comments(&self, url: &str) -> Result<Vec<Comment>, ()> {
        let response = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .map_err(|e| println!("Failed send GET request for pull request comments {}: {}", url, e))?
            .error_for_status()
            .map_err(|e| println!("Server returned an error when getting pull reqeust comments: {}", e))?;

        let res: Vec<Comment>= response.json().map_err(|e| println!("Error during the deserialization: {}", e))?;
        Ok(res)
    }
}

/// The comment of a pull request.
#[derive(Debug, Deserialize)]
pub struct Comment{
    /// The text of a comment in a pull request.
    pub body: String,
}

/// Github pull request
#[derive(Debug, Deserialize)]
pub struct PullRequest{
    /// The text of a comment in a pull request.
    pub body: String,
    /// Number of comments in a pull request.
    pub comments: usize,
}
