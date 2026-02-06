create table blocked_terms (
    id int(10) unsigned auto_increment not null,
    chat_id int(10) unsigned not null,
    term varchar(50) not null,
    action enum('delete', 'mute', 'ban'),
    primary key (id),
    key idx_chat_id (chat_id),
    unique key idx_uq_chat_id_term (chat_id, term),
    constraint fk_blocked_terms_chats foreign key (chat_id) references chats(id)
);
