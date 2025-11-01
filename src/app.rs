//! ФУНДАМЕНТ, ОСНОВА, БАЗА
//!
//! Хендлеры такие (сосал?):
//! - `GET` /skin
//!     Options ->
//!     - body (цвет бадика в ddnet value) Optional
//!     - feet (цвет бадика в ddnet value) Optional
//!     - name (название скинчика) Optional (разделять пока не буду, ибо нахуй, так как там нет повторений)
//! Если нихуя не будет, то шлём нахуй. ошибка 400 Bad Request
//! - `GET` /uvs выдаём папку со всеми uvишками, пусть кайфуют, зеркало же хули
//! - `GET` /health выписывает инфу о: Когда был и будет rsync, текущие TTL с названием, цветом
//!
//! Пример запроса:
//! https:://cdn.sasno.tv/skin?&body=322&feet=322&name=пацан_ваще_качает
//!
//! почему не /skin/{name}, а хули нет?
//!
//! Промежуточные слои...
//! - Сбор статистики, кто, зачем
//! - Мб рейтлимит
//! - Бан лист (:))

pub mod logger;
pub mod png;
pub mod skin;

use ohkami::{Ohkami, Route, claw::status};

use crate::app::skin::skin_router;
async fn health_check() -> status::NoContent {
    status::NoContent
}

pub async fn app() {
    Ohkami::new((
        "/skin".By(skin_router()),
        "/uvs".GET(health_check),
        "/health".GET(health_check),
    ))
    .howl("localhost:3000")
    .await
}
