use crate::to_string;

pub(crate) struct Anime {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) image: String,
    pub(crate) description: String,
}

#[derive(Debug)]
pub(crate) enum AnimeScrapeError {
    NetworkError,
    ParsingError,
    NoImageFound,
    NoSourceFound,
    NoArticleFound,
    NoAnchorFound,
}

const BASE_URL: &str = "https://www3.animeflv.net";
pub(crate) fn scrap_anime_browse(
    page: i32,
    genre: &Vec<String>,
    year: &Vec<String>,
    format_type: &Vec<String>,
    status: &Vec<i32>,
    search: String,
) -> Result<Vec<Anime>, AnimeScrapeError> {
    let url = get_url(page, genre, year, format_type, status, search);

    let response = reqwest::blocking::get(url);
    let html_content = response.and_then(|response| response.text());
    match html_content.map(|html_content| scraper::Html::parse_document(&html_content)) {
        Ok(document) => match scraper::Selector::parse("li article") {
            Ok(html_anime_selector) => {
                let mut animes: Vec<Anime> = vec![];
                for anime in document
                    .select(&html_anime_selector)
                    .map(|article| map_element_into_anime(article))
                {
                    if let Ok(anime) = anime {
                        animes.push(anime);
                    } else {
                        return Err(AnimeScrapeError::ParsingError);
                    }
                }
                Ok(animes)
            }
            Err(_error) => {
                return Err(AnimeScrapeError::ParsingError);
            }
        },
        Err(_error) => {
            return Err(AnimeScrapeError::NetworkError);
        }
    }
}

fn get_url(
    page: i32,
    genre: &Vec<String>,
    year: &Vec<String>,
    format_type: &Vec<String>,
    status: &Vec<i32>,
    search: String,
) -> String {
    let genre_query = genre
        .iter()
        .map(|genre_item| format!("&genre[]={genre_item}"))
        .collect::<Vec<String>>()
        .join("");
    let year_query = year
        .iter()
        .map(|year_item| format!("&year[]={year_item}"))
        .collect::<Vec<String>>()
        .join("");
    let format_type_query = format_type
        .iter()
        .map(|format_type_item| format!("&format_type[]={format_type_item}"))
        .collect::<Vec<String>>()
        .join("");
    let status_query = status
        .iter()
        .map(|status_item| to_string!(status_item))
        .collect::<Vec<String>>()
        .join("&status[]=");
    let search_query = format!("&q={search}");
    let url = format!("{BASE_URL}/browse?page={page}{genre_query}{year_query}{format_type_query}{status_query}{search_query}");
    url
}

fn map_element_into_anime(article: scraper::ElementRef) -> Result<Anime, AnimeScrapeError> {
    let anchor = get_anchor(article);

    let image = get_img(article);

    let description_selector = scraper::Selector::parse("div.Description p").unwrap();
    let description = article
        .select(&description_selector)
        .map(|description| description.text().collect())
        .collect::<Vec<String>>()
        .join(" ");

    let name_selector = scraper::Selector::parse("h3.Title").unwrap();
    let name = article
        .select(&name_selector)
        .map(|description| description.text().collect())
        .collect::<Vec<String>>()
        .join(" ");

    let url = format!("{}{}", BASE_URL, anchor?);
    let image = to_string!(image?);

    Ok(Anime {
        name,
        url,
        image,
        description,
    })
}

fn get_img(article: scraper::ElementRef) -> Result<&str, AnimeScrapeError> {
    match scraper::Selector::parse("img") {
        Ok(selector) => match article.select(&selector).next() {
            Some(image) => match image.attr("src") {
                Some(src) => Ok(src),
                None => Err(AnimeScrapeError::NoSourceFound),
            },
            None => return Err(AnimeScrapeError::NoImageFound),
        },
        Err(_error) => {
            return Err(AnimeScrapeError::ParsingError);
        }
    }
}

fn get_anchor(article: scraper::ElementRef) -> Result<&str, AnimeScrapeError> {
    match scraper::Selector::parse("a") {
        Ok(selector) => match article.select(&selector).next() {
            Some(anchor) => match anchor.attr("href") {
                Some(anchor) => Ok(anchor),
                None => Err(AnimeScrapeError::NoAnchorFound),
            },
            None => Err(AnimeScrapeError::NoArticleFound),
        },
        Err(_error) => {
            return Err(AnimeScrapeError::ParsingError);
        }
    }
}
