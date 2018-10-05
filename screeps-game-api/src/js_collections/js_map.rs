//! [`JsMap`]
use std::{collections::HashMap, marker::PhantomData};

use stdweb::Object;

pub struct JsMap<K, T> {
    inner: Object,
    phantom: PhantomData<HashMap<K, V>>,
}

// impl Index<Nucleotide> for NucleotideCount {
//     type Output = usize;

//     fn index(&self, nucleotide: Nucleotide) -> &usize {
//         match nucleotide {
//             Nucleotide::A => &self.a,
//             Nucleotide::C => &self.c,
//             Nucleotide::G => &self.g,
//             Nucleotide::T => &self.t,
//         }
//     }
// }
