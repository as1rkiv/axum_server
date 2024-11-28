--
-- 基础功能 
-- 仅用做权限整理 
-- 不显示
--
INSERT INTO `sys_menu` 
  (`id`, `path`, `name`, `component`, `redirect`, `title`, `order_no`, `single`, `hidden`)
VALUES
  (1, '/base', 'base', 'LAYOUT', '/result/maintenance', '基础功能', 1, 1, 1);


--
-- 列表页
--
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `redirect`, `title`, `icon`) 
VALUES (1, '/list', 'list', 'LAYOUT', '/list/base', '列表页', 'view-list');

-- 子列表页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (2, 'base', 'ListBase', '/list/base/index', 1, '基础列表页');

-- 子列表页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (3, 'card', 'ListCard', '/list/card/index', 1, '卡片列表页');

-- 子列表页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (4, 'filter', 'ListFilter', '/list/filter/index', 1, '筛选列表页');

-- 子列表页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (5, 'tree', 'ListTree', '/list/tree/index', 1, '树状筛选列表页');


--
-- 表单页
--
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `redirect`, `title`, `icon`) 
VALUES (6, '/form', 'form', 'LAYOUT', '/form/base', '表单页', 'edit-1');

-- 子表单页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (7, 'base', 'FormBase', '/form/base/index', 6, '基础表单页');

-- 子表单页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (8, 'step', 'FormStep', '/form/step/index', 6, '分步表单页');


--
-- 详情页
--
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `redirect`, `title`, `icon`) 
VALUES (9, '/detail', 'detail', 'LAYOUT', '/detail/base', '详情页', 'layers');

-- 子详情页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (10, 'base', 'DetailBase', '/detail/base/index', 9, '基础详情页');

-- 子详情页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (11, 'advanced', 'DetailAdvanced', '/detail/advanced/index', 9, '多卡片详情页');

-- 子详情页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (12, 'deploy', 'DetailDeploy', '/detail/deploy/index', 9, '数据详情页');

-- 子详情页
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (13, 'secondary', 'DetailSecondary', '/detail/secondary/index', 9, '二级详情页');


--
-- 外部页面
--
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `redirect`, `title`, `icon`) 
VALUES (14, '/frame', 'Frame', 'LAYOUT', '/frame/doc', '外部页面', 'internet');

-- 子外部页面
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`, `frame_src`) 
VALUES (15, 'doc', 'Doc', 'IFrame', 14, '使用文档（内嵌）', 
'https://tdesign.tencent.com/starter/docs/vue-next/get-started');

-- 子外部页面
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`, `frame_src`) 
VALUES (16, 'TDesign', 'TDesign', 'IFrame', 14, 'TDesign 文档（内嵌）', 
'https://tdesign.tencent.com/vue-next/getting-started');

-- 子外部页面
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`, `frame_src`, `frame_blank`) 
VALUES (17, 'TDesign2', 'TDesign2', 'IFrame', 14, 'TDesign 文档（外链', 
'https://tdesign.tencent.com/vue-next/getting-started', 1);

--
-- 权限管理
--
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `redirect`, `title`, `icon`) 
VALUES (18, '/permission', 'permission', 'LAYOUT', '/permission/roles', '权限管理', 'user-blocked');

-- 菜单管理
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (19, 'menus', 'PermissionMenus', '/permission/menus/index', 18, '菜单管理');

-- 角色管理
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (20, 'roles', 'PermissionRoles', '/permission/roles/index', 18, '角色管理');

-- 部门管理
INSERT INTO `sys_menu` (`id`, `path`, `name`, `component`, `pid`, `title`) 
VALUES (21, 'depts', 'PermissionDepts', '/permission/depts/index', 18, '部门管理');
