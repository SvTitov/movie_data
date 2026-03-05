const HOST: &str = "http://www.omdbapi.com/apikey={}";

pub fn get_info(apikey: &str, title: &str) {
    let url = format!("{}{}", HOST, apikey);
}
