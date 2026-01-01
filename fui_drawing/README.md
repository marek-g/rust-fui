# fui_drawing

Drawing objects for FUI UI Framework.

This crate re-exports drawing types (like `DrawingContext`, `Surface`, `Texture` and others) from `drawing` crates (`drawing-api`, `drawing-impeller` etc). The crate decides which backend implementation to use currently. The re-exported types can be used by FUI Framework and FUI Applications directly.

This is a flexible solution:
- no changes should be needed for depended crates when backend is replaced (all backends implement API defined in `drawing-api` crate)
- possibility to provide different backend implementations for different platforms
- possibility to provide different backend implementations as features
- using the types directly gives the best performance

This approach was chosen after not being satisfied with other ones:

1. Passing `DrawingContext` as a generic parameter to controls. No matter how I tried it polluted half the types in the whole library including types used in `ui!` macro, which made the library hard to use and ugly to read.

2. Using `dyn DrawingContextObjext`. As it is perfectly fine to use dynamic dispatch for control types, it doesn't make sense to do so for such granular operations like drawing primitives. It would prevent compiler from optimizing that code, which is crucial to achieving best performance.
