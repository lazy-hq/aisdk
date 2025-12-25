//! Extensions are a way to extend the capabilities of `aisdk`.
//! They are used to attach provider-specific information to core SDK structures without polluting the core API.

use parking_lot::{
    MappedRwLockReadGuard, MappedRwLockWriteGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// Extensions are a type-safe container for storing arbitrary metadata.
#[derive(Default, Clone)]
pub struct Extensions {
    map: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl Extensions {
    /// Inserts a value into the extensions map.
    pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
        self.map.write().insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Gets a value from the extensions map.
    pub fn get<T: Send + Sync + Default + 'static>(&self) -> MappedRwLockReadGuard<'_, T> {
        self.ensure::<T>();
        RwLockReadGuard::map(self.map.read(), |m| {
            m.get(&TypeId::of::<T>())
                .and_then(|b| b.downcast_ref())
                .unwrap()
        })
    }

    /// Gets a mutable value from the extensions map.
    pub fn get_mut<T: Send + Sync + Default + 'static>(&self) -> MappedRwLockWriteGuard<'_, T> {
        self.ensure::<T>();
        RwLockWriteGuard::map(self.map.write(), |m| {
            m.get_mut(&TypeId::of::<T>())
                .and_then(|b| b.downcast_mut())
                .unwrap()
        })
    }

    /// Ensures that a value of the given type is present in the extensions map.
    fn ensure<T: Default + Send + Sync + 'static>(&self) {
        if self.map.read().get(&TypeId::of::<T>()).is_none() {
            self.insert(T::default());
        }
    }
}

impl std::fmt::Debug for Extensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}
