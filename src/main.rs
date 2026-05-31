use json::JsonValue;
use json::JsonValue::Null;
use reqwest;
use std::io::Read;
use std::env;
use std::fs;
use json;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let card_num = &args[1];

    let mut res = reqwest::blocking::get(format!("https://arkhamdb.com/api/public/card/{card_num}"))?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let json = json::parse(&body).unwrap();

    let file= fs::File::create(format!("results/{card_num}.ts")).unwrap();

    if body.contains("\"asset\"") {
        parse_asset(json, file);
    } else if body.contains("\"skill\"") {
        parse_skill(json, file);
    } else if body.contains("\"event\"") {
        parse_event(json, file);
    } else if body.contains("\"enemy\"") {
        parse_enemy(json, file);
    } else if body.contains("\"location\"") {
        parse_location(json, file);
    } else if body.contains("\"treachery\"") {
        parse_treachery(json, file);
    } else if body.contains("\"agenda\"") {
        parse_agenda(json, file);
    } else if body.contains("\"act\"") {
        parse_act(json, file);
    } else {
        panic!("not parseable")
    }
    Ok(())
}

#[allow(unused_must_use)]
fn parse_asset(card: JsonValue, mut file: fs::File) {
    let slot = if &card["slot"] != &Null { &card["slot"].to_string() } else { "" };
    let code = &card["code"];
    let cost = &card["cost"];
    let skill_agility = if &card["skill_agility"] != &Null { &card["skill_agility"].to_string() } else { "0" };
    let skill_combat = if &card["skill_combat"] != &Null { &card["skill_combat"].to_string() } else { "0" };
    let skill_intellect = if &card["skill_intellect"] != &Null { &card["skill_intellect"].to_string() } else { "0" };
    let skill_willpower = if &card["skill_willpower"] != &Null { &card["skill_willpower"].to_string() } else { "0" };
    let skill_wildcard = if &card["skill_wildcard"] != &Null { &card["skill_wildcard"].to_string() } else { "0" };
    let xp = if &card["xp"] != &Null { &card["xp"].to_string() } else { "0" };
    let deck_limit = &card["deck_limit"];
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };
    let restrictions = &if &card["restrictions"] != &Null { &card["restrictions"].to_string() } else { "{}" };

    write!(file, r#"
import {{ AssetCard }} from "server/abstracts/card_inherits/player_card_inherits/costing_card_inherits/asset_card";

export class _{code} extends AssetCard {{
    slot = "{slot}";
    cost = {cost};
    skill_agility = {skill_agility};
    skill_combat = {skill_combat};
    skill_intellect = {skill_intellect};
    skill_willpower = {skill_willpower};
    skill_wildcard = {skill_wildcard};
    xp = {xp};
    deck_limit = {deck_limit};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
    restrictions = {restrictions};
}}"#);
}

#[allow(unused_must_use)]
fn parse_skill(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let skill_agility = if &card["skill_agility"] != &Null { &card["skill_agility"].to_string() } else { "0" };
    let skill_combat = if &card["skill_combat"] != &Null { &card["skill_combat"].to_string() } else { "0" };
    let skill_intellect = if &card["skill_intellect"] != &Null { &card["skill_intellect"].to_string() } else { "0" };
    let skill_willpower = if &card["skill_willpower"] != &Null { &card["skill_willpower"].to_string() } else { "0" };
    let skill_wildcard = if &card["skill_wildcard"] != &Null { &card["skill_wildcard"].to_string() } else { "0" };
    let xp = if &card["xp"] != &Null { &card["xp"].to_string() } else { "0" };
    let deck_limit = &card["deck_limit"];
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };
    let restrictions = &if &card["restrictions"] != &Null { &card["restrictions"].to_string() } else { "{}" };

    write!(file, r#"
import {{ SkillCard }} from "server/abstracts/card_inherits/player_card_inherits/skill_card";

export class _{code} extends SkillCard {{
    skill_agility = {skill_agility};
    skill_combat = {skill_combat};
    skill_intellect = {skill_intellect};
    skill_willpower = {skill_willpower};
    skill_wildcard = {skill_wildcard};
    xp = {xp};
    deck_limit = {deck_limit};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
    restrictions = {restrictions};
}}
"#);
}

#[allow(unused_must_use)]
fn parse_event(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let cost = &card["cost"];
    let skill_agility = if &card["skill_agility"] != &Null { &card["skill_agility"].to_string() } else { "0" };
    let skill_combat = if &card["skill_combat"] != &Null { &card["skill_combat"].to_string() } else { "0" };
    let skill_intellect = if &card["skill_intellect"] != &Null { &card["skill_intellect"].to_string() } else { "0" };
    let skill_willpower = if &card["skill_willpower"] != &Null { &card["skill_willpower"].to_string() } else { "0" };
    let skill_wildcard = if &card["skill_wildcard"] != &Null { &card["skill_wildcard"].to_string() } else { "0" };
    let xp = if &card["xp"] != &Null { &card["xp"].to_string() } else { "0" };
    let deck_limit = &card["deck_limit"];
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };
    let restrictions = &if &card["restrictions"] != &Null { &card["restrictions"].to_string() } else { "{}" };

    write!(file, r#"
import {{ EventCard }} from "server/abstracts/card_inherits/player_card_inherits/costing_card_inherits/event_card";

export class _{code} extends EventCard {{
    cost = {cost};
    skill_agility = {skill_agility};
    skill_combat = {skill_combat};
    skill_intellect = {skill_intellect};
    skill_willpower = {skill_willpower};
    skill_wildcard = {skill_wildcard};
    xp = {xp};
    deck_limit = {deck_limit};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
    restrictions = {restrictions};
}}
"#);
}

#[allow(unused_must_use)]
fn parse_enemy(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let health = &card["health"];
    let enemy_damage = &card["enemy_damage"];
    let enemy_horror = if &card["enemy_horror"] != &Null { &card["enemy_horror"].to_string() } else { "0" };
    let enemy_fight = &card["enemy_fight"];
    let enemy_evade = &card["enemy_evade"];
    let victory = if &card["victory"] != &Null { &card["victory"].to_string() } else { "0" };
    let encounter_name = if &card["encounter_name"] != &Null { &card["encounter_name"].to_string() } else { "" };
    let encounter_position= if &card["encounter_position"] != &Null { &card["encounter_position"].to_string() } else { "0" };
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };
    let restrictions = &if &card["restrictions"] != &Null { &card["restrictions"].to_string() } else { "{}" };

    write!(file, r#"
import {{ EnemyCard }} from "server/abstracts/card_inherits/nonplayer_card_inherits/hostile_card_inherits/enemy_card";
import {{ GamePlayer }} from "server/player";

export class _{code} extends EnemyCard {{
    health = {health};
    enemy_damage = {enemy_damage};
    enemy_horror = {enemy_horror};
    enemy_fight = {enemy_fight};
    enemy_evade = {enemy_evade};
    victory = {victory};
    engagedWith: GamePlayer | undefined;
    encounter_name = "{encounter_name}";
    encounter_position = {encounter_position};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
    restrictions = {restrictions};
}}
"#);
}

#[allow(unused_must_use)]
fn parse_location(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let shroud = &card["shroud"];
    let clues = &card["clues"];
    let stage = &card["stage"];
    let back_name = if &card["back_name"] != &Null { &card["back_name"].to_string() } else { "" };
    let back_text = if &card["back_text"] != &Null { &card["back_text"].to_string() } else { "" };
    let back_flavor = if &card["back_flavor"] != &Null { &card["back_flavor"].to_string() } else { "" };
    let encounter_name = if &card["encounter_name"] != &Null { &card["encounter_name"].to_string() } else { "" };
    let encounter_position= if &card["encounter_position"] != &Null { &card["encounter_position"].to_string() } else { "0" };
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };

    write!(file, r#"
import {{ LocationCard }} from "server/abstracts/card_inherits/nonplayer_card_inherits/story_card_inherits/location_card";

export class _{code} extends LocationCard {{
    shroud = {shroud};
    clues = {clues};
    stage = {stage};
    back_name = "{back_name}";
    back_text = `{back_text}`;
    back_flavor = `{back_flavor}`;
    encounter_name = "{encounter_name}";
    encounter_position = {encounter_position};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
}}
"#);
}

#[allow(unused_must_use)]
fn parse_treachery(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let encounter_name = if &card["encounter_name"] != &Null { &card["encounter_name"].to_string() } else { "" };
    let encounter_position= if &card["encounter_position"] != &Null { &card["encounter_position"].to_string() } else { "0" };
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };
    let restrictions = &if &card["restrictions"] != &Null { &card["restrictions"].to_string() } else { "{}" };

    write!(file, r#"
import {{ TreacheryCard }} from "server/abstracts/card_inherits/nonplayer_card_inherits/hostile_card_inherits/treachery_card";

export class _{code} extends TreacheryCard {{
    encounter_name = "{encounter_name}";
    encounter_position = {encounter_position};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
    restrictions = {restrictions};
}}
"#);
}

#[allow(unused_must_use)]
fn parse_agenda(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let doom = &card["doom"];
    let stage = &card["stage"];
    let back_name = if &card["back_name"] != &Null { &card["back_name"].to_string() } else { "" };
    let back_text = if &card["back_text"] != &Null { &card["back_text"].to_string() } else { "" };
    let back_flavor = if &card["back_flavor"] != &Null { &card["back_flavor"].to_string() } else { "" };
    let encounter_name = if &card["encounter_name"] != &Null { &card["encounter_name"].to_string() } else { "" };
    let encounter_position= if &card["encounter_position"] != &Null { &card["encounter_position"].to_string() } else { "0" };
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };

    write!(file, r#"
import {{ AgendaCard }} from "server/abstracts/card_inherits/nonplayer_card_inherits/story_card_inherits/agenda_card";

export class _{code} extends AgendaCard {{
    doom = {doom};
    stage = {stage};
    back_name = "{back_name}";
    back_text = `{back_text}`;
    back_flavor = `{back_flavor}`;
    encounter_name = "{encounter_name}";
    encounter_position = {encounter_position};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
}}
"#);
}

#[allow(unused_must_use)]
fn parse_act(card: JsonValue, mut file: fs::File) {
    let code = &card["code"];
    let clues = &card["clues"];
    let stage = &card["stage"];
    let back_name = if &card["back_name"] != &Null { &card["back_name"].to_string() } else { "" };
    let back_text = if &card["back_text"] != &Null { &card["back_text"].to_string() } else { "" };
    let back_flavor = if &card["back_flavor"] != &Null { &card["back_flavor"].to_string() } else { "" };
    let encounter_name = if &card["encounter_name"] != &Null { &card["encounter_name"].to_string() } else { "" };
    let encounter_position= if &card["encounter_position"] != &Null { &card["encounter_position"].to_string() } else { "0" };
    let pack_name = &card["pack_name"];
    let type_name = &card["type_name"];
    let faction_name = &card["faction_name"];
    let position = &card["position"];
    let exceptional = &card["exceptional"];
    let myriad = &card["myriad"];
    let name = &card["name"];
    let quantity = &card["quantity"];
    let health_per_investigator = &card["health_per_investigator"];
    let is_unique = &card["is_unique"];
    let permanent = &card["permanent"];
    let double_sided = &card["double_sided"];
    let text = &card["text"];
    let traits = &card["traits"];
    let flavor = if &card["flavor"] != &Null { &card["flavor"].to_string() } else { "" };
    let subname = if &card["subname"] != &Null { &card["subname"].to_string() } else { "" };

    write!(file, r#"
import {{ ActCard }} from "server/abstracts/card_inherits/nonplayer_card_inherits/story_card_inherits/act_card";

export class _{code} extends ActCard {{
    clues = {clues};
    stage = {stage};
    back_name = "{back_name}";
    back_text = `{back_text}`;
    back_flavor = `{back_flavor}`;
    encounter_name = "{encounter_name}";
    encounter_position = {encounter_position};
    code = "{code}";
    pack_name = "{pack_name}";
    type_name = "{type_name}";
    faction_name = "{faction_name}";
    position = {position};
    exceptional = {exceptional};
    myriad = {myriad};
    name = "{name}";
    quantity = {quantity};
    health_per_investigator = {health_per_investigator};
    is_unique = {is_unique};
    permanent = {permanent};
    double_sided = {double_sided};
    text = `{text}`;
    traits = "{traits}";
    flavor = `{flavor}`;
    subname = "{subname}";
}}
"#);
}