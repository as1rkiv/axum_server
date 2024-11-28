CREATE TABLE `sys_user_dept_role` (
  `user_id`     BIGINT    NOT NULL    COMMENT '用户ID',
  `dept_id`     BIGINT    NOT NULL    COMMENT '部门ID',
  `role_id`     BIGINT    NOT NULL    COMMENT '角色ID',
  PRIMARY KEY (`user_id`, `dept_id`, `role_id`)
) ENGINE = InnoDB COMMENT = '用户与部门与角色关联表';
