CREATE TABLE `opt_region` (
  `id`              BIGINT            AUTO_INCREMENT                  COMMENT '区域ID',
  `pid`             BIGINT            DEFAULT NULL                    COMMENT '父区域ID',
  `name`            VARCHAR(255)      NOT NULL                        COMMENT '区域名称',
  `code`            VARCHAR(16)       NOT NULL                        COMMENT '行政代码',
  `is_deleted`      BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  PRIMARY KEY (`id`, `pid`),
  INDEX (`name`),
  INDEX (`code`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB COMMENT = '行政区域表';
