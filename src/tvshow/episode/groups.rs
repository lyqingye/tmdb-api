use std::borrow::Cow;

#[derive(Clone, Debug, Default)]
pub struct TVShowEpisodeGroups {
    pub tv_id: u64,
}

impl TVShowEpisodeGroups {
    pub fn new(tv_id: u64) -> Self {
        Self { tv_id }
    }
}

impl crate::prelude::Command for TVShowEpisodeGroups {
    type Output = crate::tvshow::EpisodeGroup;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/{}/episode_groups", self.tv_id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        vec![]
    }
}

#[derive(Clone, Debug, Default)]
pub struct TVShowEpisodeGroupsDetails {
    pub id: String,
    pub language: Option<String>,
}

impl TVShowEpisodeGroupsDetails {
    pub fn new(id: String) -> Self {
        Self { id, language: None }
    }

    pub fn with_language(mut self, language: Option<String>) -> Self {
        self.language = language;
        self
    }
}

impl crate::prelude::Command for TVShowEpisodeGroupsDetails {
    type Output = crate::tvshow::EpisodeGroupDetails;

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("/tv/episode_group/{}", self.id))
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        if let Some(language) = self.language.as_ref() {
            vec![("language", Cow::Borrowed(language.as_str()))]
        } else {
            vec![]
        }
    }
}

// #[cfg(all(test, feature = "integration"))]
mod integration_tests {
    use super::TVShowEpisodeGroups;
    use crate::client::reqwest::ReqwestExecutor;
    use crate::client::Client;
    use crate::prelude::Command;
    use crate::tvshow::episode::groups::TVShowEpisodeGroupsDetails;

    #[tokio::test]
    async fn test_groups() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = TVShowEpisodeGroups::new(127532)
            .execute(&client)
            .await
            .unwrap();
        println!("{:#?}", result);
        assert!(!result.results.is_empty());
    }

    #[tokio::test]
    async fn test_groups_details() {
        let secret = std::env::var("TMDB_TOKEN_V3").unwrap();
        let client = Client::<ReqwestExecutor>::new(secret);

        let result = TVShowEpisodeGroupsDetails::new("5ad845d20e0a26434200dcb6".to_owned())
            .with_language(Some("zh-CN".to_owned()))
            .execute(&client)
            .await
            .unwrap();
        println!("{:#?}", result);
        assert!(!result.groups.is_empty());
    }
}
