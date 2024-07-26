use regex::{CaptureMatches, Regex};

pub(crate) struct AnimeEpisode {
    pub(crate) title: String,
    pub(crate) url: String,
    pub(crate) image: String,
    pub(crate) episode: String,
}
struct Episode {
    chapter: i16,
    id: i16,
}
pub(crate) fn scrap_anime_detail(url: &String) -> Option<Vec<AnimeEpisode>> {
    let response = reqwest::blocking::get(url).map(|response| response.text());
    match (response, scraper::Selector::parse("script:not(:empty)")) {
        (Ok(response), Ok(html_anime_selector)) => {
            match response.map(|html_content| {
                scraper::Html::parse_document(&html_content)
                    .select(&html_anime_selector)
                    .filter_map(|article| map_element_into_anime_episode(article))
                    .flat_map(|episode_option| episode_option.into_iter())
                    .collect::<Vec<AnimeEpisode>>()
            }) {
                Ok(html_animes_episode) => {
                    return Some(html_animes_episode);
                }
                Err(_error) => {
                    return None;
                }
            }
        }
        _ => {
            return None;
        }
    }
}

fn map_element_into_anime_episode(element: scraper::ElementRef) -> Option<Vec<AnimeEpisode>> {
    if let Some(script_content) = element.text().next() {
        match (
            extract_anime_info(script_content),
            extract_episodes_values(script_content),
        ) {
            (Some(anime_info), Some(episodes)) => Some(
                episodes
                    .into_iter()
                    .map(|episode| AnimeEpisode {
                        title: anime_info.title,
                        episode: format!("Episode {}", episode.chapter),
                        url: format!("/ver/{}-{}", anime_info.description, episode.chapter),
                        image: format!(
                            "https://cdn.animeflv.net/screenshots/{}/{}/th_3.jpg",
                            anime_info.id, episode.chapter
                        ),
                    })
                    .collect::<Vec<AnimeEpisode>>(),
            ),
            _ => None,
        }
    } else {
        None
    }
}

struct AnimeInfo {
    id: i16,
    title: String,
    description: String,
}

fn extract_anime_info(script_content: &str) -> Option<AnimeInfo> {
    match Regex::new(
        r"var anime_info = \[.(?P<id>[[:digit:]]+).,.(?P<title>[[:alnum:]]+).,.(?P<description>[[:alnum:]]+).\];",
    ) {
        Ok(anime_info_regex) => match anime_info_regex.captures(script_content) {
            Some(capture) => {
                match (
                    capture.name("id"),
                    capture.name("title"),
                    capture.name("description"),
                ) {
                    (Some(id), Some(title), Some(description)) => Some(AnimeInfo {
                        id: id.as_str().parse().unwrap_or(-1),
                        title: title.as_str().to_string(),
                        description: description.as_str().to_string(),
                    }),
                    _ => None,
                }
            }
            _ => {
                return None;
            }
        },
        Err(_error) => {
            return None;
        }
    }
}

fn extract_episodes_values(script_content: &str) -> Option<Vec<Episode>> {
    let capture_matches = get_capture_matches(script_content);

    capture_matches.into_iter().map( |capture| {
        match capture {
            
        }
    });
    return None;
}

fn get_capture_matches(script_content: &str) -> Option<CaptureMatches> {
    match (
        Regex::new(r"var episodes = (\[.*?\]);"),
        Regex::new(r"(\[((?P<chapter>[[:digit:]]+),(?P<id>[[:digit:]]+))+\])+"),
    ) {
        (Ok(episodes_var_regex), Ok(episodes_values_regex)) => episodes_var_regex
            .captures(script_content)
            .and_then(|capture| capture.get(1))
            .map(|regex_match| regex_match.as_str())
            .map(|episodes_str| episodes_values_regex.captures_iter(episodes_str)),

        _ => None,
    }
}
/* .map(|capture| Episode {
    chapter: capture.name("chapter").map(|chapter| chapter.as_str()),
    id: capture
        .name("id")
        .map(|id| id.as_str().parse().unwrap_or(-1)),
})
.collect()},*/
