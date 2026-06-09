use crate::game::{player::{self, Player}, visibility::Visibility};

pub trait Viewable {
    async fn visibility(&self) -> Visibility;
    async fn owner(&self) -> Player;
    async fn can_be_viewed_by(&self, viewer: &Player) -> bool {
        match viewer {
            &player::ADMIN => true,
            player if *player == self.owner().await && self.visibility().await.can_be_viewed_by_owner() => true,
            _ if self.visibility().await.can_be_viewed_by_not_owner() => true,
            _ => false,
        }
    }
}
