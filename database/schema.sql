CREATE TABLE IF NOT EXISTS `bans` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `chat_id` int(10) unsigned NOT NULL,
    `federation_id` int(10) unsigned DEFAULT NULL,
    `reason` varchar(50) DEFAULT NULL,
    `date` int(10) unsigned NOT NULL,
    PRIMARY KEY (`id`),
    KEY `idx_user_id_chat_id` (`chat_id`,`user_id`),
    KEY `fk_bans_user_id_idx` (`user_id`),
    KEY `fk_bans_federation_id_idx` (`federation_id`),
    CONSTRAINT `fk_bans_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `fk_bans_federation_id` FOREIGN KEY (`federation_id`) REFERENCES `federations` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `fk_bans_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `blocked_terms` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `chat_id` int(10) unsigned NOT NULL,
    `term` varchar(50) NOT NULL,
    `action` enum('delete','mute','warn','ban') NOT NULL DEFAULT 'delete',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_uq_chat_id_term` (`chat_id`,`term`),
    KEY `idx_chat_id` (`chat_id`),
    CONSTRAINT `fk_blocked_terms_chats` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `chat_configs` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `chat_id` int(10) unsigned NOT NULL,
    `greetings` tinyint(1) NOT NULL DEFAULT 0,
    `goodbye` tinyint(1) NOT NULL DEFAULT 0,
    `warn_name_changing` tinyint(1) NOT NULL DEFAULT 0,
    `remove_event_messages` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `restrict_new_users` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `captcha` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `captcha_ban_seconds` int(10) unsigned NOT NULL DEFAULT 300,
    `warn_ask_to_ask` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `adashield` tinyint(1) unsigned NOT NULL DEFAULT 1,
    `warnings` int(10) unsigned NOT NULL DEFAULT 3,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_uq_chat_id` (`chat_id`),
    KEY `fk_chat_configs_chat_id` (`chat_id`),
    CONSTRAINT `fk_chat_configs_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `chat_messages` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `chat_id` int(10) unsigned NOT NULL,
    `greetings` text DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_uq_chat_id` (`chat_id`),
    KEY `idx_chat_id` (`chat_id`),
    CONSTRAINT `fk_message_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `chat_rules` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `chat_id` int(10) unsigned NOT NULL,
    `rules` text DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_uq_chat_id` (`chat_id`),
    CONSTRAINT `fk_chat_rules_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `chats` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `federation_id` int(10) unsigned DEFAULT NULL,
    `chat_id` bigint(12) NOT NULL,
    `title` varchar(250) NOT NULL,
    `type` varchar(50) NOT NULL,
    `language` varchar(20) NOT NULL DEFAULT 'us',
    `joined` tinyint(1) NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_chats_chat_id` (`chat_id`) USING BTREE,
    KEY `idx_federation_id` (`federation_id`),
    CONSTRAINT `fk_chats_federation_id` FOREIGN KEY (`federation_id`) REFERENCES `federations` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `federations` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `hash` varchar(32) NOT NULL,
    `description` varchar(100) DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_hash` (`hash`),
    KEY `fk_federations_user_id_idx` (`user_id`),
    CONSTRAINT `fk_federations_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `macros` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `chat_id` int(10) unsigned NOT NULL,
    `macro` varchar(50) NOT NULL,
    `content` text NOT NULL,
    PRIMARY KEY (`id`),
    KEY `fk_macro_id_chat_id` (`chat_id`),
    CONSTRAINT `fk_macro_id_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `messages` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `chat_id` int(10) unsigned NOT NULL,
    `thread_id` int(10) unsigned DEFAULT NULL,
    `message_id` int(10) unsigned NOT NULL,
    `type` enum('message','edited_message','channel_post','edited_channel_post','inline_query','chosen_inline_result','callback_query','shipping_query','pre_checkout_query','poll','poll_answer','my_chat_member','chat_member','chat_join_request') NOT NULL DEFAULT 'message',
    `reply_to` int(10) unsigned DEFAULT NULL,
    `content` text DEFAULT NULL,
    `callback_query` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `entities` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `animation` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `audio` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `document` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `photo` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `sticker` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `video` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `video_note` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `voice` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `caption` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `caption_entities` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL,
    `date` int(10) unsigned NOT NULL,
    `ttl` int(10) unsigned DEFAULT NULL,
    `status` tinyint(1) unsigned NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    KEY `fk_messages_reply_to` (`reply_to`),
    KEY `idx_user_id_chat_id` (`user_id`,`chat_id`),
    KEY `idx_chat_id_message_id` (`chat_id`,`message_id`),
    CONSTRAINT `fk_messages_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `fk_messages_reply_to` FOREIGN KEY (`reply_to`) REFERENCES `messages` (`id`),
    CONSTRAINT `fk_messages_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `rel_users_chats` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `chat_id` int(10) unsigned NOT NULL,
    `joined` tinyint(1) unsigned NOT NULL DEFAULT 1,
    `captcha` char(6) DEFAULT NULL,
    `checked` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `date` int(10) unsigned NOT NULL,
    `last_seen` int(10) unsigned NOT NULL,
    `ttl` int(10) unsigned DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_id_chat_id` (`user_id`,`chat_id`),
    KEY `idx_user_id` (`user_id`),
    KEY `idx_chat_id` (`chat_id`),
    CONSTRAINT `fk_users_chats_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`),
    CONSTRAINT `fk_users_chats_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `rel_users_federations` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `federation_id` int(10) unsigned NOT NULL,
    `date` int(10) unsigned NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_uq_user_id_federation_id` (`user_id`,`federation_id`),
    KEY `fk_rel_users_federations_federation_id_idx` (`federation_id`),
    CONSTRAINT `fk_rel_users_federations_federation_id` FOREIGN KEY (`federation_id`) REFERENCES `federations` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION,
    CONSTRAINT `fk_rel_users_federations_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


CREATE TABLE IF NOT EXISTS `shield` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` bigint(10) DEFAULT NULL,
    `username` varchar(50) DEFAULT NULL,
    `date` int(10) unsigned NOT NULL,
    `reason` text DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_id` (`user_id`),
    UNIQUE KEY `idx_username` (`username`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `users` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` bigint(10) NOT NULL,
    `username` varchar(100) DEFAULT NULL,
    `first_name` varchar(100) DEFAULT NULL,
    `last_name` varchar(100) DEFAULT NULL,
    `is_channel` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `is_bot` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `is_premium` tinyint(1) unsigned NOT NULL DEFAULT 0,
    `language_code` varchar(20) DEFAULT 'us',
    PRIMARY KEY (`id`),
    UNIQUE KEY `idx_user_id` (`user_id`) USING BTREE,
    KEY `idx_is_premium` (`is_premium`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `warnings` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `user_id` int(10) unsigned NOT NULL,
    `chat_id` int(10) unsigned NOT NULL,
    `date` int(10) unsigned NOT NULL,
    `reason` varchar(100) DEFAULT NULL,
    `status` tinyint(1) unsigned NOT NULL DEFAULT 1,
    PRIMARY KEY (`id`),
    KEY `idx_user_id_chat_id` (`user_id`,`chat_id`),
    KEY `fk_warns_chat_id` (`chat_id`),
    CONSTRAINT `fk_warns_chat_id` FOREIGN KEY (`chat_id`) REFERENCES `chats` (`id`),
    CONSTRAINT `fk_warns_user_id` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
