CREATE TABLE `sys_user` (
  `id`                BIGINT            AUTO_INCREMENT                  COMMENT '用户ID',
  `username`          VARCHAR(255)      NOT NULL                        COMMENT '用户名, 唯一',
  `password`          VARCHAR(255)      NOT NULL                        COMMENT '密码',
  `fullname`          VARCHAR(255)      NOT NULL                        COMMENT '姓名',
  `last_login_ip`     VARCHAR(255)      DEFAULT NULL                    COMMENT '最后登录IP',
  `last_login_at`     TIMESTAMP         DEFAULT NULL                    COMMENT '最后登录时间',
  `avatar`            VARCHAR(255)      DEFAULT ''                      COMMENT '头像',
  `is_active`         BOOLEAN           DEFAULT 1                       COMMENT '是否激活',
  `is_deleted`        BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  `created_by`        BIGINT            NOT NULL                        COMMENT '创建人',
  `created_at`        TIMESTAMP         DEFAULT CURRENT_TIMESTAMP       COMMENT '创建时间, 自动',
  `updated_at`        TIMESTAMP         DEFAULT CURRENT_TIMESTAMP 
                                        ON UPDATE CURRENT_TIMESTAMP     COMMENT '更新时间, 自动',
  PRIMARY KEY (`id`),
  UNIQUE (`username`),
  INDEX (`fullname`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB AUTO_INCREMENT = 1000 COMMENT = '用户表';

-- 初始管理员
INSERT INTO `sys_user` 
  (`id`, `username`, `password`, `fullname`, `created_by`)
VALUES
  (1, 'admin', 'admin', '超级管理员', 0);
