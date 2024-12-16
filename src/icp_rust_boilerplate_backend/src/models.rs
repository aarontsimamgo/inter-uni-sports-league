use candid::{CandidType, Principal};

// User Role Types Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum UserRole {
    #[default]
    Player,
    Coach,
    Administrator,
    LeagueOfficial,
}

// Structure representing a User
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) owner: Principal,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Sport Type Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum SportType {
    #[default]
    Football,
    Basketball,
    Volleyball,
    Tennis,
    Cricket,
    Rugby,
    Hockey,
    Golf,
    Badminton,
    TableTennis,
}

// Struct representing a Team
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Team {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) coaches: Vec<u64>,
    pub(crate) sport_type: SportType,
    pub(crate) members: Vec<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct MatchResult {
    pub(crate) winner_team_id: u64,
    pub(crate) score_team_a: u32,
    pub(crate) score_team_b: u32,
    pub(crate) notes: String,
}

// Struct representing a Match
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Match {
    pub(crate) id: u64,
    pub(crate) home_team: Team,
    pub(crate) away_team: Team,
    pub(crate) sport_type: SportType,
    pub(crate) scheduled_date: String,
    pub(crate) result: Option<MatchResult>,
}

// Struct representing a Referee
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Referee {
    pub(crate) id: u64,
    pub(crate) owner: Principal,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) matches_officiated: Vec<String>,
    pub(crate) performance_rating: f32,
    pub(crate) total_rating: f32,
    pub(crate) total_matches: u32,
}

// Tournament Structure Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum TournamentStructure {
    #[default]
    RoundRobin,
    Knockout,
}

// Struct representing a Tournament
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tournament {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) structure: TournamentStructure,
    pub(crate) teams: Vec<String>,
    pub(crate) sport_type: SportType,
}

// Struct representing a League
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct League {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) tournaments: Vec<Tournament>,
    pub(crate) sport_type: SportType,
    pub(crate) created_by: Principal,
}

// Payloads

// Register User Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RegisterUserPayload {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Payload for updating users
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateUserPayload {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Create Team Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateTeamPayload {
    pub(crate) name: String,
    pub(crate) sport_type: SportType,
}

// Payload for adding a member to a team
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AddMemberPayload {
    pub(crate) team_id: u64,
    pub(crate) member_id: u64,
}

// Payload for Assigning a Coach to a Team
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AssignCoachPayload {
    pub(crate) team_id: u64,
    pub(crate) coach_id: u64,
}

// Schedule Match Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ScheduleMatchPayload {
    pub(crate) home_team_id: u64,
    pub(crate) away_team_id: u64,
    pub(crate) sport_type: SportType,
    pub(crate) scheduled_date: String,
}

// Match Result Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MatchResultPayload {
    pub(crate) match_id: u64,
    pub(crate) result: MatchResult,
}

// Tournament Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateTournamentPayload {
    pub(crate) name: String,
    pub(crate) structure: TournamentStructure,
    pub(crate) team_ids: Vec<String>,
    pub(crate) sport_type: SportType,
}

// League Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateLeaguePayload {
    pub(crate) name: String,
    pub(crate) sport_type: SportType,
}
