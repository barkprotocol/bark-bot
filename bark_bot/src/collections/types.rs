use crate::collections::Handler;
use std::sync::Arc;
use teloxide::{dispatching::dialogue::ErasedStorage, prelude::*};

// Defines a type alias for the dialogue, using the `Handler` enum for dialogue states and `ErasedStorage<Handler>` for storage.
pub type MyDialogue = Dialogue<Handler, ErasedStorage<Handler>>;

// Defines a result type for handlers that returns an empty `()` on success, but can return errors boxed as `Box<dyn std::error::Error + Send + Sync>`.
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

// Defines a type alias for storage that holds the dialogue state, using `Arc` to share ownership in a thread-safe way.
pub type JoinStorage = Arc<ErasedStorage<Handler>>;
