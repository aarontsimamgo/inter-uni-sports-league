#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

// Memory Management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Import the models module
mod models;

use models::*;

// Define an Error enum for handling errors
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]

pub enum Error {
    Success { msg: String },
    Error { msg: String },
    NotFound { msg: String },
    InvalidPayload { msg: String },
    Unauthorized { msg: String },
    PaymentFailed { msg: String },
    PaymentCompleted { msg: String },
}

// Thread-local storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static TEAMS_STORAGE: RefCell<StableBTreeMap<u64, Team, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static MATCHES_STORAGE: RefCell<StableBTreeMap<u64, Match, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static REFEREE_STORAGE: RefCell<StableBTreeMap<u64, Referee, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static TOURNAMENTS_STORAGE: RefCell<StableBTreeMap<u64, Tournament, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static LEAGUES_STORAGE: RefCell<StableBTreeMap<u64, League, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );

}

// Implement Storable for User
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Team
impl Storable for Team {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Team {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Match
impl Storable for Match {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Match {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Referee
impl Storable for Referee {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Referee {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Tournament
impl Storable for Tournament {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Tournament {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for League
impl Storable for League {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for League {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Helper Functions

// Generates a unique identifier for objects
fn generate_uuid() -> u64 {
    let id = ID_COUNTER.with(|counter| {
        let current_id = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_id + 1);
        current_id
    });

    id
}

// Validate email format
fn validate_email_format(email: &str) -> Result<(), String> {
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if !email_regex.is_match(email) {
        Err("Invalid email format".to_string())
    } else {
        Ok(())
    }
}

// Validate email uniqueness
fn validate_email_uniqueness(email: &str) -> Result<(), String> {
    let email_exists =
        USERS_STORAGE.with(|storage| storage.borrow().iter().any(|(_, user)| user.email == email));

    if email_exists {
        Err("User with this email already exists".to_string())
    } else {
        Ok(())
    }
}

/*
Register a new User
This function takes a RegisterUserPayload as input and returns a Result containing either a User or a String error message.
It generates a unique ID for the new user, creates a User object, and stores it in the USERS_STORAGE.
Performs validation checks on input data and handles potential registration errors.
*/
#[ic_cdk::update]
fn register_user(payload: RegisterUserPayload) -> Result<User, String> {
    // Validate the user payload to ensure all required fields are present
    if payload.name.is_empty() || payload.email.is_empty() || payload.address.is_empty() {
        return Err("Name, email, and address are required fields".to_string());
    }

    // Validate email format
    validate_email_format(&payload.email)?;

    // Check if user with this email already exists
    validate_email_uniqueness(&payload.email)?;

    // Generate unique ID
    let id = generate_uuid();

    // Create user object
    let user = User {
        id,
        owner: caller(),
        name: payload.name,
        email: payload.email,
        address: payload.address,
        role: payload.role,
    };

    // Store user in storage
    USERS_STORAGE.with(|users| {
        users.borrow_mut().insert(id, user.clone());
        Ok(user)
    })
}

//  Function to update a user's details
#[ic_cdk::update]
fn update_user(payload: UpdateUserPayload) -> Result<User, String> {
    // Ensure the user exists
    if !USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.id)) {
        return Err("User not found".to_string());
    }

    // Validate email format
    validate_email_format(&payload.email)?;

    // Check if user with this email already exists
    let email_exists = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.email == payload.email && user.id != payload.id)
    });

    if email_exists {
        return Err("User with this email already exists".to_string());
    }

    let user = User {
        id: payload.id,
        owner: caller(),
        name: payload.name,
        email: payload.email,
        address: payload.address,
        role: payload.role,
    };

    USERS_STORAGE.with(
        |storage| match storage.borrow_mut().insert(payload.id, user.clone()) {
            Some(_) => Ok(user),
            None => Err("User not found".to_string()),
        },
    )
}

// Fetch a user by ID
#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, String> {
    USERS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(user) => Ok(user.clone()),
        None => Err(format!("User with ID {} not found", id)),
    })
}

// Function to get user by owner
#[ic_cdk::query]
fn get_user_by_owner() -> Result<User, String> {
    USERS_STORAGE.with(|users| {
        let users = users.borrow();
        let user = users
            .iter()
            .find(|(_, user)| user.owner.to_text() == caller().to_text());
        match user {
            Some((_, client)) => Ok(client.clone()),
            None => Err(format!("User not found for caller: {}", caller().to_text())),
        }
    })
}

// Function to retrieve all users and throw an error if no users are found
#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    USERS_STORAGE.with(|storage| {
        let users: Vec<User> = storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();
        if users.is_empty() {
            Err("No users found".to_string())
        } else {
            Ok(users)
        }
    })
}

// FUnction to get user by name, putting into consideration the case sensitivity
#[ic_cdk::query]
fn get_user_by_name(name: String) -> Result<User, String> {
    USERS_STORAGE.with(|storage| {
        let users = storage.borrow();
        let user = users
            .iter()
            .find(|(_, user)| user.name.to_lowercase() == name.to_lowercase());
        match user {
            Some((_, client)) => Ok(client.clone()),
            None => Err(format!("User with name {} not found", name)),
        }
    })
}

/**
 * Function to create a new team
 * This function takes a CreateTeamPayload as input and returns a Result containing either a Team or a String error message.
 * It generates a unique ID for the new team, creates a Team object, and stores it in the TEAMS_STORAGE.
 * Performs validation checks on input data and handles potential registration errors.
 */

#[ic_cdk::update]
fn create_team(payload: CreateTeamPayload) -> Result<Team, String> {
    // Validate the team payload to ensure all required fields are present
    if payload.name.is_empty() {
        return Err("Name is a required field".to_string());
    }

    // Generate unique ID
    let id = generate_uuid();

    // Create team object
    let team = Team {
        id,
        name: payload.name,
        sport_type: payload.sport_type,
        members: Vec::new(),
        coaches: Vec::new(),
    };

    // Store team in storage
    TEAMS_STORAGE.with(|teams| {
        teams.borrow_mut().insert(id, team.clone());
        Ok(team)
    })
}

// Fetch a team by ID
#[ic_cdk::query]
fn get_team(id: u64) -> Result<Team, String> {
    TEAMS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(team) => Ok(team.clone()),
        None => Err(format!("Team with ID {} not found", id)),
    })
}

// Function to get all teams
#[ic_cdk::query]
fn get_all_teams() -> Result<Vec<Team>, String> {
    TEAMS_STORAGE.with(|storage| {
        let teams: Vec<Team> = storage
            .borrow()
            .iter()
            .map(|(_, team)| team.clone())
            .collect();
        if teams.is_empty() {
            Err("No teams found".to_string())
        } else {
            Ok(teams)
        }
    })
}

//  Function to add member to a team
#[ic_cdk::update]
fn add_member_to_team(payload: AddMemberPayload) -> Result<Team, String> {
    let team_id = payload.team_id;
    let member_id = payload.member_id;

    // Check if team exists
    let team = TEAMS_STORAGE.with(|storage| match storage.borrow().get(&team_id) {
        Some(team) => Some(team.clone()),
        None => None,
    });

    if team.is_none() {
        return Err("Team not found".to_string());
    }

    // Check if member exists
    let member = USERS_STORAGE.with(|storage| match storage.borrow().get(&member_id) {
        Some(user) => Some(user.clone()),
        None => None,
    });

    if member.is_none() {
        return Err(format!("Member with ID {} not found", member_id));
    }

    // Check if member is already part of a team
    let member_already_in_team = TEAMS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, team)| team.members.contains(&member_id))
    });

    if member_already_in_team {
        return Err("Member is already part of a team".to_string());
    }

    // Ensure the member is a player
    if member.as_ref().unwrap().role != UserRole::Player {
        return Err("Member must be a player".to_string());
    }

    // Add member to team
    TEAMS_STORAGE.with(|teams| {
        let mut teams = teams.borrow_mut();
        match teams.get(&team_id) {
            Some(existing_team) => {
                let mut team = existing_team.clone();
                if team.members.contains(&member_id) {
                    return Err("Member is already in this team".to_string());
                }
                team.members.push(member_id);
                teams.insert(team_id, team.clone());
                Ok(team)
            }
            None => Err("Team not found".to_string()),
        }
    })
}
// Schedule a match
#[ic_cdk::update]
pub fn schedule_match(payload: ScheduleMatchPayload) -> Result<Match, String> {
    // Validate the match payload to ensure all required fields are present
    if payload.scheduled_date.is_empty() {
        return Err("Ensure scheduled date is provided in the format YYYY-MM-DD".to_string());
    }

    let sport_type = payload.sport_type.clone();
    let home_team_id = payload.home_team_id;
    let away_team_id = payload.away_team_id;
    let scheduled_date = payload.scheduled_date;

    // Check if home team exists
    let home_team = TEAMS_STORAGE.with(|storage| match storage.borrow().get(&home_team_id) {
        Some(team) => Some(team.clone()),
        None => None,
    });

    if home_team.is_none() {
        return Err("Home team not found".to_string());
    }

    // Check if away team exists
    let away_team = TEAMS_STORAGE.with(|storage| match storage.borrow().get(&away_team_id) {
        Some(team) => Some(team.clone()),
        None => None,
    });

    if away_team.is_none() {
        return Err("Away team not found".to_string());
    }

    // Ensure the teams are not the same
    if home_team_id == away_team_id {
        return Err("Home team and away team cannot be the same".to_string());
    }

    // Generate unique ID for the match
    let id = generate_uuid();

    // Create match object
    let match_obj = Match {
        id,
        sport_type,
        home_team: home_team.unwrap(),
        away_team: away_team.unwrap(),
        scheduled_date,
        result: None,
    };

    // Store match in storage
    MATCHES_STORAGE.with(|matches| {
        matches.borrow_mut().insert(id, match_obj.clone());
        Ok(match_obj)
    })
}

// Submit match result
#[ic_cdk::update]
pub fn submit_match_result(payload: MatchResultPayload) -> Result<Match, String> {
    let match_id = payload.match_id;
    let result = payload.result;

    // Check if match exists
    let existing_match = MATCHES_STORAGE.with(|storage| match storage.borrow().get(&match_id) {
        Some(match_obj) => Some(match_obj.clone()),
        None => None,
    });

    if existing_match.is_none() {
        return Err(format!("Match with ID {} not found", match_id));
    }

    // Check if result is already submitted
    if existing_match.as_ref().unwrap().result.is_some() {
        return Err("Match result has already been submitted".to_string());
    }

    // Submit match result
    MATCHES_STORAGE.with(|matches| {
        let mut matches = matches.borrow_mut();
        match matches.get(&match_id) {
            Some(existing_match_obj) => {
                let mut updated_match = existing_match_obj.clone();
                updated_match.result = Some(result);
                matches.insert(match_id.clone(), updated_match.clone());
                Ok(updated_match)
            }
            None => Err("Match not found".to_string()),
        }
    })
}

// Get leaderboard for a specific sport type

/**
 * Function to assign a coach to a team
 * This function takes an AssignCoachPayload as input and returns a Result containing either a Team or a String error message.
 * It fetches the team and coach by their IDs, validates their existence, and assigns the coach to the team.
 * Handles potential errors such as missing team or coach, and duplicate coach assignment.
 */
#[ic_cdk::update]
fn assign_coach(payload: AssignCoachPayload) -> Result<Team, String> {
    let team_id = payload.team_id;
    let coach_id = payload.coach_id;

    // Check if team exists
    let team = TEAMS_STORAGE.with(|storage| match storage.borrow().get(&team_id) {
        Some(team) => Some(team.clone()),
        None => None,
    });

    if team.is_none() {
        return Err("Team not found".to_string());
    }

    // Check if coach exists
    let coach = USERS_STORAGE.with(|storage| match storage.borrow().get(&coach_id) {
        Some(user) => Some(user.clone()),
        None => None,
    });

    if coach.is_none() {
        return Err(format!("Coach with ID {} not found", coach_id));
    }

    // Check if coach is already part of a team
    let coach_already_in_team = TEAMS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, team)| team.coaches.contains(&coach_id))
    });

    if coach_already_in_team {
        return Err("Coach is already part of a team".to_string());
    }

    // Ensure the coach is a coach
    if coach.as_ref().unwrap().role != UserRole::Coach {
        return Err("Coach must have Coach role".to_string());
    }

    // Assign coach to team
    TEAMS_STORAGE.with(|teams| {
        let mut teams = teams.borrow_mut();
        match teams.get(&team_id) {
            Some(existing_team) => {
                let mut team = existing_team.clone();
                if team.coaches.contains(&coach_id) {
                    return Err("Coach is already in this team".to_string());
                }
                team.coaches.push(coach_id);
                teams.insert(team_id, team.clone());
                Ok(team)
            }
            None => Err("Team not found".to_string()),
        }
    })
}
// Get all matches
#[ic_cdk::query]
fn get_all_matches() -> Result<Vec<Match>, String> {
    MATCHES_STORAGE.with(|storage| {
        let matches: Vec<Match> = storage
            .borrow()
            .iter()
            .map(|(_, matches)| matches.clone())
            .collect();
        if matches.is_empty() {
            Err("No matches found".to_string())
        } else {
            Ok(matches)
        }
    })
}

// Get match by ID
#[ic_cdk::query]
fn get_match(id: u64) -> Result<Match, String> {
    MATCHES_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(match_obj) => Ok(match_obj.clone()),
        None => Err(format!("Match with ID {} not found", id)),
    })
}

// Get all matches for a specific team
#[ic_cdk::query]
fn get_matches_by_team(team_id: u64) -> Result<Vec<Match>, String> {
    MATCHES_STORAGE.with(|storage| {
        let matches: Vec<Match> = storage
            .borrow()
            .iter()
            .filter(|(_, match_obj)| {
                match_obj.home_team.id == team_id || match_obj.away_team.id == team_id
            })
            .map(|(_, match_obj)| match_obj.clone())
            .collect();
        if matches.is_empty() {
            Err("No matches found for this team".to_string())
        } else {
            Ok(matches)
        }
    })
}

// Get all matches for a specific sport type
#[ic_cdk::query]
fn get_matches_by_sport_type(sport_type: SportType) -> Result<Vec<Match>, String> {
    MATCHES_STORAGE.with(|storage| {
        let matches: Vec<Match> = storage
            .borrow()
            .iter()
            .filter(|(_, match_obj)| match_obj.sport_type == sport_type)
            .map(|(_, match_obj)| match_obj.clone())
            .collect();
        if matches.is_empty() {
            Err("No matches found for this sport type".to_string())
        } else {
            Ok(matches)
        }
    })
}

// Get all matches for a specific date
#[ic_cdk::query]
fn get_matches_by_date(date: String) -> Result<Vec<Match>, String> {
    MATCHES_STORAGE.with(|storage| {
        let matches: Vec<Match> = storage
            .borrow()
            .iter()
            .filter(|(_, match_obj)| match_obj.scheduled_date == date)
            .map(|(_, match_obj)| match_obj.clone())
            .collect();
        if matches.is_empty() {
            Err("No matches found for this date".to_string())
        } else {
            Ok(matches)
        }
    })
}

// Candid generator for exporting the Candid interface
ic_cdk::export_candid!();
