use std::{
    future::Future,
    sync::{Arc, OnceLock, Weak},
};

use tokio::sync::Mutex;

use crate::Static;

impl Static {
    pub async fn arc<T, F, Setup>(setup: Setup) -> Arc<T>
    where
        T: 'static,
        F: Future<Output = T>,
        Setup: FnOnce() -> F, {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();

        let _lock = LOCK.get_or_init(|| Mutex::new(())).lock().await;

        if Static::exists::<Weak<T>>() {
            let weak = Static::get::<Weak<T>>();
            return weak.upgrade().unwrap();
        }

        let arc = Arc::new(setup().await);
        let weak = Arc::downgrade(&arc);

        Static::set(weak);

        arc
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Deref, sync::Arc, time::Duration};

    use tokio::time::sleep;

    use crate::Static;

    async fn make_val() -> u32 {
        5
    }

    async fn get_val() -> Arc<u32> {
        Static::arc(make_val).await
    }

    async fn check() {
        let arc = get_val().await;
        sleep(Duration::from_millis(1)).await;
        assert_eq!(arc.deref(), &5);
    }

    #[tokio::test]
    async fn test1() {
        check().await;
    }

    #[tokio::test]
    async fn test2() {
        check().await;
    }

    #[tokio::test]
    async fn test3() {
        check().await;
    }

    #[tokio::test]
    async fn test4() {
        check().await;
    }

    #[tokio::test]
    async fn test6() {
        check().await;
    }

    #[tokio::test]
    async fn test7() {
        check().await;
    }

    #[tokio::test]
    async fn test8() {
        check().await;
    }

    #[tokio::test]
    async fn test9() {
        check().await;
    }
}
