const NAMELESS_ROOT: u8 = 1 << 0;
const DYNAMIC_LISTS: u8 = 1 << 1;

#[derive(Debug, Clone, Copy, Default)]
pub struct NbtOptions {
    flags: u8,
}

impl NbtOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self { flags: 0 }
    }

    /// Since Minecraft 1.20.2, NBT sent doesn't have a name for the root tag
    #[must_use]
    pub const fn nameless_root(mut self, enabled: bool) -> Self {
        if enabled {
            self.flags |= NAMELESS_ROOT;
        } else {
            self.flags &= !NAMELESS_ROOT;
        }
        self
    }

    /// Since Minecraft 1.21.5, Lists can contain elements of different types
    #[must_use]
    pub const fn dynamic_lists(mut self, enabled: bool) -> Self {
        if enabled {
            self.flags |= DYNAMIC_LISTS;
        } else {
            self.flags &= !DYNAMIC_LISTS;
        }
        self
    }

    /// Checks if nameless root is enabled.
    #[must_use]
    pub const fn is_nameless_root(&self) -> bool {
        (self.flags & NAMELESS_ROOT) != 0
    }

    /// Checks if dynamic lists are enabled.
    #[must_use]
    pub const fn is_dynamic_lists(&self) -> bool {
        (self.flags & DYNAMIC_LISTS) != 0
    }
}
