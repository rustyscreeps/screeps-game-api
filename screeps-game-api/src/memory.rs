use stdweb::{Array, JsSerialize, Reference, Value};
use stdweb::unstable::{TryFrom, TryInto};

/// TODO: do we even need this over just a raw 'Reference'?
pub struct MemoryReference(Reference);

impl MemoryReference {
    pub fn num(&self, path: &str) -> Option<f64> {
        (js! {
            return (@{&self.0})[@{path}];
        }).try_into()
            .map(Some)
            .unwrap_or_default()
    }

    pub fn int(&self, path: &str) -> Option<i32> {
        (js! {
            return (@{&self.0})[@{path}];
        }).try_into()
            .map(Some)
            .unwrap_or_default()
    }

    pub fn dict(&self, path: &str) -> Option<MemoryReference> {
        (js! {
            var v = (@{&self.0})[@{path}];
            if (_.isArray(v)) {
                return null;
            } else {
                return v || null;
            }
        }).try_into()
            .map(Some)
            .unwrap_or_default()
            .map(MemoryReference)
    }

    pub fn keys(&self) -> Vec<String> {
        js_unwrap!(Object.keys(@{&self.0}))
    }

    pub fn del(&self, path: &str) {
        js! {
            (@{&self.0})[@{path}] = undefined;
        }
    }

    pub fn set<T>(&self, path: &str, value: T)
    where
        T: JsSerialize,
    {
        js! {
            (@{&self.0})[@{path}] = @{value};
        }
    }

    pub fn arr<T>(&self, path: &str) -> Option<Vec<T>>
    where
        T: TryFrom<Value, Error = <Reference as TryFrom<Value>>::Error>,
    {
        let x: Reference = (js! {
            var v = (@{&self.0})[@{path}];
            if (!_.isArray(v)) {
                return null;
            } else {
                return v || null;
            }
        }).try_into()
            .ok()?;

        // Memory arrays don't have the regular Array as their prototype - they
        // have the 'outside' type.
        let as_arr: Array = unsafe {
            use stdweb::ReferenceType;
            Array::from_reference_unchecked(x)
        };

        as_arr.try_into().ok()
    }
}

impl TryFrom<Value> for MemoryReference {
    type Error = <Reference as TryFrom<Value>>::Error;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let r: Reference = v.try_into()?; // fail early.
        Ok(MemoryReference((js! {
            var v = (@{r});
            if (_.isArray(v)) {
                return null;
            } else {
                return v;
            }
        }).try_into()?))
    }
}

pub fn root() -> MemoryReference {
    js_unwrap!(Memory)
}
