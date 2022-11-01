use std::ops::DerefMut;

use bevy::prelude::*;
use turborand::rng::AtomicRng;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct Random(AtomicRng);
