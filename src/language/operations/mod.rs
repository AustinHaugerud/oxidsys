mod flow_control;

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

pub trait Operation {
    fn op_code(&self) -> u16;
    fn documentation(&self) -> &str;
    fn identifier(&self) -> &str;
}
