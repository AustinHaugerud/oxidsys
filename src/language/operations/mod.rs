pub mod agents_and_teams;
pub mod audio;
pub mod factions;
pub mod flow_control;
pub mod game;
pub mod game_control;
pub mod game_notes;
pub mod input;
pub mod item;
pub mod math;
pub mod mercantile;
pub mod multiplayer;
pub mod op_modifiers;
pub mod other;
pub mod output_and_messages;
pub mod party;
pub mod position;
pub mod presentation;
pub mod quest;
pub mod scene_resources;
pub mod scenes_and_missions;
pub mod script;
pub mod string;
pub mod tableau;
pub mod trigger;
pub mod troop;
pub mod world;

/**
 * A special thank you to Alexander Lomski AKA Lav for providing an excellent basis
 * on how to organize and document operations. The below is pasted from Lav's
 edited header_operations.py with awesome organization and documentation.

################################################################################
# [ Z00 ] INTRODUCTION AND CREDITS
################################################################################

# Everyone who has ever tried to mod Mount&Blade games knows perfectly well,
# that the documentation for it's Module System is severely lacking. Warband
# Module System, while introducing many new and useful operations, did not
# improve considerably in the way of documentation. What's worse, a number of
# outright errors and inconsistencies appeared between what was documented in
# the comments to the header_operations.py file (which was the root source of
# all Warband scripting documentation, whether you like it or not), and what
# was actually implemented in the game engine.

# Sooner or later someone was bound to dedicate some time and effort to fix
# this problem by properly documenting the file. It just so happened that I
# was the first person crazy enough to accept the challenge.

# I have tried to make this file a self-sufficient source of information on
# every operation that the Warband scripting engine knows of. Naturally I
# failed - there are still many operations for which there is simply not
# enough information, or operations with effects that have not yet been
# thoroughly tested and confirmed. But as far as I know, there is currently
# no other reference more exhaustive than this. I tried to make the file
# useful to both seasoned scripters and complete newbies, and to a certain
# degree this file can even serve as a tutorial into Warband scripting -
# though it still won't replace the wealth of tutorials produced by the
# Warband modding community.

# I really hope you will find it useful as well.

#                                    Alexander Lomski AKA Lav. Jan 18th, 2012.

# And the credits.

# First of all, I should credit Taleworlds for the creation of this game and
# it's Module System. Without them, I wouldn't be able to work on this file
# so even though I'm often sceptical about their programming style and quality
# of their code, they still did a damn good job delivering this game to all
# of us.

# And then I should credit many members from the Warband modding community
# who have shared their knowledge and helped me clear out many uncertainties
# and inconsistencies. Special credits (in no particular order) go to
# cmpxchg8b, Caba'drin, SonKidd, MadVader, dunde, Ikaguia, MadocComadrin,
# Cjkjvfnby, shokkueibu.
*/
use std::collections::HashMap;

pub trait Operation {
    fn op_code(&self) -> u32;
    fn documentation(&self) -> &'static str;
    fn identifier(&self) -> &'static str;
}

fn load_operands_list() -> Vec<Box<Operation>> {
    let mut result = agents_and_teams::load_operands();
    result.extend(audio::load_operands());
    result.extend(factions::load_operands());
    result.extend(flow_control::load_operands());
    result.extend(game::load_operands());
    result.extend(game_control::load_operands());
    result.extend(game_notes::load_operands());
    result.extend(input::load_operands());
    result.extend(item::load_operands());
    result.extend(math::load_operands());
    result.extend(mercantile::load_operands());
    result.extend(multiplayer::load_operands());
    result.extend(op_modifiers::load_operands());
    result.extend(other::load_operands());
    result.extend(output_and_messages::load_operands());
    result.extend(party::load_operands());
    result.extend(position::load_operands());
    result.extend(presentation::load_operands());
    result.extend(quest::load_operands());
    result.extend(scene_resources::load_operands());
    result.extend(scenes_and_missions::load_operands());
    result.extend(script::load_operands());
    result.extend(string::load_operands());
    result.extend(tableau::load_operands());
    result.extend(trigger::load_operands());
    result.extend(troop::load_operands());
    result.extend(world::load_operands());
    result
}

pub fn load_operands_map() -> HashMap<&'static str, Box<Operation>> {
    let list = load_operands_list();
    let mut map: HashMap<&str, Box<Operation>> = HashMap::new();

    for op in list {
        map.insert(op.identifier(), op);
    }

    map
}
