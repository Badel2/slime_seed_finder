cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // weak-alloc only enabled in webassembly architecture
        // Because it uses 32-bit pointers, so it is limited to 4GB of RAM.
        // We want to use every single byte of that 4GB, so weak-alloc is useful there.
        use std::alloc::System;
        use weak_alloc::WeakAlloc;

        type InnerAlloc = System;

        pub type ArcRef<T> = weak_alloc::ArcRef<T, InnerAlloc>;
        pub type WeakRef<T> = weak_alloc::WeakRef<T, InnerAlloc>;

        #[global_allocator]
        static A: WeakAlloc<System> = WeakAlloc::new(System);

        pub fn give_and_upgrade<T: Send + Sync + 'static>(x: T) -> ArcRef<T> {
            A.give_and_upgrade(x)
        }
    } else {
        pub type ArcRef<T> = std::sync::Arc<T>;
        pub type WeakRef<T> = std::sync::Weak<T>;

        // mock weak-alloc in other architectures
        pub fn give_and_upgrade<T: Send + Sync + 'static>(x: T) -> ArcRef<T> {
            let arc = std::sync::Arc::new(x);
            // Increase refcount
            let leak_arc = arc.clone();
            std::mem::forget(leak_arc);
            // Return Arc with refcount 2 to ensure that the memory will never be deallocated
            // because users may expect to be able to do ArcRef::downgrade and upgrade later.
            arc
        }
    }
}
