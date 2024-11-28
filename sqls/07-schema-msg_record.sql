CREATE TABLE `message_record` (
  `id`              BIGINT                AUTO_INCREMENT                  COMMENT '消息ID',
  `session_id`      BIGINT                DEFAULT NULL                    COMMENT '会话ID',
  `sender_id`       BIGINT                NOT NULL                        COMMENT '发送者ID',
  `receiver_id`     BIGINT                NOT NULL                        COMMENT '接收者ID',
  `msg_type`        TINYINT UNSIGHED      NOT NULL                        COMMENT '消息类型',
  `content`         VARCHAR(512)          NOT NULL                        COMMENT '消息内容',
  `read`            BOOLEAN               DEFAULT 0                       COMMENT '读取状态',
  `sent_ip`         VARCHAR(32)           DEFAULT ''                      COMMENT '发送IP',
  `sent_at`         TIMESTAMP             DEFAULT CURRENT_TIMESTAMP       COMMENT '创建时间, 自动',
  PRIMARY KEY (`id`),
  UNIQUE `sender_receiver_sent_at` (`sender_id`, `receiver_id`, `sent_at`),
  INDEX `sender_receiver` (`sender_id`, `receiver_id`),
  INDEX (`sent_at`)
) ENGINE = InnoDB COMMENT = '消息记录表';
