CREATE TABLE `sys_role` (
  `id`              BIGINT                AUTO_INCREMENT                  COMMENT '角色ID',
  `name`            VARCHAR(255)          NOT NULL                        COMMENT '角色名, 唯一',
  `data_scope`      TINYINT UNSIGNED      DEFAULT 0                       COMMENT '数据访问范围',
  `default`         BOOLEAN               DEFAULT 0                       COMMENT '是否为默认角色',
  `desc`            VARCHAR(255)          DEFAULT ''                      COMMENT '描述',
  `is_deleted`      BOOLEAN               DEFAULT 0                       COMMENT '软删除',
  `created_by`      BIGINT                NOT NULL                        COMMENT '创建人',
  `created_at`      TIMESTAMP             DEFAULT CURRENT_TIMESTAMP       COMMENT '创建时间, 自动',
  `updated_at`      TIMESTAMP             DEFAULT CURRENT_TIMESTAMP 
                                          ON UPDATE CURRENT_TIMESTAMP     COMMENT '更新时间, 自动',
  PRIMARY KEY (`id`),
  UNIQUE (`name`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB COMMENT = '角色表';
