
struct  Anime {
    name: String,
    url: String,
    image: String,
    description: String
}
const BASE_URL: &str = "https://www3.animeflv.net";
fn main() {
    let html_animes = scrap_anime_browse(1);

    for html_anime in html_animes {        
        println!("{} - {} - {} - {}", html_anime.name, html_anime.url, html_anime.image, html_anime.description);
    }
}

fn scrap_anime_browse(page: i32) -> Vec<Anime> {
    let response = reqwest::blocking::get( format!("{BASE_URL}/browse?page={page}"));
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);
    let html_anime_selector = scraper::Selector::parse("li article").unwrap();
    let html_animes = document
    .select(&html_anime_selector).map(|article| map_element_into_anime(article)).collect::<Vec<Anime>>();
    return html_animes;
}

fn map_element_into_anime(article: scraper::ElementRef) -> Anime {
    let anchor_selecctor = scraper::Selector::parse("a").unwrap();
    let anchor = article.select(&anchor_selecctor).next().unwrap().attr("href").unwrap();

    let image_selector = scraper::Selector::parse("img").unwrap();
    let image = article.select(&image_selector).next().unwrap().attr("src").unwrap().to_string();

    let description_selector = scraper::Selector::parse("div.Description p").unwrap();
    let description = article.select(&description_selector).map(|description| description.text().collect()).collect::<Vec<String>>().join(" ");
    
    let name_selector = scraper::Selector::parse("h3.Title").unwrap();
    let name = article.select(&name_selector).map(|description| description.text().collect()).collect::<Vec<String>>().join(" ");
    return Anime {
        name,
        url: format!("{BASE_URL}{anchor}"),
        image,
        description
    }
}
