//! Crate for testing whether the deserialize codegen is capable of handling
//! real Minecraft-generated files.

#![feature(test)]
extern crate test;

#[macro_use] extern crate serde_derive;
extern crate serde;

extern crate nbt;
extern crate nbt_serde;

use std::fs::File;

use nbt_serde::decode::from_gzip;

#[test]
fn deserialize_simple_player() {
    let mut file = File::open("../tests/simple_player.dat").unwrap();
    let _: data::PlayerData = from_gzip(&mut file).unwrap();
}

#[test]
fn deserialize_complex_player() {
    let mut file = File::open("../tests/complex_player.dat").unwrap();
    let _: data::PlayerData = from_gzip(&mut file).unwrap();
}

#[test]
fn deserialize_level() {
    let mut file = File::open("../tests/level.dat").unwrap();
    let _: data::Level = from_gzip(&mut file).unwrap();
}

mod bench {
    use std::io;
    use std::io::Read;

    use test::Bencher;

    use nbt_serde::encode::to_writer;

    use super::*;

    #[bench]
    fn deserialize_simple_player_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/simple_player.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            let _: data::PlayerData = from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn deserialize_simple_player_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/simple_player.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            nbt::Blob::from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn serialize_simple_player_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/simple_player.dat").unwrap();
        let nbt: data::PlayerData = from_gzip(&mut file).unwrap();
        b.iter(|| {
            to_writer(&mut io::sink(), &nbt, None)
        });
    }

    #[bench]
    fn serialize_simple_player_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/simple_player.dat").unwrap();
        let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
        b.iter(|| {
            nbt.write(&mut io::sink())
        });
    }

    #[bench]
    fn deserialize_complex_player_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/complex_player.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            let _: data::PlayerData = from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn deserialize_complex_player_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/complex_player.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            nbt::Blob::from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn serialize_complex_player_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/complex_player.dat").unwrap();
        let nbt: data::PlayerData = from_gzip(&mut file).unwrap();
        b.iter(|| {
            to_writer(&mut io::sink(), &nbt, None)
        });
    }

    #[bench]
    fn serialize_complex_player_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/complex_player.dat").unwrap();
        let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
        b.iter(|| {
            nbt.write(&mut io::sink())
        });
    }

    #[bench]
    fn deserialize_level_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/level.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            let _: data::Level = from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn deserialize_level_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/level.dat").unwrap();
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).unwrap();
        b.iter(|| {
            let mut src = std::io::Cursor::new(&contents[..]);
            nbt::Blob::from_gzip(&mut src).unwrap();
        });
    }

    #[bench]
    fn serialize_level_as_struct(b: &mut Bencher) {
        let mut file = File::open("../tests/level.dat").unwrap();
        let nbt: data::Level = from_gzip(&mut file).unwrap();
        b.iter(|| {
            to_writer(&mut io::sink(), &nbt, None)
        });
    }

    #[bench]
    fn serialize_level_as_blob(b: &mut Bencher) {
        let mut file = File::open("../tests/level.dat").unwrap();
        let nbt = nbt::Blob::from_gzip(&mut file).unwrap();
        b.iter(|| {
            nbt.write(&mut io::sink())
        });
    }
}

mod data {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Level {
        #[serde(rename = "Data")]
        pub data: LevelData
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LevelData {
        #[serde(rename = "RandomSeed")] seed: i64,
        #[serde(rename = "DayTime")] daytime: i64,
        #[serde(rename = "Player")] player: PlayerData,
        initialized: bool,
        version: i32,
        #[serde(rename = "allowCommands")] allow_commands: bool,
        #[serde(rename = "LastPlayed")] last_played: i64,
        #[serde(rename = "SpawnZ")] spawn_z: i32,
        #[serde(rename = "SpawnX")] spawn_x: i32,
        #[serde(rename = "SpawnY")] spawn_y: i32,
        #[serde(rename = "LevelName")] name: String,
        #[serde(rename = "MapFeatures")] map_features: bool,

        #[serde(rename = "GameType")] game_type: i32,
        #[serde(rename = "Difficulty")] difficulty: i8,
        #[serde(rename = "DifficultyLocked")] difficulty_locked: bool,

        #[serde(rename = "generatorName")] generator_name: String,
        #[serde(rename = "generatorOptions")] generator_options: String,
        #[serde(rename = "generatorVersion")] generator_version: i32,

        #[serde(rename = "Time")] time: i64,
        #[serde(rename = "clearWeatherTime")] clear_weather_time: i32,
        #[serde(rename = "thunderTime")] thunder_time: i32,
        #[serde(rename = "rainTime")] rain_time: i32,

        thundering: bool,
        raining: bool,
        hardcore: bool,

        #[serde(rename = "GameRules")] game_rules: GameRules,
        #[serde(rename = "SizeOnDisk")] size_on_disk: i64,

        #[serde(rename = "BorderCenterX")] border_center_x: f64,
        #[serde(rename = "BorderCenterY")] border_center_y: Option<f64>,
        #[serde(rename = "BorderCenterZ")] border_center_z: f64,
        #[serde(rename = "BorderWarningBlocks")] border_warning_blocks: f64,
        #[serde(rename = "BorderWarningTime")] border_warning_time: f64,
        #[serde(rename = "BorderSafeZone")] border_safe_zone: f64,
        #[serde(rename = "BorderSize")] border_size: f64,
        #[serde(rename = "BorderSizeLerpTarget")] border_size_lerp_target: f64,
        #[serde(rename = "BorderSizeLerpTime")] border_size_lerp_time: i64,
        #[serde(rename = "BorderDamagePerBlock")] border_damage_per_block: f64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlayerData {
        #[serde(rename = "PersistentId")] persistant_id: Option<i32>,
        #[serde(rename = "playerGameType")] game_type: i32,
        abilities: PlayerAbilityData,
        #[serde(rename = "Score")] score: Option<i32>,

        #[serde(rename = "Dimension")] dimension: i32,
        #[serde(rename = "OnGround")] on_ground: bool,
        #[serde(rename = "FallDistance")] fall_distance: f32,
        #[serde(rename = "Motion")] motion: Vec<f64>, // [f64; 3]
        #[serde(rename = "Pos")] position: Vec<f64>, // [f64; 3]
        #[serde(rename = "Rotation")] rotation: Vec<f32>, // [f32; 2]

        #[serde(rename = "SpawnX")] spawn_x: i32,
        #[serde(rename = "SpawnY")] spawn_y: i32,
        #[serde(rename = "SpawnZ")] spawn_z: i32,
        #[serde(rename = "SpawnForced")] spawn_forced: Option<bool>,

        #[serde(rename = "PortalCooldown")] portal_cooldown: Option<i32>,
        #[serde(rename = "Invulnerable")] invulnerable: Option<bool>,

        #[serde(rename = "AttackTime")] attack_time: Option<i16>,
        #[serde(rename = "HurtTime")] hurt_time: i16,
        #[serde(rename = "HurtByTimestamp")] hurt_by: Option<i32>,
        #[serde(rename = "DeathTime")] death_time: i16,
        #[serde(rename = "Sleeping")] sleeping: bool,
        #[serde(rename = "SleepTimer")] sleep_timer: i16,

        #[serde(rename = "Health")] health: i16,
        #[serde(rename = "HealF")] heal: Option<f32>,
        #[serde(rename = "foodLevel")] food_level: i32,
        #[serde(rename = "foodTickTimer")] food_tick_timer: i32,
        #[serde(rename = "foodSaturationLevel")] food_saturation_level: f32,
        #[serde(rename = "foodExhaustionLevel")] food_exhaustion_level: f32,

        #[serde(rename = "Fire")] fire: i16,
        #[serde(rename = "Air")] air: i16,

        #[serde(rename = "XpP")] xp_p: f32,
        #[serde(rename = "XpLevel")] xp_level: i32,
        #[serde(rename = "XpTotal")] xp_total: i32,
        #[serde(rename = "XpSeed")] xp_seed: Option<i32>,

        #[serde(rename = "Inventory")] inventory: Vec<InventoryEntry>,
        #[serde(rename = "EnderItems")] ender_items: Vec<i8>,

        #[serde(rename = "SelectedItemSlot")] selected_item_slot: Option<i32>,
        #[serde(rename = "SelectedItem")] selected_item: Option<InventoryEntry>,
        #[serde(rename = "UUIDLeast")] uuid_least: Option<i64>,
        #[serde(rename = "UUIDMost")] uuid_most: Option<i64>,
        #[serde(rename = "AbsorptionAmount")] absorbtion_amount: Option<f32>,
        #[serde(rename = "Attributes")] attributes: Option<Vec<AttributeEntry>>,
        #[serde(rename = "ActiveEffects")] active_effects: Option<Vec<ActiveEffect>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlayerAbilityData {
        invulnerable: bool,
        instabuild: bool,
        flying: bool,
        #[serde(rename = "flySpeed")] fly_speed: f32,
        #[serde(rename = "walkSpeed")] walk_speed: f32,
        #[serde(rename = "mayBuild")] may_build: bool,
        #[serde(rename = "mayfly")] may_fly: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryEntry {
        id: String,
        #[serde(rename = "Slot")] slot: Option<i8>,
        #[serde(rename = "Count")] count: i8,
        #[serde(rename = "Damage")] damage: i16,
        #[serde(rename = "tag")] info: Option<InventoryEntryInfo>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryEntryInfo {
        display: Option<InventoryEntryDisplay>,
        #[serde(rename = "RepairCost")] repair_cost: Option<i32>,
        #[serde(rename = "ench")] enchantments: Vec<Enchantment>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct InventoryEntryDisplay {
        #[serde(rename = "Name")] name: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Enchantment {
        id: i16,
        #[serde(rename = "lvl")] level: i16,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct EnderItemsEntry {
        id: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeEntry {
        #[serde(rename = "Name")] name: String,
        #[serde(rename = "Base")] base: f64,
        #[serde(rename = "Modifiers")] modifiers: Option<Vec<AttributeModifier>>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AttributeModifier {
        #[serde(rename = "Name")] name: String,
        #[serde(rename = "Amount")] amount: f64,
        #[serde(rename = "Operation")] operation: i32,
        #[serde(rename = "UUIDLeast")] uuid_least: i64,
        #[serde(rename = "UUIDMost")] uuid_most: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ActiveEffect {
        #[serde(rename = "Id")] id: i8,
        #[serde(rename = "Duration")] base: i32,
        #[serde(rename = "Ambient")] ambient: bool,
        #[serde(rename = "Amplifier")] amplifier: bool,
        #[serde(rename = "ShowParticles")] show_particles: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GameRules {
        #[serde(rename = "doMobLoot")] mob_loot: String,
        #[serde(rename = "doTileDrops")] tile_drops: String,
        #[serde(rename = "doFireTick")] fire_tick: String,
        #[serde(rename = "mobGriefing")] mob_griefing: String,
        #[serde(rename = "commandBlockOutput")] command_block_output: String,
        #[serde(rename = "doMobSpawning")] mob_spawning: String,
        #[serde(rename = "keepInventory")] keep_inventory: String,
        #[serde(rename = "showDeathMessages")] show_death_messages: String,
        #[serde(rename = "doEntityDrops")] entity_drops: String,
        #[serde(rename = "naturalRegeneration")] natural_regeneration: String,
        #[serde(rename = "logAdminCommands")] log_admin_commands: String,
        #[serde(rename = "doDaylightCycle")] daylight_cycle: String,
        #[serde(rename = "sendCommandFeedback")] send_command_feedback: String,
        #[serde(rename = "randomTickSpeed")] random_tick_speed: String,
        #[serde(rename = "reducedDebugInfo")] reduced_debug_info: String,
    }
}
