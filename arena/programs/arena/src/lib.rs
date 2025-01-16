use anchor_lang::prelude::*;
use mpl_core::Asset;
use std::str;

declare_id!("8EacGtcQv8R9jFvYFRPZEYec2CqqqHSJRCZNU6GJooXH");

#[program]
pub mod arena {
    use super::*;

    pub fn fight(ctx: Context<Fight>) -> Result<i64> {
        let upgrade_chestplate = process_account("Chestplate", &ctx.accounts.chestplate)?;
        let upgrade_gloves = process_account("Gloves", &ctx.accounts.gloves)?;
        let upgrade_boots = process_account("Boots", &ctx.accounts.boots)?;
        let upgrade_sword = process_account("Sword", &ctx.accounts.sword)?;
        let upgrade_helmet = process_account("Helmet", &ctx.accounts.helmet)?;
        let upgrade_neck = process_account("Neck", &ctx.accounts.neck)?;
        let upgrade_ring = process_account("Ring", &ctx.accounts.ring)?;

        let mut fighter = Fighter::new(
            upgrade_sword,
            upgrade_helmet,
            upgrade_chestplate,
            upgrade_gloves,
            upgrade_boots,
            upgrade_neck,
            upgrade_ring,
        );

        let mut bot = Fighter::new(5, 5, 5, 5, 5, 5, 5);

        msg!("Fighter Stats: {:?}", fighter);
        msg!("Bot Stats: {:?}", bot);

        let clock = Clock::get()?;
        let timestamp = clock.unix_timestamp;
        msg!("Timestamp: {}", timestamp);

        let mut random_generator = RandomGenerator::new(timestamp);

        let winner = simulate_combat(&mut fighter, &mut bot, &mut random_generator);

        msg!("Winner: {}", winner);

        Ok(timestamp)
    }
}

#[derive(Debug)]
pub struct Fighter {
    atk: i32,
    hp: i32,
    armor: i32,
    acc: i32,
    evasion: i32,
    crit: i32,
}

impl Fighter {
    pub fn new(
        upg_atk: i32,
        upg_hp: i32,
        upg_armor: i32,
        upg_acc: i32,
        upg_evasion: i32,
        upg_crit1: i32,
        upg_crit2: i32,
    ) -> Self {
        Self {
            atk: 5 + (upg_atk as f32 * 0.1 * 5.0).ceil() as i32,
            hp: 100 + (upg_hp as f32 * 0.1 * 100.0).ceil() as i32,
            armor: 2 + (upg_armor as f32 * 0.1 * 2.0).ceil() as i32,
            acc: (upg_acc as f32 * 0.1 * 10.0).ceil() as i32,
            evasion: (upg_evasion as f32 * 0.1 * 10.0).ceil() as i32,
            crit: (upg_crit1 as f32 * 0.1 * 5.0).ceil() as i32
                + (upg_crit2 as f32 * 0.1 * 5.0).ceil() as i32,
        }
    }
}

fn simulate_combat(
    fighter: &mut Fighter,
    bot: &mut Fighter,
    random_generator: &mut RandomGenerator,
) -> String {
    loop {
        if simulate_attack("Fighter", fighter, "Bot", bot, random_generator) {
            return "Fighter".to_string();
        }
        if simulate_attack("Bot", bot, "Fighter", fighter, random_generator) {
            return "Bot".to_string();
        }
        msg!("Another round of attacks happens!");
    }
}

fn simulate_attack(
    attacker_name: &str,
    attacker: &mut Fighter,
    defender_name: &str,
    defender: &mut Fighter,
    random_generator: &mut RandomGenerator,
) -> bool {
    let mut does_hit = false;
    let mut does_crit = false;

    let rng = random_generator.next();
    if rng <= attacker.acc as u8 {
        does_hit = true;
    } else {
        let rng = random_generator.next();
        if rng <= defender.evasion as u8 {
            does_hit = false;
        } else {
            does_hit = true;
        }
    }

    if does_hit {
        let rng = random_generator.next();
        if rng <= attacker.crit as u8 {
            does_crit = true;
        }

        msg!("{} hits {}!", attacker_name, defender_name);

        let damage = get_attack_damage(attacker.atk, defender.armor, random_generator);
        let final_damage = if does_crit {
            msg!("{} lands a critical hit!", attacker_name);
            damage * 2
        } else {
            damage
        };

        msg!("{} deals {} damage!", attacker_name, final_damage);
        defender.hp -= final_damage;

        if defender.hp <= 0 {
            defender.hp = 0;
            msg!("{}'s hp drops to 0! {} wins!", defender_name, attacker_name);
            return true;
        }

        msg!("{}'s hp drops to {}!", defender_name, defender.hp);
    } else {
        msg!("{} evades {} successfully!", defender_name, attacker_name);
    }

    false
}

fn get_attack_damage(
    player_atk: i32,
    enemy_armor: i32,
    random_generator: &mut RandomGenerator,
) -> i32 {
    let enemy_reduction = 0.06 * enemy_armor as f32 / (1.0 + 0.06 * enemy_armor as f32);
    let damage = player_atk as f32 * (1.0 - enemy_reduction);
    damage.ceil() as i32
}

pub fn process_account(name: &str, account: &Option<AccountInfo>) -> Result<i32> {
    if let Some(account) = account {
        let metadata_data = account.try_borrow_data()?;
        let asset = Asset::from_bytes(&metadata_data).unwrap();
        let uri = &asset.base.uri;
        msg!("{} Raw data as string: {:?}", name, uri);

        let parsed_params = parse_query_params(uri.as_str());
        let upgrade_value = parsed_params
            .into_iter()
            .find(|(key, _)| key == "upgrade")
            .and_then(|(_, value)| value.parse::<i32>().ok())
            .unwrap_or(0);

        msg!("{} Upgrade Value: {}", name, upgrade_value);
        return Ok(upgrade_value);
    } else {
        msg!("{}: None", name);
    }

    Ok(0)
}

fn parse_query_params(uri: &str) -> Vec<(String, String)> {
    let mut params = Vec::new();

    if let Some(query_start) = uri.find('?') {
        let query_str = &uri[query_start + 1..];

        for param in query_str.split('&') {
            let mut key_value = param.splitn(2, '=');
            let key = key_value.next().unwrap_or("").to_string();
            let value = key_value.next().unwrap_or("").to_string();
            params.push((key, value));
        }
    }

    params
}

pub struct RandomGenerator {
    seed: u64,
    counter: u64,
}

impl RandomGenerator {
    pub fn new(seed: i64) -> Self {
        Self {
            seed: seed as u64,
            counter: 0,
        }
    }

    pub fn next(&mut self) -> u8 {
        let a: u64 = 1664525;
        let c: u64 = 1013904223;
        let m: u64 = 2_u64.pow(32);

        let current_seed = self.seed.wrapping_add(self.counter);
        self.counter += 1;

        let random = (a.wrapping_mul(current_seed).wrapping_add(c)) % m;
        (random % 101) as u8
    }
}

#[derive(Accounts)]
pub struct Fight<'info> {
    pub chestplate: Option<AccountInfo<'info>>,
    pub gloves: Option<AccountInfo<'info>>,
    pub boots: Option<AccountInfo<'info>>,
    pub sword: Option<AccountInfo<'info>>,
    pub helmet: Option<AccountInfo<'info>>,
    pub neck: Option<AccountInfo<'info>>,
    pub ring: Option<AccountInfo<'info>>,

    #[account(mut)]
    pub payer: Signer<'info>,
}
