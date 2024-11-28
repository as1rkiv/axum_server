CREATE TABLE `opt_car_brand` (
  `id`              BIGINT            AUTO_INCREMENT                  COMMENT '品牌ID',
  `name`            VARCHAR(255)      NOT NULL                        COMMENT '品牌名称',
  `logo`            VARCHAR(255)      NOT NULL                        COMMENT '品牌logo',
  `country`         VARCHAR(255)      NOT NULL                        COMMENT '所属国家',
  `firstletter`     VARCHAR(2)        NOT NULL                        COMMENT '首字母',
  `is_deleted`      BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  PRIMARY KEY (`id`),
  INDEX (`name`),
  INDEX (`country`),
  INDEX (`firstletter`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB COMMENT = '车辆品牌表';
