const API_KEY: &str = "RGAPI-6e12b911-eac7-40d5-a800-0f9540e84b67";

use async_std;
use serde_json::Value;

#[async_std::main]
async fn main() -> surf::Result<()> {
    println!("Using API_KEY : {}", API_KEY);

    /*
    let mut res = surf::get("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/Raiigo44").header("X-Riot-Token", "RGAPI-6e12b911-eac7-40d5-a800-0f9540e84b67").await?;
    let mut match_list = surf::get("https://euw1.api.riotgames.com/lol/match/v4/matchlists/by-account/yzcdBgfpoTAu4v423cF9LwM7sTQqqkIkEIg1yLXPOq2zvLM").header("X-Riot-Token", "RGAPI-6e12b911-eac7-40d5-a800-0f9540e84b67").await?;
    println!("{}", res.body_string().await?);
    println!("{}", match_list.body_string().await?);
    */

    for elem in get_matchs("Raiigo44").await {
        println!("{}", elem);
    }

    println!("{}", surf::get("https://euw1.api.riotgames.com/lol/match/v4/matches/5225768529").header("X-Riot-Token", "RGAPI-6e12b911-eac7-40d5-a800-0f9540e84b67").recv_string().await?);

    println!("{}", get_kda("Raiigo44", "5270457260").await);

    Ok(())

}

async fn get_matchs(player_name: &str) -> Vec<String> {

    let mut http_summoner_info = String::from("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/");
    http_summoner_info.push_str(player_name);

    let summoner_info = surf::get(http_summoner_info).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();

    let summoner_info_value: Value = serde_json::from_str(&summoner_info).unwrap();

    let account_id = summoner_info_value["accountId"].to_string().replace("\"", "");

    let mut http_matchs = String::from("https://euw1.api.riotgames.com/lol/match/v4/matchlists/by-account/");
    http_matchs.push_str(&account_id);
    let matchs_info = surf::get(http_matchs).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();
    let match_value: Value = serde_json::from_str(&matchs_info).unwrap();

    let mut matchsId = Vec::<String>::new();
    for elem in match_value["matches"].as_array().unwrap() {
        println!("{}", elem["gameId"]);
        matchsId.push(elem["gameId"].to_string());
    }

    return matchsId;

}

async fn get_kda(summoner_name: &str, match_id: &str) -> String {

    let mut http_game = String::from("https://euw1.api.riotgames.com/lol/match/v4/matches/");
    http_game.push_str(match_id);

    let game_info = surf::get(http_game).header("X-Riot-Token", API_KEY).recv_string().await.unwrap();
    let game_value: Value = serde_json::from_str(&game_info).unwrap();

    let participant_identities: &Vec<Value> = game_value["participantIdentities"].as_array().unwrap();

    let mut participant_id: String = String::new();

    for elem in participant_identities {
        let current_participant_id = elem["participantId"].to_string();
        if elem["player"]["summonerName"] == summoner_name {
            participant_id = current_participant_id;
        }
    }

    let participants = game_value["participants"].as_array().unwrap();

    let mut kda = String::new();

    for elem in participants {
        if elem["participantId"].to_string() == participant_id {
            let kills = elem["stats"]["kills"].to_string();
            let deaths = elem["stats"]["deaths"].to_string();
            let assists = elem["stats"]["assists"].to_string();
            kda.push_str(&kills);
            kda.push_str("/");
            kda.push_str(&deaths);
            kda.push_str("/");
            kda.push_str(&assists);

        }
    }

    kda
}