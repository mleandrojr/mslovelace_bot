alter table ada.shield add username varchar(50) null default null after telegram_user_id;
alter table ada.shield add unique key idx_uq_user_id (user_id);
alter table ada.shield change column telegram_user_id user_id bigint(10) null default null;
alter table ada.shield drop key idx_telegram_user_id, add unique key idx_user_id (user_id);
