CREATE TABLE `sys_dept` (
  `id`              BIGINT            AUTO_INCREMENT                  COMMENT '部门ID',
  `pid`             BIGINT            DEFAULT NULL                    COMMENT '父部门ID',
  `name`            VARCHAR(255)      NOT NULL                        COMMENT '部门名称',
  `desc`            VARCHAR(255)      DEFAULT ''                      COMMENT '描述',
  `is_deleted`      BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  `created_by`      BIGINT            NOT NULL                        COMMENT '创建人',
  `created_at`      TIMESTAMP         DEFAULT CURRENT_TIMESTAMP       COMMENT '创建时间, 自动',
  `updated_at`      TIMESTAMP         DEFAULT CURRENT_TIMESTAMP 
                                      ON UPDATE CURRENT_TIMESTAMP     COMMENT '更新时间, 自动',
  PRIMARY KEY (`id`, `pid`),
  INDEX (`name`),
  INDEX (`is_deleted`),
  INDEX (`created_by`)
) ENGINE = InnoDB COMMENT = '部门表';
