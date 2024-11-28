CREATE TABLE `sys_menu` (
  `id`                  BIGINT            AUTO_INCREMENT                  COMMENT '菜单ID',
  `pid`                 BIGINT            DEFAULT NULL                    COMMENT '父菜单ID',
  `path`                VARCHAR(255)      DEFAULT ''                      COMMENT '路径',
  `name`                VARCHAR(255)      DEFAULT ''                      COMMENT '名称',
  `component`           VARCHAR(255)      NOT NULL                        COMMENT '前端组件/鉴权资源',
  `action`              VARCHAR(8)        DEFAULT ''                      COMMENT '鉴权动作',
  `redirect`            VARCHAR(255)      DEFAULT NULL                    COMMENT '重定向',
  -- meta
  `title`               VARCHAR(255)      NOT NULL                        COMMENT '菜单标题',
  `icon`                VARCHAR(255)      DEFAULT NULL                    COMMENT '菜单图标',
  `order_no`            SMALLINT          DEFAULT 0                       COMMENT '显示顺序',
  `single`              BOOLEAN           DEFAULT 0                       COMMENT '单独显示',
  `expanded`            BOOLEAN           DEFAULT 0                       COMMENT '展开',
  `hidden`              BOOLEAN           DEFAULT 0                       COMMENT '隐藏菜单',
  `hidden_breadcrumb`   BOOLEAN           DEFAULT 0                       COMMENT '隐藏面包屑',
  `keep_alive`          BOOLEAN           DEFAULT 0                       COMMENT '保持活跃',
  `frame_src`           VARCHAR(255)      DEFAULT NULL                    COMMENT '外部链接地址',
  `frame_blank`         BOOLEAN           DEFAULT 0                       COMMENT '外部链接是否新窗口打开',
  `enforce`             BOOLEAN           DEFAULT 0                       COMMENT '是否需要鉴权',
  `is_deleted`          BOOLEAN           DEFAULT 0                       COMMENT '软删除',
  `created_by`          BIGINT            NOT NULL                        COMMENT '创建人',
  `created_at`          TIMESTAMP         DEFAULT CURRENT_TIMESTAMP       COMMENT '创建时间, 自动',
  `updated_at`          TIMESTAMP         DEFAULT CURRENT_TIMESTAMP 
                                          ON UPDATE CURRENT_TIMESTAMP     COMMENT '更新时间, 自动',
  PRIMARY KEY (`id`, `pid`),
  INDEX `enforce_resource_action` (`component`, `action`, `enforce`),
  INDEX (`is_deleted`)
) ENGINE = InnoDB COMMENT = '菜单表';
