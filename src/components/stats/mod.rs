/// Stores the core stats of an entity.
/// 
/// Stats can be fetched using a `StatCollectionEvent`,
/// and changed using a `StatChangeEvent`.
pub mod characteristic_stats;

pub mod stat;

/// A statictic with a base value, and an optional cap.
/// 
/// Any nonpermanent change to a stat should be accomplished through
/// a `StatModifier`, rather than through setter methods,
/// with the exception of stats that are constantly changing, such as `HP`.
pub mod capped_stat;

/// A dynamic modifier to a stat.
/// 
/// Each modifier gives a flat bonus or penalty to the stat or its cap.
pub mod stat_modifier;

#[cfg(test)]
pub(crate) mod test_util;