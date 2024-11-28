CREATE TABLE `sys_dept_menu` (
  `dept_id`           BIGINT    NOT NULL    COMMENT '部门ID',
  `menu_id`           BIGINT    NOT NULL    COMMENT '菜单ID',
  PRIMARY KEY (`dept_id`, `menu_id`)
) ENGINE = InnoDB COMMENT = '部门与菜单关联表';
