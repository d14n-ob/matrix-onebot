use matrix_sdk::Room;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

pub async fn unsafe_action_join_all_room(room: Room) {
    info!("Unsafe action: Auto joined room {}", room.room_id());
    let mut delay = 2;

    while let Err(err) = room.join().await {
        warn!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

        sleep(Duration::from_secs(delay)).await;
        delay *= 2;

        if delay > 3600 {
            warn!("Can't join room {} ({err:?})", room.room_id());
            break;
        }
    }
    info!("Successfully joined room {}", room.room_id());
}