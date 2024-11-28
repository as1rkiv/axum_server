CREATE TABLE `opt_car_series` (
  `id`              BIGINT            AUTO_INCREMENT                  COMMENT '系列ID',
  `brand_id`        BIGINT            NOT NULL                        COMMENT '品牌ID',
  `name`            VARCHAR(255)      NOT NULL                        COMMENT '系列名称',
  `level`           VARCHAR(255)      NOT NULL                        COMMENT '系列等级',
  `logo`            VARCHAR(255)      NOT NULL                        COMMENT '系列logo',
  `factory`         VARCHAR(255)      NOT NULL                        COMMENT '制造工厂',
  `power`           VARCHAR(255)      NOT NULL                        COMMENT '动力类型',
  `is_deleted`      BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  PRIMARY KEY (`id`),
  INDEX (`brand_id`),
  INDEX (`name`),
  INDEX (`level`),
  INDEX (`factory`),
  INDEX (`power`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB COMMENT = '车辆品牌系列表';
