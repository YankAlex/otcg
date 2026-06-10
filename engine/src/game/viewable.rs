use crate::game::{player::{self, Player}, visibility::Visibility};

pub trait Viewable: Sync {
    fn visibility(&self) -> impl Future<Output = Visibility> + Send;
    fn owner(&self) -> impl Future<Output = Player> + Send;
    fn can_be_viewed_by(&self, viewer: &Player) -> impl Future<Output = bool> + Send {
        async move {
            match viewer {
                &player::ADMIN => true,
                player if *player == self.owner().await && self.visibility().await.can_be_viewed_by_owner() => true,
                _ if self.visibility().await.can_be_viewed_by_not_owner() => true,
                _ => false,
            }
        }
    }
}
