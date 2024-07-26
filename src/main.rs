use anime_episode::scrap_anime_detail;

mod anime;
mod anime_episode;
mod macro_rules;

fn main() {
    let genre = vec![to_string!("accion"), to_string!("ciencia-ficcion")];
    let year = vec![to_string!("2024"), to_string!("2020")];
    let format_type = vec![to_string!("tv")];
    let status = vec![1];
    let search = to_string!("");

    if let Ok(html_animes) =
        anime::scrap_anime_browse(1, &genre, &year, &format_type, &status, search)
    {
        if let Some(first_value) = html_animes.get(0) {
            if let Some(scrap_anime_detail) = scrap_anime_detail(&first_value.url) {
                for anime_episode in scrap_anime_detail {
                    println!("{} - {}", anime_episode.title, anime_episode.url);
                }
            }
        }
    }
}
