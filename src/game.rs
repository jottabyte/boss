//! Puts all the pieces together into a useable form, stitching together the play-by-play, with the boxscore, player, venue etc.
//! 
//! 
#![allow(unused)]

use crate::nathan::Data;
// use crate::metadata::MetaData;
use crate::players::{Player, SideCode as BatSideCode, SideDescription as BatSideDescription};
use crate::play_by_play::{RunnerData, Code, PlayEventType, Event, Trajectory, HalfInning, Hardness, SideCode, SideDescription, PitchTypeCode, PitchTypeDescription, AllPlays, PlateAppearanceData};
use crate::boxscore::{Pos, WeatherCondition, WindDirection, BoxScoreData};
use crate::schedule::{GameType, GameTypeDescription, AbstractGameState, GameMetaData, GameDate};
use crate::venues::{SurfaceType, RoofType, TimeZone, VenueData, VenueXY, Venue};
use crate::coaches::CoachData;
use crate::team::{TeamData, Team};
use crate::metadata::MetaData;
use crate::date::Date;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use core::f64::consts::*;


pub (crate) struct GameData <'m> {
    pub (crate) pitch_data: Vec<AllPlays>,
    pub (crate) meta_data: &'m MetaData,
    pub (crate) game_pk: u32,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PitcherSPRP {
    SP,
    RP,
}


/// Pitch is the final serializable struct that we'll export from this module. It will flatten all the at-bat level
/// data for easy use. This is intentionally de-normalized for ease of use. 
/// For ease of sorting, all counts (inning, outs balls, strikes, pitches etc. will start wit num_).
#[derive(Debug, Serialize, Deserialize)]
pub struct Pitch {
  
    pub play_id: String,

    //at_bat level meta-data
    pub half_inning: HalfInning,
    pub num_plate_appearance: u8,
    pub num_inning: u8,

    pub start_time: Option<String>,

    //Defense on the pitch. For now, use the starting lineups for everything. Update later to include subs
    //In practice, the positions should never be None. Its possible a boxscore won't be available for a game,
    //in which case we'd have no defense data. 
    pub catcher_id: Option<u32>,
    pub catcher_name: Option<String>,
    pub first_base_id: Option<u32>,
    pub first_base_name: Option<String>,
    pub second_base_id: Option<u32>,
    pub second_base_name: Option<String>,
    pub short_stop_id: Option<u32>,
    pub short_stop_name: Option<String>,
    pub third_base_id: Option<u32>,
    pub third_base_name: Option<String>,
    pub left_field_id: Option<u32>,
    pub left_field_name: Option<String>,
    pub center_field_id: Option<u32>,
    pub center_field_name: Option<String>,
    pub right_field_id: Option<u32>,
    pub right_field_name: Option<String>,

    //Umpires and coaches
    pub hp_umpire_id: Option<u32>,
    pub hp_umpire_name: Option<String>,
    pub hp_umpire_dob: String,
    pub hp_umpire_age: Option<f32>,
    pub hp_umpire_height: Option<u8>,
    pub hp_umpire_height_str: Option<String>,

    pub pitcher: u32,
    pub pitcher_team_id: u32,
    pub pitcher_team_name: String,
    pub pitcher_parent_team_id: u32,
    pub pitcher_parent_team_name: String,
    pub pitcher_throws: SideCode,
    pub pitcher_throws_desc: Option<SideDescription>,
    pub pitcher_name: String,
    pub pitcher_dob: String,
    pub pitcher_mlb_debut_date: String,
    pub pitcher_age: Option<f32>,
    pub pitcher_birth_city: Option<String>,
    pub pitcher_birth_state_province: Option<String>,
    pub pitcher_birth_country: Option<String>,
    pub pitcher_height_str: Option<String>,
    pub pitcher_height_in: u8,
    pub pitcher_weight: Option<u16>,
    pub pitcher_draft_school_name: Option<String>,
    pub pitcher_draft_year: Option<u16>,
    pub pitcher_draft_pick_number: Option<u16>,
    pub pitcher_fangraphs_id: Option<String>,
    pub pitcher_retrosheet_id: Option<String>,
    pub pitcher_highschool_city: Option<String>,
    pub pitcher_highschool_prov_state: Option<String>,
    pub pitcher_college_name: Option<String>,
    pub pitcher_sp_rp: PitcherSPRP,

    pub pitcher_num_pitch: u16,
    pub pitcher_num_plate_appearance: u16,

    pub batter: u32,
    pub batter_name: String,
    pub batter_team_id: u32,
    pub batter_team_name: String,
    pub batter_parent_team_id: u32,
    pub batter_parent_team_name: String,
    pub batter_dob: String,
    pub batter_mlb_debut_date: String,
    pub batter_age: Option<f32>,
    pub batter_birth_city: Option<String>,
    pub batter_birth_state_province: Option<String>,
    pub batter_birth_country: Option<String>,
    pub batter_height_str: Option<String>,
    pub batter_height_in: u8,
    pub batter_weight: Option<u16>,
    pub batter_draft_school_name: Option<String>,
    pub batter_draft_year: Option<u16>,
    pub batter_draft_pick_number: Option<u16>,
    pub batter_fangraphs_id: Option<String>,
    pub batter_retrosheet_id: Option<String>,
    pub batter_highschool_city: Option<String>,
    pub batter_highschool_prov_state: Option<String>,
    pub batter_college_name: Option<String>,

    pub batter_bats: SideCode,
    pub batter_bats_desc: Option<SideDescription>,
    pub batter_stands: Option<BatSideCode>,
    pub batter_stands_desc: Option<BatSideDescription>,
    pub batter_pos: Pos,
    pub batter_batting_order: Option<u16>,
    pub strike_zone_bottom: f32,
    pub strike_zone_top: f32,
    
    //pitch level meta-data
    pub pitch_num_plate_appearance: u8,
    pub pitch_num_inning: u8,
    ///Pitcher specific pitch num
    pub pitch_num_game: u16,   
    
    ///Did the pitch have a pickoff play right before it?
    pub preceded_by_pickoff: bool,

    //Is it possible to turn a double play on a ground ball? 
    pub double_play_opportunity: bool,

    //RE288 State
    pub balls_start: u8,
    pub balls_end: u8,
    pub strikes_start: u8,
    pub strikes_end: u8,
    pub outs_start: u8,
    pub outs_end: u8,
    pub base_value_start: u8,
    pub base_value_end: u8,
    pub runs_scored: u8,
    // is the batter responsible for the base/out/ball/strike state change?
    pub re_288_batter_responsible: bool,
    pub re_288_start: f32,
    pub re_288_end: f32,
    pub re_288_val: f32,
  
    //pitch-specific data, options for MLB-level data
    pub in_play: u8,
    pub swing: u8,
    // When swing == 1 this will be Some(0) or Some (1), Else None
    pub swing_and_miss: Option<u8>,
    pub foul: u8,
    pub bunt: Option<bool>,

    pub description: String,
    pub plate_appearance_description: String,
    pub plate_appearance_result: Option<Event>,

    pub pitch_speed_start: Option<f64>,
    pub pitch_speed_end: Option<f64>,
    pub pitch_break_vertical_induced: Option<f64>,
    pub pitch_break_horizontal: Option<f64>,
    pub pitch_spin_rate: Option<f64>,
    pub pitch_spin_direction: Option<f64>,
    pub pitch_plate_time: Option<f64>,
    pub pitch_extension: Option<f64>,
    pub pitch_pixels_x: Option<f64>,
    pub pitch_pixels_y: Option<f64>,
    pub pitch_a_x: Option<f64>,
    pub pitch_a_y: Option<f64>,
    pub pitch_a_z: Option<f64>,
    pub pitch_pfx_x: Option<f64>,
    pub pitch_pfx_z: Option<f64>,
    pub pitch_p_x: Option<f64>,
    pub pitch_p_z: Option<f64>,
    pub pitch_v_x0: Option<f64>,
    pub pitch_v_y0: Option<f64>,
    pub pitch_v_z0: Option<f64>,
    pub pitch_x0: Option<f64>,
    pub pitch_y0: Option<f64>,
    pub pitch_z0: Option<f64>,
    pub pitch_type_code: Option<PitchTypeCode>,
    pub pitch_type_desc: Option<PitchTypeDescription>,

    pub xr: Option<f64>,
    pub yr: Option<f64>,
    pub zr: Option<f64>,
    pub tr: Option<f64>,
    pub vxr: Option<f64>,
    pub vyr: Option<f64>,
    pub vzr: Option<f64>,
    pub tf: Option<f64>,
    pub vxbar: Option<f64>,
    pub vybar: Option<f64>,
    pub vzbar: Option<f64>,
    pub vbar: Option<f64>,
    pub vxhat: Option<f64>,
    pub vyhat: Option<f64>,
    pub vzhat: Option<f64>,
    pub ad: Option<f64>,
    pub atx: Option<f64>,
    pub aty: Option<f64>,
    pub atz: Option<f64>,
    pub atx_hat: Option<f64>,
    pub aty_hat: Option<f64>,
    pub atz_hat: Option<f64>,
    pub at: Option<f64>,
    pub phi_t: Option<f64>,
    pub ivb: Option<f64>,
    pub hb: Option<f64>,
    pub cd: Option<f64>,

    //1B, 2B, 3B, HR, strikeout, walk for easy summing in analytical tools
    pub in_play_result: Option<Event>,
    pub in_play_1b: Option<u8>,
    pub in_play_2b: Option<u8>,
    pub in_play_3b: Option<u8>,
    pub in_play_hr: Option<u8>,
    pub strikeout: u8,
    pub walk: u8,

    pub fielded_by_id: Option<u32>,
    pub fielded_by_pos: Option<Pos>,
    pub fielded_by_name: String,

    // hit data
    pub hit_data_coord_x: Option<f64>, 
    pub hit_data_coord_y: Option<f64>, 
    pub hit_data_trajectory: Option<Trajectory>,
    pub hit_data_contact_quality: Option<Hardness>,
    pub hit_data_launch_angle: Option<f32>,
    pub hit_data_exit_velocity: Option<f32>,
    pub hit_data_total_distance: Option<f32>,
    //Angle from 0 = 3B/LF Line to 90 1B/RF Line
    pub hit_data_spray_angle: Option<f64>,
    //distance calculated from spray chart
    pub hit_data_calc_distance: Option<f64>,





    // MetaData

    // Fields relevant to the level of play
    pub sport_id: u32,
    pub sport_code: String,
    pub sport_name: String,
    pub sport_abbr: String,
    pub sport_affilliation: crate::sports::MLB,
    pub sport_level_of_play: u8,
    
    pub team_name_home: String,
    pub team_name_away: String,


    // Game Level MetaData
    pub game_pk: u32,
    pub game_type: GameType,
    pub game_type_desc: GameTypeDescription,
    pub game_date: String,
    pub game_year: u16,
    pub game_month: u8,
    pub game_status: AbstractGameState,
    
    // Venue Metadata
    pub venue_id: u32,
    pub venue_home_plate_x: f32,
    pub venue_home_plate_y: f32,
    pub venue_name: String,
    pub venue_city: String,
    pub venue_state: String,
    pub venue_state_abbr: String,
    pub venue_time_zone: crate::venues::TimeZone,
    pub venue_time_zone_offset: i8,
    pub venue_capacity: Option<u32>,
    pub venue_surface: Option<crate::venues::SurfaceType>,
    pub venue_roof: Option<crate::venues::RoofType>,
    pub venue_left_line: Option<u16>,
    pub venue_left: Option<u16>,
    pub venue_left_center: Option<u16>,
    pub venue_center: Option<u16>,
    pub venue_right_center: Option<u16>,
    pub venue_right: Option<u16>,
    pub venue_right_line: Option<u16>,
    pub venue_retrosheet_id: String,
    pub venue_latitude: Option<f32>,
    pub venue_longitude: Option<f32>,

    pub league_name: String,

    //Boxscore MetaData
    pub game_attendance: Option<u32>,
    pub game_first_pitch: Option<f32>,
    pub game_weather_temp_f: Option<f32>,
    pub game_weather_temp_c: Option<f32>,
    pub game_weather_condition: Option<crate::boxscore::WeatherCondition>,
    pub game_wind_speed_mph: Option<u8>,
    pub game_wind_direction: Option<crate::boxscore::WindDirection>,   
    
}

// Get the player name for our player map and unwrap safely. If we don't have
// an id or a player name, return an empty string.
fn get_name (id: Option<u32>, player_map: &HashMap<u32, Player>) -> Option<String> {

    match id {
        Some (id) => 
            {
                let player = player_map.get(&id);
                match player {
                    Some (player) => Some(player.name.to_owned()),
                    None => None,
                }
            },
        None => None,
    }
}

// Get the coach data. We probably should just implement Default for Coach, but we're
// going to handle the null case here.
fn get_coach (id: Option<u32>, game_date: GameDate, coach_map: &HashMap<u32, Player>) -> 
    (Option<u32>, Option<String>, Option<Date>, Option<f32>, Option<bool>) 
{

    match id {
       Some (coach_id) =>
        {
            let coach = coach_map.get(&coach_id);
            match coach {
                Some (coach) => {
                    //Check here for a bad unwrap. Fix this later.
                    let age = match coach.birth_date {
                        Some (dob) =>  Some(game_date - dob.into()),
                        None => None,
                    };
                    (id, Some(coach.clone().name), coach.birth_date, age, Some(coach.mlb_debut_date.is_some()))
                },
                None => (id, None, None, None, None),
            }
        },
        None => (id, None, None, None, None),
    }

}


fn get_ump (id: Option<u32>, game_date: GameDate, ump_map: &HashMap<u32, Player>) -> 
    (Option<String>, Option<Date>, Option<f32>, Option<u8>, Option<String>) 
{

    match id {
       Some (ump_id) =>
        {
            let ump = ump_map.get(&ump_id);
            match ump {
                Some (ump) => (
                    {
                        let age = match ump.birth_date {
                        Some (dob) =>  Some(game_date - dob.into()),
                        None => None,
                        };
                        (Some(ump.clone().name), ump.birth_date, age, Some(ump.height_in), ump.clone().height_str)
                    }
                ),
                None => (None, None, None, None, None),
            }
        },
        None => (None, None, None, None, None),
    }

}


///Convert all the data about the game into a vector of pitches
impl <'m> From <GameData<'m>> for Vec<Pitch> {

    fn from (data: GameData) -> Vec<Pitch> {

        let plays = data.pitch_data;

        // 300 should be around the size of each game. This will minimize allocations
        let mut pitches: Vec<Pitch> = Vec::with_capacity(300);
        let game_pk = data.game_pk;
        
        //we start with the schedule, so we can safely unwrap here
        let sched_meta = data.meta_data.schedule.get(&game_pk).unwrap();
        let year: u16 = sched_meta.game_date.year;

        // We check here to make sure the game has a boxscore before processing. If we
        // don't have a boxscore, we'll be missing a LOT of critical information, such as the defense.
        // At some point, we may want to include games with no boxscore data, but for now we'll exclude them.
        // We handle all other possible missing metadata through Default impls, or explicitly in the code below.
        if !data.meta_data.boxscore.contains_key(&game_pk) {return vec![]};
        let box_meta = data.meta_data.boxscore.get(&game_pk).unwrap();

        // Handle the case where we don't have venue metadata
        let venue_meta = match data.meta_data.venue.get(&(sched_meta.game_venue_id, year)) {
            Some (venue) => venue.to_owned(),
            None => Venue::default(),
        };

        //Handle the case where we don't have coach metadata
        // let coaches = match data.meta_data.coaches.get(&game_pk) {
        //     Some (coaches) => coaches.to_owned(),
        //     None => CoachData::default(),
        // };

        // We handle the None case later
        let venue_x_y = data.meta_data.venue_x_y.get(&sched_meta.game_venue_id);
        
        let home_team = match data.meta_data.teams.get(&(box_meta.home_team_id, year)) {
            Some (team) => team.to_owned(),
            None => Team::default(),
        };
        
        let away_team = match data.meta_data.teams.get(&(box_meta.away_team_id, year)) {
            Some (team) => team.to_owned(),
            None => Team::default(),
        };
        
        let home_parent_team = match data.meta_data.teams.get(&(box_meta.home_parent_team_id, year)) {
            Some (team) => team.to_owned(),
            None => Team::default(),
        };
        
        let away_parent_team = match data.meta_data.teams.get(&(box_meta.away_parent_team_id, year)) {
            Some (team) => team.to_owned(),
            None => Team::default(),
        };
               
        let player_meta = data.meta_data.players.clone();
        
        let mut home_defense = box_meta.home_defense;
        let mut away_defense = box_meta.away_defense;

        let home_players: HashMap<u32, Option<u16>> = box_meta.home_players.iter().map(|p| (p.id, p.batting_order)).collect();
        let away_players: HashMap<u32, Option<u16>> = box_meta.away_players.iter().map(|p| (p.id, p.batting_order)).collect();

        let hp_umpire_id = box_meta.hp_umpire_id;
        let hp_details = get_ump(hp_umpire_id, sched_meta.game_date, &player_meta);

        let hp_umpire_dob = match hp_details.1 {
            Some (dob) => dob.to_string(),
            None => "".to_string(),
        };

        let sport_id = sched_meta.sport_id;
        let sport_details = crate::sports::get_sport(sport_id);

        let (venue_home_plate_x, venue_home_plate_y) = match venue_x_y {
            Some (venue) => (venue.x.unwrap_or(crate::STADIUM_X), venue.y.unwrap_or(crate::STADIUM_Y)),
            None => (crate::STADIUM_X, crate::STADIUM_Y),
        };

        // Set the initial half-inning state that we check against
        let mut previous_half_inning = HalfInning::Top;
        let mut base_value_start = 0u8;
        let mut base_value_end = 0u8;
        let mut outs_start = 0u8;
        let mut outs_end = 0u8;
        let mut pitch_num_inning = 0u8;
        
        let mut pitch_num_game = 0u16;

        let mut pitcher_num_pitch_game = (0u16, 0u16);
        let mut pitcher_num_plate_appearance_game = (0u16, 0u16);

        // Set the default SP/RP to SP, we'll update this later if there is a pitching sub
        let mut pitcher_sp_rp = (PitcherSPRP::SP, PitcherSPRP::SP);

        // We'll keep track of all runners in a vec, which we'll update after every plate appearance and
        // clear at the end of each half-inning. 
        let mut runner_state: HashMap<u32, RunnerData> = HashMap::new();

        for plate_app in plays {
            // Set the initial state for the half inning if the half inning has changed since the last plate appearance
            
            let mut preceded_by_pickoff = false;
            
            let mut pitch_num_plate_appearance = 0;
            
            let half_inning = plate_app.about.half_inning;
            let num_plate_appearance = plate_app.about.plate_appearance_index + 1;
            let num_inning = plate_app.about.inning_num;
            let batter = plate_app.matchup.batter_id;
            let batter_bats = plate_app.matchup.batter_bat_side_code;
            let batter_bats_desc = plate_app.matchup.batter_bat_side_desc;
            let pitcher = plate_app.matchup.pitcher_id;
            let pitcher_throws = plate_app.matchup.pitcher_pitch_hand_code;
            let pitcher_throws_desc = plate_app.matchup.pitcher_pitch_hand_desc;

            // Update the pitcher_num_plate_appearance (home, away) tuple
            match half_inning {
                HalfInning::Top => {
                    pitcher_num_plate_appearance_game.0 += 1;
                },
                HalfInning::Bottom => {
                    pitcher_num_plate_appearance_game.1 += 1;
                }
            }

            
            
            // Balls and Strike are always reset to 0 at the beginning of each plate appearance
            let mut balls_start = 0u8;
            let mut balls_end = 0u8;
            let mut strikes_start = 0u8;
            let mut strikes_end = 0u8;
            
            //If we see a different half-inning reset the base/out state as well as the pitch_num_inning
            if half_inning != previous_half_inning {
                base_value_start = 0;
                base_value_end = 0;
                outs_start = 0;
                outs_end = 0;
                pitch_num_inning = 0;
                runner_state.drain();
            }
            
            // Each plate appearance may only update a subset of active runners. We'll need to keep a
            // state machine to keep track of all active runners. 
            
            // let runner_data = plate_app.runners;


            // let batter_expect = format!("Couldn't find data for batter id: {}", &batter);
            // let pitcher_expect = format!("Couldn't find data for pitcher id: {}", &pitcher);
            
            // let batter_details = player_meta.get(&batter).unwrap_or_default().clone();
            // let pitcher_details = player_meta.get(&pitcher).unwrap_or_default().clone();

            let player_def = Player::default();
            
            let batter_details = match player_meta.get(&batter) {
                Some (batter_det) => batter_det,
                None => {
                    // dbg!(&batter);
                    &player_def
                },
            };

            let pitcher_details = match player_meta.get(&pitcher) {
                Some (pitcher_det) => pitcher_det,
                None => {
                    // dbg!(&pitcher);
                    &player_def
                },
            };

            let batter_dob = match batter_details.birth_date {
                Some (dob) => dob.to_string(),
                None => "".to_string(),
            };
            let pitcher_dob = match pitcher_details.birth_date {
                Some (dob) => dob.to_string(),
                None => "".to_string(),
            };
            let batter_mlb_debut_date = match batter_details.mlb_debut_date {
                Some (dob) => dob.to_string(),
                None => "".to_string(),
            };
            let pitcher_mlb_debut_date = match pitcher_details.mlb_debut_date {
                Some (dob) => dob.to_string(),
                None => "".to_string(),
            };

            // let (batting_coach, pitching_coach, batting_manager, pitching_manager) = match half_inning {
            //     HalfInning::Top =>
            //         ( coaches.away_coaches.batting_coach, coaches.home_coaches.pitching_coach,
            //           coaches.away_coaches.manager, coaches.home_coaches.manager,
            //         ),
            //     HalfInning::Bottom =>
            //         ( coaches.home_coaches.batting_coach, coaches.away_coaches.pitching_coach,
            //           coaches.home_coaches.manager, coaches.away_coaches.manager,
            //         ),
            // };

            // let batting_coach_details = get_coach(batting_coach, sched_meta.game_date, &player_meta);
            // let pitching_coach_details = get_coach(pitching_coach, sched_meta.game_date, &player_meta);
            // let batting_manager_details = get_coach(batting_manager, sched_meta.game_date, &player_meta);
            // let pitching_manager_details = get_coach(pitching_manager, sched_meta.game_date, &player_meta);

            // let batting_coach_dob = match batting_coach_details.2 {
            //     Some (dob) => dob.to_string(),
            //     None => "".to_string(),
            // };

            // let pitching_coach_dob = match pitching_coach_details.2 {
            //     Some (dob) => dob.to_string(),
            //     None => "".to_string(),
            // };

            // let batting_manager_dob = match batting_manager_details.2 {
            //     Some (dob) => dob.to_string(),
            //     None => "".to_string(),
            // };

            // let pitching_manager_dob = match pitching_manager_details.2 {
            //     Some (dob) => dob.to_string(),
            //     None => "".to_string(),
            // };


            //Set the defensive and offensive players
            let (mut defense, players) = match half_inning {
                HalfInning::Top => (home_defense, away_players.clone()),
                HalfInning::Bottom => (away_defense, home_players.clone()),
            };

            let batter_age = match batter_details.birth_date {
                Some (age) => Some(sched_meta.game_date - age.into()),
                None => None,
            };

            let pitcher_age = match pitcher_details.birth_date {
                Some (age) => Some(sched_meta.game_date - age.into()),
                None => None,
            };

            let (batter_team_id, batter_team_name, batter_parent_team_id, batter_parent_team_name) = match half_inning {
                HalfInning::Top => (away_team.id, away_team.clone().team_city_name, away_parent_team.id, away_parent_team.clone().team_city_name),
                HalfInning::Bottom => (home_team.id, home_team.clone().team_city_name, home_parent_team.id, home_parent_team.clone().team_city_name),
            };

            let (pitcher_team_id, pitcher_team_name, pitcher_parent_team_id, pitcher_parent_team_name) = match half_inning {
                HalfInning::Top => (home_team.id, home_team.clone().team_city_name, home_parent_team.id, home_parent_team.clone().team_city_name),
                HalfInning::Bottom => (away_team.id, away_team.clone().team_city_name, away_parent_team.id, away_parent_team.clone().team_city_name),
            };

            let mut re_288_batter_responsible = true;

            //Some plays don't have any events, but have runner events. We'll update those here in that case
            //Still have an issue if this is a run scoring event, don't know how to fix that yet
            if plate_app.play_events.len() == 0 {
                // First, we need to de-duplicate the runner data. We'll take the last record for each runner
                // and assume that it's the correct one. This may or may not be accurate. Based on the source code in std,
                // I think this this will always take the last one, but not positive.
                let runner_data: HashMap<u32, RunnerData> =
                    plate_app.runners.clone().into_iter()
                    .filter(|r| r.play_index == -1)    
                    .map(|r| (r.runner_id, r))
                    .collect();
                
                // We update our runner state with the new runner data. This will overwrite the old values, but more
                // importantly, it will keep all the old values. For outs, we'll use just the runner_data, for base value
                // we'll use our runner_state.
                for runner in runner_data.values() {
                    runner_state.insert(runner.runner_id, *runner);
                };


                re_288_batter_responsible = false;
                base_value_end = runner_state.values()
                .map (|r| r.end_base_value)
                .sum();
            
                outs_end = outs_start + runner_data.values()
                .filter(|v| v.play_index == -1)
                .map (|r| r.outs)
                .sum::<u8>();
            }

            for event in plate_app.play_events {


                // First, we need to de-duplicate the runner data. We'll take the last record for each runner
                // and assume that it's the correct one. This may or may not be accurate. Based on the source code in std,
                // I think this this will always take the last one, but not positive.
                let runner_data: HashMap<u32, RunnerData> =
                    plate_app.runners.clone().into_iter()
                    .filter(|r| r.play_index == event.index as i8)    
                    .map(|r| (r.runner_id, r))
                    .collect();
                
                // We update our runner state with the new runner data. This will overwrite the old values, but more
                // importantly, it will keep all the old values. For outs, we'll use just the runner_data, for base value
                // we'll use our runner_state.
                for runner in runner_data.values() {
                    runner_state.insert(runner.runner_id, *runner);
                };

                let (fielded_by_id, fielded_by_pos) = plate_app.runners.clone().into_iter()
                    .filter(|r| r.play_index == event.index as i8)
                    .filter(|r| r.fielded_by_pos.is_some())
                    .map(|r| (r.fielded_by_id, r.fielded_by_pos))
                    .nth(0).unwrap_or((None, None))
                    ;

                let fielded_by_name = match fielded_by_id {
                    Some(id) => match player_meta.get(&id) {
                        Some(player) => player.name.clone(),
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };


                // Our runner state is persistent, so we don't need to match it to the specific pitch.
                base_value_end = runner_state.values()
                                                .map (|r| r.end_base_value)
                                                .sum();
                // PROBLEM: If a run scores on a non-pitch event (or base state changes) we aren't capturing that.
                // Runs and outs are only relevant for that pitch, so we match up the pitch specific event data
                let runs_scored = runner_data.values()
                                                .filter(|v| v.play_index == event.index as i8)
                                                .map (|r| r.runs)
                                                .sum();

                outs_end = outs_start + runner_data.values()
                                                .filter(|v| v.play_index == event.index as i8)
                                                .map (|r| r.outs)
                                                .sum::<u8>();


                // if runner_data.len() > 0 {
                //     if runner_data[0].play_index == -1 {base_value_end = runner_data[0].end_base_value}
                // }
                // ;

                match event.play_event_type {
                    PlayEventType::Action => {
                        //Update the defense here.
                        
                        match event.details.event {
                            // Substitution will have one entry, while switch will have at least 2. We don't
                            // care who the player being switched out is, since we just overwrite the position. It also
                            // doesn't matter who is subbing in for who, the position that that player moves to is all
                            // we care about, since if a player is moving, he'll have another entry.
                            Some(Event::DefensiveSubstitution) | Some(Event::DefensiveSwitch) => {
                                // We should have player and position info for every defensive switch, however,
                                // this will panic for DHs, who have no position. so we need to check for that.
                                // If we don't have a position for the batter, we just assume they're a DH.
                                let player_id = event.player.unwrap().id;
                                let position = match event.position {
                                    Some (pos) => pos.abbreviation,
                                    _ => Pos::DesignatedHitter,
                                };
                                                                
                                match position {
                                    Pos::Catcher =>          {defense.catcher =           Some(player_id)},
                                    Pos::FirstBase =>        {defense.first_base =        Some(player_id)},
                                    Pos::SecondBase =>       {defense.second_base =       Some(player_id)},
                                    Pos::ShortStop =>        {defense.short_stop =        Some(player_id)},
                                    Pos::ThirdBase =>        {defense.third_base =        Some(player_id)},
                                    Pos::LeftField =>        {defense.left_field =        Some(player_id)},
                                    Pos::RightField =>       {defense.right_field =       Some(player_id)},
                                    Pos::CenterField =>      {defense.center_field =      Some(player_id)},
                                    Pos::DesignatedHitter => {defense.designated_hitter = Some(player_id)},
                                    _ => {},
                                };
                                
                                // Update the home_defense and or away_defense since we switch the defense each half inning.
                                if half_inning == HalfInning::Top    {home_defense = defense;};
                                if half_inning == HalfInning::Bottom {away_defense = defense;};

                            },
                            //If we have an offensive substitution and a relevant base, we need to update our runner_state
                            Some(Event::OffensiveSubstitution) => {

                                match event.base {
                                    Some (base) => {
                                        runner_state = runner_state.values()
                                            .filter (|runner| runner.end_base_value != 2u8.pow(base as u32 -1))
                                            .map (|runner| (runner.runner_id, *runner))
                                            .collect();
                                    },
                                    _ => {}
                                }
                            },
                            Some(Event::PitchingSubstitution) => {
                                            // Update the pitcher_num_plate_appearance (home, away) tuple
                                match half_inning {
                                    HalfInning::Top => {
                                        pitcher_sp_rp.0 = PitcherSPRP::RP;
                                        pitcher_num_plate_appearance_game.0 = 0;
                                        pitcher_num_pitch_game.0 = 0;

                                    },
                                    HalfInning::Bottom => {
                                        pitcher_sp_rp.1 = PitcherSPRP::RP;
                                        pitcher_num_plate_appearance_game.1 = 0;
                                        pitcher_num_pitch_game.1 = 0;
                                    }
                                }
                            }


                            // Do nothing for all other event types for now, except mark the batter as not responsible
                            // I'm not entirely sure if this works properly yet, but we are ignoring all base/out state changes
                            // that don't result from a ball/strike/foul/in-play. We also have no way of taking away responsibility
                            // for hit-and-runs that end up as caught stealing.
                            _ => {re_288_batter_responsible = false;},
                        }
                        
                    }
                    PlayEventType::Pickoff => {
                        preceded_by_pickoff = true;
                    }
                    PlayEventType::NoPitch | PlayEventType::Stepoff => {
                        // Do Nothing Here
                    }
                    PlayEventType::Pitch => {
                        
                        pitch_num_game += 1;
                        pitch_num_plate_appearance += 1;
                        pitch_num_inning +=1;

                        // Update the pitcher_num_pitch_game (home, away) tuple
                        match half_inning {
                            HalfInning::Top => {
                                pitcher_num_pitch_game.0 += 1;
                            },
                            HalfInning::Bottom => {
                                pitcher_num_pitch_game.1 += 1;
                            }
                        }

                        // We need the defense that's off the field to find the batter's current position in the game
                        let defense_to_use_for_batter_pos = match half_inning {
                            HalfInning::Top => away_defense,
                            HalfInning::Bottom => home_defense,
                        };

                        let batter_pos = {
                                 if defense_to_use_for_batter_pos.catcher ==           Some(batter)  {Pos::Catcher}
                            else if defense_to_use_for_batter_pos.first_base ==        Some(batter)  {Pos::FirstBase}
                            else if defense_to_use_for_batter_pos.second_base ==       Some(batter)  {Pos::SecondBase}
                            else if defense_to_use_for_batter_pos.short_stop ==        Some(batter)  {Pos::ShortStop}
                            else if defense_to_use_for_batter_pos.third_base ==        Some(batter)  {Pos::ThirdBase}
                            else if defense_to_use_for_batter_pos.left_field ==        Some(batter)  {Pos::LeftField}
                            else if defense_to_use_for_batter_pos.right_field ==       Some(batter)  {Pos::RightField}
                            else if defense_to_use_for_batter_pos.center_field ==      Some(batter)  {Pos::CenterField}
                            else if defense_to_use_for_batter_pos.pitcher ==           Some(batter)  {Pos::Pitcher}
                            else if defense_to_use_for_batter_pos.designated_hitter == Some(batter)  {Pos::DesignatedHitter}
                            else {Pos::Bench}
                        };

                        let (in_play_result, in_play_1b, in_play_2b, in_play_3b, in_play_hr) = match event.details.is_in_play.unwrap() {
                            true => {
                                let in_play_result = plate_app.result.plate_appearance_result.unwrap_or(Event::Other);
                                let in_play_1b = if in_play_result == Event::Single {1} else {0};
                                let in_play_2b = if in_play_result == Event::Double {1} else {0};
                                let in_play_3b = if in_play_result == Event::Triple {1} else {0};
                                let in_play_hr = if in_play_result == Event::HomeRun {1} else {0};
                                (Some(in_play_result), Some(in_play_1b), Some(in_play_2b), Some(in_play_3b), Some(in_play_hr))
                            },
                            false => (None, None, None, None, None),
                        };
            
                        let mut strikeout = 0;
                        let mut walk = 0;
            
                        let mut swing = 0;
                        let mut swing_and_miss = None;
                        let mut foul = 0;

                        match event.details.code.unwrap() {
                            // Ball or Ball in Dirt or Hit Batter
                            Code::BD | Code::B | Code::P | Code::I | Code::H | Code::VS | Code::VC | Code::VB | Code::VP | Code::V  => {
                                // balls_end = balls_start + 1;
                                
                                swing = 0;
                            },
                            
                            // Called Strike or Automating strike
                            Code::C | Code::A | Code::AC | Code::AB => {
                                // strikes_end = strikes_start +1;
                                swing = 0;
                            },
                            
                            //Swinging Strike or Swinging Strike Blocked or missed bunt
                            Code::S  | Code::W | Code::Q | Code::M => {
                                // strikes_end = strikes_start + 1;
                                swing = 1;
                                swing_and_miss = Some(1);
                            },

                            //Foul Ball
                            Code::F | Code::R | Code::L  => {
                                foul = 1;
                                // if strikes_start < 2 {strikes_end = strikes_start + 1};
                                swing = 1;
                                swing_and_miss = Some(0);
                            },
                            // Foul Tip
                            Code::T | Code::O=> {
                                foul = 1;
                                // if strikes_start < 2 {strikes_end = strikes_start + 1};
                                swing = 1;
                                swing_and_miss = Some(1);
                            },

                            //In Play
                            Code::D | Code::E | Code::X | Code::J| Code::Y  | Code::Z => {
                                swing = 1;
                                swing_and_miss = Some (0);
                            },

                            // No Pitch
                            Code::N | Code::PSO | Code::PO => {

                            }

                            // All other cases, such as POs we PANIC as we want to classify them all
                            // _ => {panic!("Missing logic for Code")}
                        };
                        
                        // There are in some cases extra pitches, so we always correct to the count given
                        // in the data. This *might* lead to double counting of walks and strikeouts, if this pitch
                        // is duplicated in the data. Should check for this at some point. 
                        let balls_end = event.count.balls.unwrap();
                        let strikes_end = event.count.strikes.unwrap();

                        if strikes_end == 3 {strikeout = 1;};
                        if balls_end == 4 {walk = 1};


                        //if our event type is a pitch, we can safely unwrap the pitch_data
                        let pitch_data = event.pitch_data.unwrap();
                        
                        let (pitch_break_vertical_induced, pitch_break_horizontal, pitch_spin_rate, pitch_spin_direction) = match pitch_data.breaks {
                            Some (breaks) => 
                                (breaks.break_vertical_induced, breaks.break_horizontal, breaks.spin_rate,breaks.spin_direction),
                            None => (None, None, None, None),
                        };

                        let (pitch_type_code, pitch_type_desc) = match event.details.pitch_type {
                            Some (pitch_type) => (Some(pitch_type.code.unwrap_or(PitchTypeCode::UN)), Some(pitch_type.description.unwrap_or(PitchTypeDescription::Unknown))),
                            None => (None, None),
                        };

                        let 
                            (   hit_data_coord_x, hit_data_coord_y,
                                hit_data_contact_quality,  hit_data_trajectory,
                                hit_data_exit_velocity, hit_data_launch_angle, hit_data_total_distance,
                                bunt 
                            ) = match event.hit_data {
                            
                            Some (hit_data) => 
                                {
                                let (x,y) = match hit_data.coordinates {
                                    Some(c) => (c.x, c.y),
                                    None => (None, None),
                                };
                                let bunt = match hit_data.trajectory {
                                    Some(Trajectory::BuntGroundBall) |
                                    Some(Trajectory::BuntLineDrive) |
                                    Some(Trajectory::BuntPopUp) 
                                        => Some (true),
                                    Some(_) => Some (false),
                                    None => None,
                                };
                                (x, y, hit_data.hardness, hit_data.trajectory,
                                hit_data.launch_speed, hit_data.launch_angle, hit_data.total_distance, bunt,
                            )},
                            None => (None, None, None, None, None, None, None, None),
                        };

                        // Calculate the spray angle and the hit distance (in pixels)
                        let (hit_data_spray_angle, hit_data_calc_distance) = match (hit_data_coord_x, hit_data_coord_y) {
                            (Some(x), Some(y)) => 
                                {
                                    let x_2 = (venue_home_plate_x as f64 - x) * (venue_home_plate_x as f64 - x) ;
                                    let y_2 = (venue_home_plate_y as f64 - y) * (venue_home_plate_y as f64 - y) ;
                                    
                                    let hit_data_calc_distance = (x_2 + y_2).sqrt();
                                    
                                    

                                    let temp_angle = ((venue_home_plate_y as f64 - y)/hit_data_calc_distance).acos()/PI*180f64;

                                    let hit_data_spray_angle = match (x < venue_home_plate_x as f64) {
                                        true =>  45f64 - temp_angle,
                                        false => 45f64 + temp_angle,
                                    };

                                    (Some(hit_data_spray_angle), Some(hit_data_calc_distance))
                                },
                            (_, _) => (None, None),
                        };

                        let double_play_opportunity = {
                            outs_start < 2 &&
                            (base_value_start == 1 || base_value_start == 3 || base_value_start ==5 || base_value_start == 7)
                        };

                        // Due to challenges with the Runner data, we hard-code a hack here that forces them to be a max of 7. This affects only
                        // a small subset of records and will need to be fixed at some point.
                        if base_value_end > 7 {base_value_end = 7};
                        if base_value_start > 7 {base_value_start = 7};

                        let re_288_start = data.meta_data.re_288_default.get(&(balls_start, strikes_start, base_value_start, outs_start)).unwrap_or(&0f32);
                        let re_288_end = if outs_end == 3 {&0f32} else {data.meta_data.re_288_default.get(&(balls_end % 4, strikes_end % 3, base_value_end, outs_end % 3)).unwrap_or(&0f32)};
                        let re_288_val = re_288_end - re_288_start + runs_scored as f32;

                        
                        let (pitcher_sp_rp_half, pitcher_num_plate_appearance, pitcher_num_pitch) = match half_inning {
                            HalfInning::Top => (pitcher_sp_rp.0, pitcher_num_plate_appearance_game.0, pitcher_num_pitch_game.0),
                            HalfInning::Bottom => (pitcher_sp_rp.1, pitcher_num_plate_appearance_game.1, pitcher_num_pitch_game.1)
                        };

                        let team_name_home = if half_inning == HalfInning::Top {pitcher_team_name.clone()} else {batter_team_name.clone()};
                        let team_name_away = if half_inning == HalfInning::Top {batter_team_name.clone()} else {pitcher_team_name.clone()};

                        let enough_pitch_data: bool = 
                                pitch_data.coordinates.v_y0.is_some() 
                            &&  pitch_data.coordinates.v_x0.is_some() 
                            &&  pitch_data.coordinates.v_z0.is_some() 
                            &&  pitch_data.coordinates.a_y.is_some()
                            &&  pitch_data.coordinates.a_x.is_some()
                            &&  pitch_data.coordinates.a_z.is_some()
                            &&  pitch_data.coordinates.y0.is_some()
                            &&  pitch_data.coordinates.x0.is_some()
                            &&  pitch_data.coordinates.z0.is_some()
                            &&  pitch_data.coordinates.p_x.is_some()
                            &&  pitch_data.coordinates.p_z.is_some()
                            &&  pitch_data.extension.is_some();

                        let nathan_data : crate::nathan::Nathan = if enough_pitch_data {
                        
                            crate::nathan::Data {
                                extension: pitch_data.extension.expect("Missing a variable for pitch details"),
                                vy0: pitch_data.coordinates.v_y0.expect("Missing a variable for pitch details"),
                                vx0: pitch_data.coordinates.v_x0.expect("Missing a variable for pitch details"),
                                vz0: pitch_data.coordinates.v_z0.expect("Missing a variable for pitch details"),
                                ay: pitch_data.coordinates.a_y.expect("Missing a variable for pitch details"),
                                ax: pitch_data.coordinates.a_x.expect("Missing a variable for pitch details"),
                                az: pitch_data.coordinates.a_z.expect("Missing a variable for pitch details"),
                                plate_x: pitch_data.coordinates.p_x.expect("Missing a variable for pitch details"),
                                plate_z: pitch_data.coordinates.p_z.expect("Missing a variable for pitch details"),
                                x0: pitch_data.coordinates.x0.expect("Missing a variable for pitch details"),
                                y0: pitch_data.coordinates.y0.expect("Missing a variable for pitch details"),
                                z0: pitch_data.coordinates.z0.expect("Missing a variable for pitch details"),
                            }.into()}
                            else {
                                crate::nathan::Nathan::default() 
                            }                           
                            ;

                        let plate_appearance_description = plate_app.result.plate_appearance_result_description.clone().unwrap_or_default();
                        
                        let imputed_hit_trajectory: Option<Trajectory> = if event.details.is_in_play.unwrap() {
                                 if plate_appearance_description.contains("line drive") {Some(Trajectory::LineDrive)}
                            else if plate_appearance_description.contains("lines out") {Some(Trajectory::LineDrive)}
                            else if plate_appearance_description.contains("flies out") {Some(Trajectory::FlyBall)}
                            else if plate_appearance_description.contains("fly ball") {Some(Trajectory::FlyBall)}
                            else if plate_appearance_description.contains("ground ball") {Some(Trajectory::GroundBall)}
                            else if plate_appearance_description.contains("grounds out") {Some(Trajectory::GroundBall)}
                            else if plate_appearance_description.contains("pop fly") {Some(Trajectory::PopUp)}
                            else if plate_appearance_description.contains("pops out") {Some(Trajectory::PopUp)}
                            else {Some(Trajectory::Unknown)}  
                        } else {None};
                        

                        let hit_data_trajectory = match hit_data_trajectory {
                            None => imputed_hit_trajectory,
                            _ => hit_data_trajectory,
                        };

                        let bunt_description = if event.details.is_in_play.unwrap() {
                            Some ( event.details.description.clone().unwrap_or_default().contains("bunt"))
                        } else {None};
                       
                        let bunt = match bunt {
                            Some(b) => Some (b),
                            None => bunt_description,
                        };

                        pitches.push(
                            Pitch {
                                
                                play_id: event.play_id.unwrap_or_default(),

                                half_inning,
                                num_plate_appearance,
                                num_inning,

                                start_time: event.start_time,

                                catcher_id: defense.catcher,
                                first_base_id: defense.first_base,
                                second_base_id: defense.second_base,
                                short_stop_id: defense.short_stop,
                                third_base_id: defense.third_base,
                                left_field_id: defense.left_field,
                                center_field_id: defense.center_field,
                                right_field_id: defense.right_field,
                                
                                catcher_name: get_name(defense.catcher, &player_meta),
                                first_base_name: get_name(defense.first_base, &player_meta),
                                second_base_name: get_name(defense.second_base, &player_meta),
                                short_stop_name: get_name(defense.short_stop, &player_meta),
                                third_base_name: get_name(defense.third_base, &player_meta),
                                left_field_name: get_name(defense.left_field, &player_meta),
                                center_field_name: get_name(defense.center_field, &player_meta),
                                right_field_name: get_name(defense.right_field, &player_meta),  

                                hp_umpire_id,
                                hp_umpire_name: hp_details.0.clone(), 
                                hp_umpire_dob: hp_umpire_dob.clone(),
                                hp_umpire_age: hp_details.2,
                                hp_umpire_height: hp_details.3,
                                hp_umpire_height_str: hp_details.4.clone(),

                                sport_id,
                                sport_code: sport_details.code.into(),
                                sport_name: sport_details.name.into(),
                                sport_abbr: sport_details.abbr.into(),
                                sport_affilliation: sport_details.affiliation,
                                sport_level_of_play: sport_details.level_of_play_rank,

                                team_name_home,
                                team_name_away,

                                venue_id: sched_meta.game_venue_id,
                                venue_home_plate_x,
                                venue_home_plate_y,
                                venue_name: venue_meta.venue_name.clone(),
                                venue_city: venue_meta.venue_city.clone(),
                                venue_state: venue_meta.venue_state.clone(),
                                venue_state_abbr: venue_meta.venue_state_abbr.clone(),
                                venue_time_zone: venue_meta.venue_time_zone,
                                venue_time_zone_offset: venue_meta.venue_time_zone_offset,
                                venue_capacity: venue_meta.venue_capacity,
                                venue_surface: venue_meta.venue_surface,
                                venue_roof: venue_meta.venue_roof,
                                venue_left_line: venue_meta.venue_left_line,
                                venue_left: venue_meta.venue_left,
                                venue_left_center: venue_meta.venue_left_center,
                                venue_center: venue_meta.venue_center,
                                venue_right_center: venue_meta.venue_right_center,
                                venue_right: venue_meta.venue_right,
                                venue_right_line: venue_meta.venue_right_line,
                                venue_retrosheet_id: venue_meta.venue_retrosheet_id.clone(),
                                venue_latitude: venue_meta.venue_latitude,
                                venue_longitude: venue_meta.venue_longitude,
                                
                                pitcher,
                                pitcher_throws,
                                pitcher_throws_desc,
                                batter,
                                batter_bats,
                                batter_bats_desc,
                                batter_batting_order: *players.get(&batter).unwrap_or(&None),
                                batter_pos,
                                strike_zone_top: pitch_data.strike_zone_top,
                                strike_zone_bottom: pitch_data.strike_zone_bottom,

                                batter_team_id,
                                batter_team_name: batter_team_name.clone(),
                                batter_parent_team_id,
                                batter_parent_team_name: batter_parent_team_name.clone(),
                                batter_age,
                                batter_name: batter_details.name.clone(),
                                batter_dob: batter_dob.clone(),
                                batter_mlb_debut_date: batter_mlb_debut_date.clone() ,
                                batter_birth_city: batter_details.birth_city.clone(),
                                batter_birth_state_province: batter_details.birth_state_province.clone(),
                                batter_birth_country: batter_details.birth_country.clone(),
                                batter_height_str: batter_details.height_str.clone(),
                                batter_height_in: batter_details.height_in,
                                batter_weight: batter_details.weight,
                                batter_draft_school_name: batter_details.draft_school_name.clone(),
                                batter_draft_year: batter_details.draft_year,
                                batter_draft_pick_number: batter_details.draft_pick_number,
                                batter_fangraphs_id: batter_details.fangraphs_id.clone(),
                                batter_retrosheet_id: batter_details.retrosheet_id.clone(),
                                batter_highschool_city: batter_details.highschool_city.clone(),
                                batter_highschool_prov_state: batter_details.highschool_prov_state.clone(),
                                batter_college_name: batter_details.college_name.clone(),

                                batter_stands: batter_details.bat_side_code,
                                batter_stands_desc: batter_details.bat_side_description,

                                pitcher_team_id,
                                pitcher_team_name: pitcher_team_name.clone(),
                                pitcher_parent_team_id,
                                pitcher_parent_team_name: pitcher_parent_team_name.clone(),
                                pitcher_age,
                                pitcher_name: pitcher_details.name.clone(),
                                pitcher_dob: pitcher_dob.clone(),
                                pitcher_mlb_debut_date: pitcher_mlb_debut_date.clone(),
                                pitcher_birth_city: pitcher_details.birth_city.clone(),
                                pitcher_birth_state_province: pitcher_details.birth_state_province.clone(),
                                pitcher_birth_country: pitcher_details.birth_country.clone(),
                                pitcher_height_str: pitcher_details.height_str.clone(),
                                pitcher_height_in: pitcher_details.height_in,
                                pitcher_weight: pitcher_details.weight,
                                pitcher_draft_school_name: pitcher_details.draft_school_name.clone(),
                                pitcher_draft_year: pitcher_details.draft_year,
                                pitcher_draft_pick_number: pitcher_details.draft_pick_number,
                                pitcher_fangraphs_id: pitcher_details.fangraphs_id.clone(),
                                pitcher_retrosheet_id: pitcher_details.retrosheet_id.clone(),
                                pitcher_highschool_city: pitcher_details.highschool_city.clone(),
                                pitcher_highschool_prov_state: pitcher_details.highschool_prov_state.clone(),
                                pitcher_college_name: pitcher_details.college_name.clone(),
                                
                                pitcher_sp_rp: pitcher_sp_rp_half,

                                pitcher_num_plate_appearance,
                                pitcher_num_pitch,

                                pitch_num_plate_appearance,
                                pitch_num_inning,    
                                pitch_num_game,
                                preceded_by_pickoff,
                                balls_start,
                                balls_end,
                                strikes_start,
                                strikes_end,
                                outs_start,
                                outs_end,
                                base_value_start,
                                base_value_end,
                                runs_scored,
                                re_288_batter_responsible,
                                re_288_start: *re_288_start,
                                re_288_end: *re_288_end,
                                re_288_val,
                                swing,
                                foul,
                                swing_and_miss,
                                double_play_opportunity,

                                description: event.details.description.unwrap_or_default(),
                                plate_appearance_description,
                                plate_appearance_result: plate_app.result.plate_appearance_result,
                                
                                in_play: event.details.is_in_play.unwrap().into(),
                                
                                pitch_speed_start: pitch_data.start_speed,
                                pitch_speed_end: pitch_data.end_speed,
                                pitch_break_vertical_induced,
                                pitch_break_horizontal,
                                pitch_spin_rate,
                                pitch_spin_direction,
                                pitch_plate_time: pitch_data.plate_time,
                                pitch_extension: pitch_data.extension,
                                pitch_pixels_x: pitch_data.coordinates.x,
                                pitch_pixels_y: pitch_data.coordinates.y,
                                pitch_a_x: pitch_data.coordinates.a_x,
                                pitch_a_y: pitch_data.coordinates.a_y,
                                pitch_a_z: pitch_data.coordinates.a_z,
                                pitch_pfx_x: pitch_data.coordinates.pfx_x,
                                pitch_pfx_z: pitch_data.coordinates.pfx_z,
                                pitch_p_x: pitch_data.coordinates.p_x,
                                pitch_p_z: pitch_data.coordinates.p_z,
                                pitch_v_x0: pitch_data.coordinates.v_x0,
                                pitch_v_y0: pitch_data.coordinates.v_y0,
                                pitch_v_z0: pitch_data.coordinates.v_z0,
                                pitch_x0: pitch_data.coordinates.x0,
                                pitch_y0: pitch_data.coordinates.y0,
                                pitch_z0: pitch_data.coordinates.z0,
                                pitch_type_code,
                                pitch_type_desc,
                                
                                in_play_1b,
                                in_play_2b,
                                in_play_3b,
                                in_play_hr,
                                in_play_result,
                                bunt,
                                strikeout,
                                walk,
                                
                                hit_data_coord_x, 
                                hit_data_coord_y, 
                                hit_data_trajectory, 
                                hit_data_contact_quality, 
                                hit_data_launch_angle, 
                                hit_data_exit_velocity, 
                                hit_data_total_distance, 
                                hit_data_spray_angle,
                                hit_data_calc_distance,

                                fielded_by_id,
                                fielded_by_name,
                                fielded_by_pos,


                                game_pk: sched_meta.game_pk,
                                game_type: sched_meta.game_type,
                                game_type_desc: sched_meta.game_type_desc,
                                game_date: sched_meta.game_date.to_string(),
                                game_year: sched_meta.game_date.year,
                                game_month: sched_meta.game_date.month,
                                game_status: sched_meta.game_status,
                                game_weather_condition: box_meta.game_weather_condition,
                                game_weather_temp_c: box_meta.game_weather_temp_c,
                                game_weather_temp_f: box_meta.game_weather_temp_f,
                                game_wind_direction: box_meta.game_wind_direction,
                                game_wind_speed_mph: box_meta.game_wind_speed_mph,
                                game_attendance: box_meta.attendance,
                                game_first_pitch: box_meta.first_pitch,

                                league_name: box_meta.home_league_name.clone().unwrap_or_default(),

                                //Nathan Variables

                                xr: nathan_data.xr,
                                yr: nathan_data.yr,
                                zr: nathan_data.zr,
                                vxr: nathan_data.vxr,
                                vyr: nathan_data.vyr,
                                vzr: nathan_data.vzr,
                                tf: nathan_data.tf,
                                tr: nathan_data.tr,
                                vxbar: nathan_data.vxbar,
                                vybar: nathan_data.vybar,
                                vzbar: nathan_data.vzbar,
                                vbar: nathan_data.vbar,
                                vxhat: nathan_data.vxhat,
                                vyhat: nathan_data.vyhat,
                                vzhat: nathan_data.vzhat,
                                ad: nathan_data.ad,
                                atx: nathan_data.atx,
                                aty: nathan_data.aty,
                                atz: nathan_data.atz,
                                atx_hat: nathan_data.atx_hat,
                                aty_hat: nathan_data.aty_hat,
                                atz_hat: nathan_data.atz_hat,
                                at: nathan_data.at,
                                phi_t: nathan_data.phi_t,
                                ivb: nathan_data.ivb,
                                hb: nathan_data.hb,
                                cd: nathan_data.cd,
                                

                            }
                        );
                        // The pitches.push() function ends here
                        // If we've pushed a pitch, we can reset the preceded_by_pickoff flag
                        preceded_by_pickoff = false;

                        // Set the new start_state for the next pitch
                        balls_start = balls_end;
                        strikes_start = strikes_end;
                        outs_start = outs_end;
                        base_value_start = base_value_end;

                        // Set the half_inning state so we can check if it has changed
                        previous_half_inning = half_inning;
                    }
                }
            }
        }
        pitches
    }
}