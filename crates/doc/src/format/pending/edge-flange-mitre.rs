//! An Edge Flange feature can choose how the corners its flanges make are mitred: `mitre: false`
//! leaves them open, as between flanges of separate features, and `mitre_gap` (a length or an
//! expression) closes them with the feature's own gap in place of the sheet metal rule's. Older
//! designs need no transformation: a feature without them mitres its corners with the rule's gap,
//! as it always did, and saves as before. The last release asks for an update instead of building
//! the rule's mitre where a design asks for another.
