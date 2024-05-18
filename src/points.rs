use indexmap::IndexMap;
use msgpack_simple::parser;
use rmp_serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Instant;
use std::{collections::HashMap, fs::File, io::Read};

use std::str::FromStr;
use strum_macros::{EnumIter, EnumString};

#[derive(EnumString, PartialEq, Eq, Hash, Clone, Copy, Debug, Deserialize, Serialize, EnumIter)]
pub enum Category {
    Total,
    Novice,
    Moderate,
    Brutal,
    Insane,
    Dummy,
    #[strum(to_string = "DDmaX.Easy")]
    #[serde(rename = "DDmaX.Easy")]
    DDmaXEasy,
    #[strum(to_string = "DDmaX.Next")]
    #[serde(rename = "DDmaX.Next")]
    DDmaXNext,
    #[strum(to_string = "DDmaX.Pro")]
    #[serde(rename = "DDmaX.Pro")]
    DDmaXPro,
    #[strum(to_string = "DDmaX.Nut")]
    #[serde(rename = "DDmaX.Nut")]
    DDmaXNut,
    Oldschool,
    Solo,
    Race,
    Fun,
}

#[derive(Debug, Deserialize)]
struct Rank {
    name: String,
    points: u64,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct LeaderboardRank {
    points: u64,
    rank: u64,
}

#[derive(Deserialize, Debug)]
struct ServerRanks {
    _total_points: u64,
    points_ranks: Vec<Rank>,
    teampoints_ranks: Vec<Rank>,
    rankpoints_ranks: Vec<Rank>,
}

#[derive(Debug, Clone)]
pub struct Leaderboard {
    pub weekly_points: IndexMap<String, LeaderboardRank>,
    pub monthly_points: IndexMap<String, LeaderboardRank>,
    pub yearly_points: IndexMap<String, LeaderboardRank>,
    pub points: HashMap<Category, IndexMap<String, LeaderboardRank>>, // Category -> Name
    pub rank_points: HashMap<Category, IndexMap<String, LeaderboardRank>>, // Category -> Name
    pub team_points: HashMap<Category, IndexMap<String, LeaderboardRank>>, // Category -> Name
}

fn skip_msgpack(buf: &mut Vec<u8>) {
    // this sucks: I'm using a different messagepack
    // libary just to get the size of a messagepack
    let (_decoded, length) = parser::parse(buf).unwrap();
    buf.drain(..length);
}

fn create_leaderboard(ranks: &[Rank]) -> IndexMap<String, LeaderboardRank> {
    let mut leaderboard_map: IndexMap<String, LeaderboardRank> = IndexMap::new();
    let mut rank = 0;
    let mut prev_points = 0;
    let mut ranks_tied = 1;

    for player in ranks.iter() {
        if player.points != prev_points {
            // Tied players should have the same rank.
            rank += ranks_tied;
            ranks_tied = 1;
        } else {
            ranks_tied += 1;
        }

        leaderboard_map.insert(
            player.name.clone(),
            LeaderboardRank {
                points: player.points,
                rank,
            },
        );
        prev_points = player.points;
    }

    leaderboard_map
}
fn read_leaderboard(buf: &mut Vec<u8>) -> IndexMap<String, LeaderboardRank> {
    let mut deserializer = Deserializer::new(&buf[..]);
    let leaderboard: Vec<Rank> = Deserialize::deserialize(&mut deserializer).unwrap();

    skip_msgpack(buf);
    create_leaderboard(&leaderboard)
}

fn read_server_leaderboard(buf: &mut [u8]) -> HashMap<String, ServerRanks> {
    let mut deserializer = Deserializer::new(&*buf);
    let server_ranks: HashMap<String, ServerRanks> =
        Deserialize::deserialize(&mut deserializer).unwrap();
    server_ranks
}

pub fn parse_points() -> Leaderboard {
    let parse_points: bool = env::var("PARSE_POINTS")
        .unwrap_or("false".to_string())
        .parse()
        .unwrap();

    if !parse_points {
        return Leaderboard {
            weekly_points: IndexMap::new(),
            monthly_points: IndexMap::new(),
            yearly_points: IndexMap::new(),
            points: HashMap::new(),
            rank_points: HashMap::new(),
            team_points: HashMap::new(),
        };
    }

    let players_msgpack = "players.msgpack";
    let mut file = File::open(players_msgpack).expect("Unable to open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Unable to read file");

    let now = Instant::now();

    skip_msgpack(&mut data); // Categories
    skip_msgpack(&mut data); // Maps
    skip_msgpack(&mut data); // Total Points
    let points_leaderboard = read_leaderboard(&mut data); // Points rankings
    let weekly_points = read_leaderboard(&mut data); // Weekly points rankings
    let monthly_points = read_leaderboard(&mut data); // Monthly points rankings
    let yearly_points = read_leaderboard(&mut data); // Yearly points rankings
    let teampoints_leaderboard = read_leaderboard(&mut data); // Team points rankings
    let rankpoints_leaderboard = read_leaderboard(&mut data); // Rank points rankings

    let server_ranks = read_server_leaderboard(&mut data);

    let mut points: HashMap<Category, IndexMap<String, LeaderboardRank>> = HashMap::new();
    let mut rank_points: HashMap<Category, IndexMap<String, LeaderboardRank>> = HashMap::new();
    let mut team_points: HashMap<Category, IndexMap<String, LeaderboardRank>> = HashMap::new();

    for category in server_ranks.iter() {
        points.insert(
            Category::from_str(category.0).unwrap(),
            create_leaderboard(&category.1.points_ranks),
        );
        rank_points.insert(
            Category::from_str(category.0).unwrap(),
            create_leaderboard(&category.1.rankpoints_ranks),
        );
        team_points.insert(
            Category::from_str(category.0).unwrap(),
            create_leaderboard(&category.1.teampoints_ranks),
        );
    }

    // Total
    points.insert(Category::Total, points_leaderboard);
    rank_points.insert(Category::Total, rankpoints_leaderboard);
    team_points.insert(Category::Total, teampoints_leaderboard);
    // add totalpoints

    let leaderboard = Leaderboard {
        weekly_points,
        monthly_points,
        yearly_points,
        points,
        rank_points,
        team_points,
    };

    let elapsed = now.elapsed();
    tracing::info!("parsed players.msgpack in {:.2?}", elapsed);

    leaderboard
}
