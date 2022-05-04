use anyhow::Result;
use reqwest::{
    blocking::Client,
    header::{
        ACCEPT, ACCEPT_LANGUAGE, CONNECTION, CONTENT_TYPE, COOKIE, DNT, HOST, ORIGIN, REFERER, TE,
        USER_AGENT,
    },
};
use serde::Deserialize;
use serde_aux::prelude::*;
use serde_json::Value;

fn main() {
    fetch_matches().expect("Failed to fetch matches");
}

#[derive(Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "isPostponed")]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub postponed: String, // might be int
    #[serde(rename = "Month")]
    pub month: String,
    #[serde(rename = "Year")]
    pub year: String,
    #[serde(rename = "Day")]
    pub day: String,
    #[serde(rename = "thePlace")]
    pub location: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "theOpponentString")]
    pub opponent: String,
    #[serde(rename = "isCancelled")]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub cancelled: String,
    #[serde(rename = "theTitle")]
    pub name: String,
    #[serde(rename = "homeOrAway")]
    #[serde(deserialize_with = "deserialize_string_from_number")]
    pub home: String,
    #[serde(rename = "theTime")]
    pub time: String,
    #[serde(rename = "rescheddate")]
    pub rescheduled_date: String,
}

fn fetch_matches() -> Result<()> {
    let client = Client::new();
    let resp = client
        .post("https://goffstownathletics.com/main/calendarWeekEvents")
        .header(HOST, "goffstownathletics.com")
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:100.0) Gecko/20100101 Firefox/100.0",
        )
        .header(ACCEPT, "application/json, text/javascript, */*; q=0.01")
        .header(ACCEPT_LANGUAGE, "en-US,en;q=0.5")
        .header(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .header("x-requested-with", "XMLHttpRequest")
        .header(ORIGIN, "https://goffstownathletics.com")
        .header(DNT, "1")
        .header(CONNECTION, "keep-alive")
        .header(REFERER, "https://goffstownathletics.com/main/calendar")
        .header(COOKIE, "wfx_unq=1UFMH2Zpyl68DE6k; cfid=e7e60dae-33cf-4fdf-aabd-38328c1adbc5; cftoken=0; ERD=30B2F5090B2BE8E73FA88560548E6651F6F90073D7B4BBBF1BEA2946CA9FADAA; CALDATE=5/3/2022; CALVIEW=week")
        .header("sec-fetch-dest", "empty")
        .header("sec-fetch-mode", "cors")
        .header("sec-fetch-site", "same-origin")
        .header(TE, "trailers").body("fromMonth=5&fromYear=2022&fromDay=3&toMonth=5&toYear=2022&toDay=7").send()?;

    let raw_games: Vec<Vec<Value>> = serde_json::from_str(&resp.text()?)?;

    let mut games = Vec::new();
    for raw_game in raw_games.get(1).unwrap() {
        dbg!(raw_game);
        let game: Event = serde_json::from_value(raw_game.clone()).unwrap();
        games.push(game);
    }

    dbg!(games);

    Ok(())
}
